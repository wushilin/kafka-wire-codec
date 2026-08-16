//! Decode-side cursor over a chain of `Bytes` chunks.
//!
//! The shell decode path reads large frames as pool-sized chunks instead of
//! one contiguous buffer. [`ChunkChain`] walks that chain: small fields are
//! decoded across chunk boundaries with at most a 16-byte stitch copy, small
//! variable-length fields (strings, tagged data) are zero-copy when they fall
//! inside one chunk, and records payloads come out as [`RecordsChunks`] —
//! zero-copy slices of the read chunks, never made contiguous.

use crate::error::DecodeError;
use crate::types::{RecordsChunks, StrBytes};
use bytes::{Buf, Bytes};
use std::collections::VecDeque;
use uuid::Uuid;

/// A frame body as an ordered chain of `Bytes` chunks, consumed front to back.
pub struct ChunkChain {
    chunks: VecDeque<Bytes>,
    remaining: usize,
}

impl ChunkChain {
    pub fn new(chunks: impl IntoIterator<Item = Bytes>) -> Self {
        let chunks: VecDeque<Bytes> = chunks.into_iter().filter(|c| !c.is_empty()).collect();
        let remaining = chunks.iter().map(|c| c.len()).sum();
        ChunkChain { chunks, remaining }
    }

    /// Unconsumed bytes across all chunks.
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    pub fn is_empty(&self) -> bool {
        self.remaining == 0
    }

    fn eof(&self, needed: usize) -> DecodeError {
        DecodeError::UnexpectedEof {
            needed,
            available: self.remaining,
        }
    }

    /// Copy exactly `out.len()` bytes into `out`, advancing the chain.
    fn fill(&mut self, out: &mut [u8]) -> Result<(), DecodeError> {
        if self.remaining < out.len() {
            return Err(self.eof(out.len()));
        }
        let mut off = 0;
        while off < out.len() {
            let front = self.chunks.front_mut().expect("remaining covers out");
            let take = front.len().min(out.len() - off);
            out[off..off + take].copy_from_slice(&front[..take]);
            front.advance(take);
            if front.is_empty() {
                self.chunks.pop_front();
            }
            off += take;
            self.remaining -= take;
        }
        Ok(())
    }

    /// Take `n` bytes as one contiguous `Bytes`: zero-copy when the front
    /// chunk covers them, otherwise one small copy (used for strings and
    /// tagged-field data — bounded small on real frames).
    pub fn take_contiguous(&mut self, n: usize) -> Result<Bytes, DecodeError> {
        if self.remaining < n {
            return Err(self.eof(n));
        }
        if n == 0 {
            return Ok(Bytes::new());
        }
        let front = self.chunks.front_mut().expect("remaining >= n > 0");
        if front.len() >= n {
            let b = front.split_to(n);
            if front.is_empty() {
                self.chunks.pop_front();
            }
            self.remaining -= n;
            return Ok(b);
        }
        let mut v = vec![0u8; n];
        self.fill(&mut v)?;
        Ok(Bytes::from(v))
    }

    /// Take `n` bytes as zero-copy chunk slices — the records fast path.
    pub fn take_chunks(&mut self, n: usize) -> Result<RecordsChunks, DecodeError> {
        if self.remaining < n {
            return Err(self.eof(n));
        }
        let mut out = RecordsChunks::new();
        let mut left = n;
        while left > 0 {
            let front = self.chunks.front_mut().expect("remaining covers left");
            let take = front.len().min(left);
            out.push(front.split_to(take));
            if front.is_empty() {
                self.chunks.pop_front();
            }
            left -= take;
            self.remaining -= take;
        }
        Ok(out)
    }
}

// ── Fixed-width readers ───────────────────────────────────────────────────────

macro_rules! ch_fixed {
    ($name:ident, $t:ty, $n:literal) => {
        pub fn $name(ch: &mut ChunkChain) -> Result<$t, DecodeError> {
            let mut b = [0u8; $n];
            ch.fill(&mut b)?;
            Ok(<$t>::from_be_bytes(b))
        }
    };
}

ch_fixed!(ch_get_i16, i16, 2);
ch_fixed!(ch_get_u16, u16, 2);
ch_fixed!(ch_get_i32, i32, 4);
ch_fixed!(ch_get_u32, u32, 4);
ch_fixed!(ch_get_i64, i64, 8);
ch_fixed!(ch_get_f64, f64, 8);

pub fn ch_get_i8(ch: &mut ChunkChain) -> Result<i8, DecodeError> {
    let mut b = [0u8; 1];
    ch.fill(&mut b)?;
    Ok(b[0] as i8)
}

pub fn ch_get_bool(ch: &mut ChunkChain) -> Result<bool, DecodeError> {
    Ok(ch_get_i8(ch)? != 0)
}

pub fn ch_get_uuid(ch: &mut ChunkChain) -> Result<Uuid, DecodeError> {
    let mut b = [0u8; 16];
    ch.fill(&mut b)?;
    Ok(Uuid::from_bytes(b))
}

// ── Varints (same strictness as the contiguous readers) ──────────────────────

pub fn ch_get_uvarint(ch: &mut ChunkChain) -> Result<u64, DecodeError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        let mut b = [0u8; 1];
        ch.fill(&mut b)?;
        let b = b[0];
        if shift == 63 && (b & 0x7f) > 1 {
            return Err(DecodeError::InvalidVarint);
        }
        result |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift > 63 {
            return Err(DecodeError::InvalidVarint);
        }
    }
}

pub fn ch_get_uvarint32(ch: &mut ChunkChain) -> Result<u32, DecodeError> {
    let v = ch_get_uvarint(ch)?;
    u32::try_from(v).map_err(|_| DecodeError::InvalidVarint)
}

// ── Strings / bytes / records ─────────────────────────────────────────────────

pub fn ch_get_string(ch: &mut ChunkChain) -> Result<Option<StrBytes>, DecodeError> {
    let len = ch_get_i16(ch)?;
    if len < 0 {
        return Ok(None);
    }
    let raw = ch.take_contiguous(len as usize)?;
    Ok(Some(
        StrBytes::from_utf8(raw).map_err(|_| DecodeError::InvalidUtf8)?,
    ))
}

pub fn ch_get_compact_string(ch: &mut ChunkChain) -> Result<Option<StrBytes>, DecodeError> {
    match ch_get_compact_bytes(ch)? {
        None => Ok(None),
        Some(raw) => Ok(Some(
            StrBytes::from_utf8(raw).map_err(|_| DecodeError::InvalidUtf8)?,
        )),
    }
}

pub fn ch_get_bytes(ch: &mut ChunkChain) -> Result<Option<Bytes>, DecodeError> {
    let len = ch_get_i32(ch)?;
    if len < 0 {
        return Ok(None);
    }
    Ok(Some(ch.take_contiguous(len as usize)?))
}

pub fn ch_get_compact_bytes(ch: &mut ChunkChain) -> Result<Option<Bytes>, DecodeError> {
    let len = ch_get_uvarint32(ch)?;
    if len == 0 {
        return Ok(None);
    }
    Ok(Some(ch.take_contiguous((len - 1) as usize)?))
}

/// Legacy records: like bytes, but the payload stays chunked (zero-copy).
pub fn ch_get_records(ch: &mut ChunkChain) -> Result<Option<RecordsChunks>, DecodeError> {
    let len = ch_get_i32(ch)?;
    if len < 0 {
        return Ok(None);
    }
    Ok(Some(ch.take_chunks(len as usize)?))
}

/// Compact records: like compact bytes, but the payload stays chunked.
pub fn ch_get_compact_records(ch: &mut ChunkChain) -> Result<Option<RecordsChunks>, DecodeError> {
    let len = ch_get_uvarint32(ch)?;
    if len == 0 {
        return Ok(None);
    }
    Ok(Some(ch.take_chunks((len - 1) as usize)?))
}

// ── Tagged fields ─────────────────────────────────────────────────────────────

/// Read one tagged field from the chain. Tagged data is small on real frames
/// (records never live in tagged sections), so it is surfaced contiguously and
/// decoded with the ordinary buffer-based code.
pub fn ch_get_tagged_field(ch: &mut ChunkChain) -> Result<(u32, Bytes), DecodeError> {
    let tag = ch_get_uvarint32(ch)?;
    let size = ch_get_uvarint32(ch)? as usize;
    Ok((tag, ch.take_contiguous(size)?))
}

/// Read a whole tagged-fields section, preserving all tags raw.
pub fn ch_get_tagged_fields(ch: &mut ChunkChain) -> Result<Vec<(u32, Bytes)>, DecodeError> {
    let count = ch_get_uvarint32(ch)? as usize;
    let mut fields = Vec::with_capacity(count.min(ch.remaining() / 2));
    for _ in 0..count {
        fields.push(ch_get_tagged_field(ch)?);
    }
    Ok(fields)
}
