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
    contiguous_max: usize,
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
    /// Chunks released to the allocator (watermark rejections + trims).
    freed: AtomicUsize,
    /// Buffers returned via `abort` (failed or cancelled reads).
    aborted: AtomicUsize,
    /// Standby-lock acquisitions that had to wait (contended).
    lock_contended: AtomicUsize,
    /// Cumulative nanoseconds spent waiting on the standby lock. Only
    /// measured when contention actually occurs (try_lock-first), so the
    /// uncontended hot path never reads a clock.
    lock_wait_nanos: std::sync::atomic::AtomicU64,
}

/// A point-in-time snapshot of a [`PooledSupplier`]'s counters.
///
/// All counters are always on: they are relaxed atomics (~1 ns), and lock
/// timing is measured only when contention actually occurs, so there is no
/// profiling switch to flip and nothing to pay when idle.
#[non_exhaustive]
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
    /// Chunks released to the allocator (watermark rejections + `trim`).
    /// Invariant: `created == in_flight + standby + freed`.
    pub freed: usize,
    /// Buffers returned via `abort` — failed or cancelled reads.
    pub aborted: usize,
    /// Standby-lock acquisitions that had to wait behind another thread.
    pub lock_contended: usize,
    /// Total nanoseconds spent waiting on the standby lock (contended
    /// acquisitions only — uncontended ones never read a clock).
    pub lock_wait_nanos: u64,
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
                contiguous_max: chunk_size,
                max_standby,
                standby: Mutex::new(Vec::new()),
                in_flight: AtomicUsize::new(0),
                created: AtomicUsize::new(0),
                reused: AtomicUsize::new(0),
                high_watermark: AtomicUsize::new(0),
                freed: AtomicUsize::new(0),
                aborted: AtomicUsize::new(0),
                lock_contended: AtomicUsize::new(0),
                lock_wait_nanos: std::sync::atomic::AtomicU64::new(0),
            }),
        }
    }

    /// Lower the contiguous threshold below `chunk_size` (frames above it are
    /// chunked; frames above `chunk_size` always are). Construction-time only,
    /// before the supplier is cloned/shared.
    pub fn contiguous_max(mut self, n: usize) -> Self {
        let inner = Arc::get_mut(&mut self.inner)
            .expect("contiguous_max must be set before the supplier is cloned");
        assert!(
            n <= inner.chunk_size,
            "contiguous_max cannot exceed chunk_size (one pooled block)"
        );
        inner.contiguous_max = n;
        self
    }

    pub fn stats(&self) -> PoolStats {
        PoolStats {
            in_flight: self.inner.in_flight.load(Ordering::Relaxed),
            standby: self.inner.standby_lock().len(),
            created: self.inner.created.load(Ordering::Relaxed),
            reused: self.inner.reused.load(Ordering::Relaxed),
            high_watermark: self.inner.high_watermark.load(Ordering::Relaxed),
            freed: self.inner.freed.load(Ordering::Relaxed),
            aborted: self.inner.aborted.load(Ordering::Relaxed),
            lock_contended: self.inner.lock_contended.load(Ordering::Relaxed),
            lock_wait_nanos: self.inner.lock_wait_nanos.load(Ordering::Relaxed),
        }
    }

    /// Free all standby chunks now (e.g. after a burst).
    pub fn trim(&self) {
        let mut standby = self.inner.standby_lock();
        let n = standby.len();
        standby.clear();
        drop(standby);
        self.inner.freed.fetch_add(n, Ordering::Relaxed);
    }
}

impl BufferSupplier for PooledSupplier {
    fn strategy(&self, frame_len: usize) -> ReadStrategy {
        if frame_len <= self.inner.contiguous_max {
            // Fits the contiguous threshold: fast path, still pooled.
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
        let reused = self.inner.standby_lock().pop();
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
        // A failed or cancelled read: undo the acquire and restock the chunk
        // so flaky peers cost neither reuse nor counter accuracy.
        self.inner.aborted.fetch_add(1, Ordering::Relaxed);
        self.inner.restock(buf);
    }
}

impl PoolInner {
    /// Standby lock with contention accounting: `try_lock` first, so the
    /// uncontended path costs exactly a `lock()` and never reads a clock;
    /// only genuine waits are counted and timed.
    fn standby_lock(&self) -> std::sync::MutexGuard<'_, Vec<BytesMut>> {
        match self.standby.try_lock() {
            Ok(g) => g,
            Err(std::sync::TryLockError::WouldBlock) => {
                let start = std::time::Instant::now();
                let g = self.standby.lock().unwrap();
                self.lock_contended.fetch_add(1, Ordering::Relaxed);
                self.lock_wait_nanos
                    .fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
                g
            }
            Err(std::sync::TryLockError::Poisoned(p)) => p.into_inner(),
        }
    }

    fn restock(&self, mut buf: BytesMut) {
        self.in_flight.fetch_sub(1, Ordering::Relaxed);
        buf.clear();
        let standby_len = {
            let mut standby = self.standby_lock();
            if standby.len() < self.max_standby {
                standby.push(buf);
                return;
            }
            standby.len()
        };
        // Watermark reached: the chunk frees at end of scope, OUTSIDE the lock.
        let _ = standby_len;
        self.freed.fetch_add(1, Ordering::Relaxed);
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
