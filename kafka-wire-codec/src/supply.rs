//! Caller-pluggable buffer policy for frame reads.
//!
//! The frame's 4-byte length prefix tells the exact body size before any body
//! byte is read, so buffering policy can be decided per frame, up front. A
//! [`BufferSupplier`] owns that policy: it picks the read path (contiguous
//! fast path vs. chunked shell path) and provides the buffers — from plain
//! allocation, a pool, a spool, wherever. The codec never knows which.
//!
//! When no supplier is given, [`DefaultSupplier`] applies a size threshold
//! with plain allocation — zero configuration, and chunk-sized allocations sit
//! below allocator mmap thresholds so even the default avoids large-allocation
//! churn.

use bytes::{Bytes, BytesMut};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// How to read one frame body, chosen per frame from its exact length.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadStrategy {
    /// Read the whole body into one contiguous buffer (the fast path for
    /// small/medium frames; decode with the ordinary `decode`).
    Contiguous,
    /// Read the body as pool-sized chunks (the shell path for payload-heavy
    /// frames; decode with `decode_chained` on the message's `*Shell` type).
    Chunked { chunk_size: usize },
}

/// Buffer policy + provider for frame reads.
///
/// `strategy` is the policy point: it sees the exact frame length (from the
/// wire's length prefix) before any body byte is read — this is where a
/// size threshold, admission control, or budgeting lives. `acquire` is the
/// mechanism point: malloc, borrow from a pool, NUMA-pin — the codec only
/// requires a writable buffer with at least `len` capacity.
///
/// The return path is [`BufferSupplier::seal`]: after filling a buffer the
/// codec asks the supplier to convert it into the frozen `Bytes` it will
/// slice. The default just freezes (drop = free); a pooling supplier wraps
/// the buffer in an owner whose `Drop` returns it to the pool — every slice
/// of the chunk shares that owner's refcount, so the chunk comes back
/// automatically when the last reference (typically on the outbound-write
/// side) is dropped, on whatever thread that happens.
pub trait BufferSupplier: Send + Sync {
    /// Choose the read path for a frame of exactly `frame_len` body bytes.
    fn strategy(&self, frame_len: usize) -> ReadStrategy;

    /// Provide a buffer with at least `len` bytes of capacity.
    fn acquire(&self, len: usize) -> BytesMut;

    /// Convert a filled buffer into the `Bytes` the codec will slice. This is
    /// the pool's return path: override to attach a drop-time reclaim (see
    /// [`PooledSupplier`]). The default freezes with no reclaim.
    ///
    /// Pairing contract: for every buffer obtained from [`Self::acquire`],
    /// the codec calls exactly one of `seal` (read succeeded) or
    /// [`Self::abort`] (read failed **or the read future was cancelled** —
    /// the codec holds acquired buffers in a drop guard, so async
    /// cancellation at a suspension point still runs `abort`). Stateful
    /// suppliers can rely on it.
    fn seal(&self, buf: BytesMut) -> Bytes {
        buf.freeze()
    }

    /// Give back a buffer whose read FAILED before it could be sealed (e.g. a
    /// mid-body EOF or I/O error). The default drops it; a pooling supplier
    /// should restock it so a flaky peer doesn't bleed reuse or skew counters.
    fn abort(&self, buf: BytesMut) {
        drop(buf);
    }
}

/// The built-in policy: contiguous up to a threshold, chunked above it, plain
/// allocation for both.
#[derive(Debug, Clone)]
pub struct DefaultSupplier {
    /// Largest frame read contiguously (default 1 MiB).
    pub contiguous_max: usize,
    /// Chunk size for larger frames (default 1 MiB).
    pub chunk_size: usize,
}

impl Default for DefaultSupplier {
    fn default() -> Self {
        DefaultSupplier {
            contiguous_max: 1 << 20,
            chunk_size: 1 << 20,
        }
    }
}

impl BufferSupplier for DefaultSupplier {
    fn strategy(&self, frame_len: usize) -> ReadStrategy {
        if frame_len <= self.contiguous_max {
            ReadStrategy::Contiguous
        } else {
            ReadStrategy::Chunked {
                chunk_size: self.chunk_size,
            }
        }
    }

    fn acquire(&self, len: usize) -> BytesMut {
        BytesMut::with_capacity(len)
    }
}

// ── Built-in pooling supplier ─────────────────────────────────────────────────

/// A batteries-included chunk pool: uniform blocks of `chunk_size`, reused
/// across frames, returned automatically on drop. Construct once, share
/// (it's `Clone` — clones share the pool), pass to `read_frame_supplied`.
///
/// Policy: a frame that fits in ONE chunk is read contiguously into a single
/// pooled block (small frames stay on the fast path with zero extra cost);
/// anything larger is chunked. Every buffer this supplier issues has
/// `chunk_size` capacity and is reclaimed when the last `Bytes` reference to
/// it drops — cargo slices, frame segments, decoded field slices included.
/// Standby chunks beyond `max_standby` are freed instead of retained, which
/// is the pool's trim watermark.
#[derive(Clone)]
pub struct PooledSupplier {
    inner: Arc<PoolInner>,
}

struct PoolInner {
    chunk_size: usize,
    max_standby: usize,
    standby: Mutex<Vec<BytesMut>>,
    /// Chunks currently out (issued, not yet dropped back).
    in_flight: AtomicUsize,
    /// Fresh allocations ever made (misses).
    created: AtomicUsize,
    /// Standby hits (reuse).
    reused: AtomicUsize,
    /// Max in_flight ever observed.
    high_watermark: AtomicUsize,
}

/// A point-in-time snapshot of a [`PooledSupplier`]'s counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStats {
    /// Chunks currently issued and not yet returned.
    pub in_flight: usize,
    /// Chunks sitting in the pool ready for reuse.
    pub standby: usize,
    /// Fresh allocations made so far (pool misses).
    pub created: usize,
    /// Reuses served from standby (pool hits).
    pub reused: usize,
    /// Highest `in_flight` ever observed.
    pub high_watermark: usize,
}

impl PooledSupplier {
    /// `chunk_size`: the uniform block size (e.g. 1 MiB). `max_standby`: how
    /// many idle chunks the pool retains before freeing returns (the memory
    /// watermark is roughly `(in_flight + max_standby) * chunk_size`).
    pub fn new(chunk_size: usize, max_standby: usize) -> Self {
        assert!(chunk_size > 0, "chunk_size must be non-zero");
        PooledSupplier {
            inner: Arc::new(PoolInner {
                chunk_size,
                max_standby,
                standby: Mutex::new(Vec::new()),
                in_flight: AtomicUsize::new(0),
                created: AtomicUsize::new(0),
                reused: AtomicUsize::new(0),
                high_watermark: AtomicUsize::new(0),
            }),
        }
    }

    pub fn stats(&self) -> PoolStats {
        PoolStats {
            in_flight: self.inner.in_flight.load(Ordering::Relaxed),
            standby: self.inner.standby.lock().unwrap().len(),
            created: self.inner.created.load(Ordering::Relaxed),
            reused: self.inner.reused.load(Ordering::Relaxed),
            high_watermark: self.inner.high_watermark.load(Ordering::Relaxed),
        }
    }

    /// Free all standby chunks now (e.g. after a burst).
    pub fn trim(&self) {
        self.inner.standby.lock().unwrap().clear();
    }
}

impl BufferSupplier for PooledSupplier {
    fn strategy(&self, frame_len: usize) -> ReadStrategy {
        if frame_len <= self.inner.chunk_size {
            // Fits in one pooled block: contiguous fast path, still pooled.
            ReadStrategy::Contiguous
        } else {
            ReadStrategy::Chunked {
                chunk_size: self.inner.chunk_size,
            }
        }
    }

    fn acquire(&self, len: usize) -> BytesMut {
        debug_assert!(
            len <= self.inner.chunk_size,
            "PooledSupplier never asks for more than one chunk"
        );
        let reused = self.inner.standby.lock().unwrap().pop();
        let buf = match reused {
            Some(b) => {
                self.inner.reused.fetch_add(1, Ordering::Relaxed);
                b
            }
            None => {
                self.inner.created.fetch_add(1, Ordering::Relaxed);
                BytesMut::with_capacity(self.inner.chunk_size)
            }
        };
        let now = self.inner.in_flight.fetch_add(1, Ordering::Relaxed) + 1;
        self.inner.high_watermark.fetch_max(now, Ordering::Relaxed);
        buf
    }

    fn seal(&self, buf: BytesMut) -> Bytes {
        // Wrap the filled chunk in a drop-returning owner: every slice the
        // codec takes shares this owner's refcount, so the chunk returns to
        // the pool when the LAST reference is dropped.
        Bytes::from_owner(PooledChunk {
            buf: Some(buf),
            pool: Arc::clone(&self.inner),
        })
    }

    fn abort(&self, buf: BytesMut) {
        // A failed read: undo the acquire and restock the chunk so flaky
        // peers cost neither reuse nor counter accuracy.
        self.inner.restock(buf);
    }
}

impl PoolInner {
    fn restock(&self, mut buf: BytesMut) {
        self.in_flight.fetch_sub(1, Ordering::Relaxed);
        buf.clear();
        let mut standby = self.standby.lock().unwrap();
        if standby.len() < self.max_standby {
            standby.push(buf);
        }
        // else: drop -> free (watermark trim).
    }
}

/// Owner of a pooled chunk while it circulates as `Bytes`.
struct PooledChunk {
    buf: Option<BytesMut>,
    pool: Arc<PoolInner>,
}

impl AsRef<[u8]> for PooledChunk {
    fn as_ref(&self) -> &[u8] {
        self.buf.as_deref().unwrap_or(&[])
    }
}

impl Drop for PooledChunk {
    fn drop(&mut self) {
        if let Some(b) = self.buf.take() {
            self.pool.restock(b);
        } else {
            self.pool.in_flight.fetch_sub(1, Ordering::Relaxed);
        }
    }
}
