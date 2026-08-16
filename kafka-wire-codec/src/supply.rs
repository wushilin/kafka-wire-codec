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

use bytes::BytesMut;

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
/// Reclaim is deliberately not part of the trait: buffers flow into
/// refcounted `Bytes` and drop on whatever thread finishes writing them out.
/// A pooling supplier should retain handles to the chunks it issues and
/// re-issue any standby chunk that is unique again (`Bytes`-refcount observed
/// during `acquire`), which makes return-to-pool a passive observation with no
/// drop hooks.
pub trait BufferSupplier: Send + Sync {
    /// Choose the read path for a frame of exactly `frame_len` body bytes.
    fn strategy(&self, frame_len: usize) -> ReadStrategy;

    /// Provide a buffer with at least `len` bytes of capacity.
    fn acquire(&self, len: usize) -> BytesMut;
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
