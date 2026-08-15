use crate::error::DecodeError;
use crate::types::StrBytes;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use uuid::Uuid;

// ── Encode sinks ──────────────────────────────────────────────────────────────

/// Payloads at or above this many bytes are kept as shared segments (zero-copy)
/// by [`SegmentedBuf`]; smaller ones are copied into the scratch buffer to avoid
/// fragmenting the output into tiny segments.
pub const DEFAULT_ZERO_COPY_THRESHOLD: usize = 1024;

/// Sink for the generated encoders. Implemented by [`BytesMut`] (contiguous,
/// always copies) and [`SegmentedBuf`] (zero-copy: large [`Bytes`] payloads are
/// retained by refcounted handle instead of being memcpy'd).
pub trait WireBuf {
    fn put_u8(&mut self, v: u8);
    fn put_slice(&mut self, s: &[u8]);
    /// Append a refcounted payload. Implementations may retain the handle
    /// (zero-copy) instead of copying the bytes.
    fn put_shared(&mut self, b: &Bytes);
}

impl WireBuf for BytesMut {
    #[inline]
    fn put_u8(&mut self, v: u8) {
        BufMut::put_u8(self, v);
    }
    #[inline]
    fn put_slice(&mut self, s: &[u8]) {
        BufMut::put_slice(self, s);
    }
    #[inline]
    fn put_shared(&mut self, b: &Bytes) {
        BufMut::put_slice(self, b);
    }
}

/// Zero-copy encode sink: an ordered list of [`Bytes`] segments.
///
/// Scalar fields and small payloads accumulate in a scratch buffer; a `Bytes`
/// payload of at least `threshold` bytes flushes the scratch as a segment and
/// is appended as a refcounted handle — the payload itself is never copied.
/// This is the proxy fast path: decoded `records`/`bytes` fields (themselves
/// zero-copy slices of the inbound frame) flow to the outbound frame untouched.
pub struct SegmentedBuf {
    scratch: BytesMut,
    segments: Vec<Bytes>,
    threshold: usize,
}

impl SegmentedBuf {
    pub fn new() -> Self {
        Self::with_threshold(DEFAULT_ZERO_COPY_THRESHOLD)
    }

    /// `threshold`: minimum payload size kept as a shared segment. `0` shares
    /// every payload regardless of size.
    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            scratch: BytesMut::new(),
            segments: Vec::new(),
            threshold,
        }
    }

    /// Total encoded length so far, across all segments and the scratch tail.
    pub fn len(&self) -> usize {
        self.segments.iter().map(|s| s.len()).sum::<usize>() + self.scratch.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.scratch.is_empty()
    }

    fn flush_scratch(&mut self) {
        if !self.scratch.is_empty() {
            self.segments.push(self.scratch.split().freeze());
        }
    }

    /// Finish and return the ordered segments.
    pub fn into_segments(mut self) -> Vec<Bytes> {
        self.flush_scratch();
        self.segments
    }
}

impl Default for SegmentedBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl WireBuf for SegmentedBuf {
    #[inline]
    fn put_u8(&mut self, v: u8) {
        BufMut::put_u8(&mut self.scratch, v);
    }
    #[inline]
    fn put_slice(&mut self, s: &[u8]) {
        BufMut::put_slice(&mut self.scratch, s);
    }
    fn put_shared(&mut self, b: &Bytes) {
        if b.len() >= self.threshold {
            self.flush_scratch();
            self.segments.push(b.clone());
        } else {
            BufMut::put_slice(&mut self.scratch, b);
        }
    }
}

// ── Decode helpers ────────────────────────────────────────────────────────────

pub fn get_i8(buf: &mut Bytes) -> Result<i8, DecodeError> {
    ensure(buf, 1)?;
    Ok(buf.get_i8())
}

pub fn get_i16(buf: &mut Bytes) -> Result<i16, DecodeError> {
    ensure(buf, 2)?;
    Ok(buf.get_i16())
}

pub fn get_u16(buf: &mut Bytes) -> Result<u16, DecodeError> {
    ensure(buf, 2)?;
    Ok(buf.get_u16())
}

pub fn get_i32(buf: &mut Bytes) -> Result<i32, DecodeError> {
    ensure(buf, 4)?;
    Ok(buf.get_i32())
}

pub fn get_u32(buf: &mut Bytes) -> Result<u32, DecodeError> {
    ensure(buf, 4)?;
    Ok(buf.get_u32())
}

pub fn get_i64(buf: &mut Bytes) -> Result<i64, DecodeError> {
    ensure(buf, 8)?;
    Ok(buf.get_i64())
}

pub fn get_f64(buf: &mut Bytes) -> Result<f64, DecodeError> {
    ensure(buf, 8)?;
    Ok(buf.get_f64())
}

pub fn get_bool(buf: &mut Bytes) -> Result<bool, DecodeError> {
    Ok(get_i8(buf)? != 0)
}

pub fn get_uuid(buf: &mut Bytes) -> Result<Uuid, DecodeError> {
    ensure(buf, 16)?;
    let mut uuid = [0u8; 16];
    buf.copy_to_slice(&mut uuid);
    Ok(Uuid::from_bytes(uuid))
}

/// Unsigned varint (used for compact array/string lengths and tagged fields).
///
/// Strict per Kafka's `readUnsignedVarlong`: at most 10 bytes, and the 10th
/// byte may only contribute the single remaining value bit — a final byte that
/// would shift value bits past 63 is rejected instead of silently truncated.
pub fn get_uvarint(buf: &mut Bytes) -> Result<u64, DecodeError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        ensure(buf, 1)?;
        let b = buf.get_u8();
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

/// Unsigned varint bounded to u32 (Kafka's varint32: compact lengths, tags).
/// Values above `u32::MAX` are a protocol violation and are rejected rather
/// than truncated.
pub fn get_uvarint32(buf: &mut Bytes) -> Result<u32, DecodeError> {
    let v = get_uvarint(buf)?;
    u32::try_from(v).map_err(|_| DecodeError::InvalidVarint)
}

/// Signed varint (zigzag encoded). Used by record batch internals.
pub fn get_varint(buf: &mut Bytes) -> Result<i64, DecodeError> {
    let n = get_uvarint(buf)?;
    Ok(((n >> 1) as i64) ^ (-((n & 1) as i64)))
}

/// Legacy string: int16 length (-1 = null), then UTF-8 bytes.
///
/// UTF-8 is validated here, once — the returned [`StrBytes`] is still a
/// zero-copy slice of `buf`.
pub fn get_string(buf: &mut Bytes) -> Result<Option<StrBytes>, DecodeError> {
    let len = get_i16(buf)?;
    if len < 0 {
        return Ok(None);
    }
    let len = len as usize;
    ensure(buf, len)?;
    let raw = buf.split_to(len);
    Ok(Some(
        StrBytes::from_utf8(raw).map_err(|_| DecodeError::InvalidUtf8)?,
    ))
}

/// Compact string: uvarint length (0 = null, N+1 = N bytes). UTF-8 validated.
pub fn get_compact_string(buf: &mut Bytes) -> Result<Option<StrBytes>, DecodeError> {
    match get_compact_bytes(buf)? {
        None => Ok(None),
        Some(raw) => Ok(Some(
            StrBytes::from_utf8(raw).map_err(|_| DecodeError::InvalidUtf8)?,
        )),
    }
}

/// Legacy bytes: int32 length (-1 = null), then raw bytes.
pub fn get_bytes(buf: &mut Bytes) -> Result<Option<Bytes>, DecodeError> {
    let len = get_i32(buf)?;
    if len < 0 {
        return Ok(None);
    }
    let len = len as usize;
    ensure(buf, len)?;
    Ok(Some(buf.split_to(len)))
}

/// Compact bytes: uvarint length (0 = null, N+1 = N bytes).
pub fn get_compact_bytes(buf: &mut Bytes) -> Result<Option<Bytes>, DecodeError> {
    let len = get_uvarint32(buf)?;
    if len == 0 {
        return Ok(None);
    }
    let len = (len - 1) as usize;
    ensure(buf, len)?;
    Ok(Some(buf.split_to(len)))
}

/// Read one tagged field: uvarint tag, plain uvarint byte length (NOT compact
/// +1), then the raw data as a zero-copy slice.
pub fn get_tagged_field(buf: &mut Bytes) -> Result<(u32, Bytes), DecodeError> {
    let tag = get_uvarint32(buf)?;
    let size = get_uvarint32(buf)? as usize;
    ensure(buf, size)?;
    Ok((tag, buf.split_to(size)))
}

/// Read the tagged-fields section; preserves unknown tags as raw Bytes.
pub fn get_tagged_fields(buf: &mut Bytes) -> Result<Vec<(u32, Bytes)>, DecodeError> {
    let count = get_uvarint32(buf)? as usize;
    // Capacity is clamped by what the buffer could possibly hold (each field
    // needs at least 2 bytes: tag + size) so a malicious count cannot force a
    // huge upfront allocation.
    let mut fields = Vec::with_capacity(count.min(buf.len() / 2));
    for _ in 0..count {
        fields.push(get_tagged_field(buf)?);
    }
    Ok(fields)
}

// ── Size helpers ──────────────────────────────────────────────────────────────

pub fn uvarint_size(mut v: u64) -> usize {
    let mut size = 1usize;
    while v >= 0x80 {
        v >>= 7;
        size += 1;
    }
    size
}

/// Size of a zigzag varint. Used by record batch internals.
pub fn varint_size(v: i64) -> usize {
    uvarint_size(((v << 1) ^ (v >> 63)) as u64)
}

pub fn string_size(s: &str) -> usize {
    2 + s.len()
}

pub fn nullable_string_size(s: Option<&str>) -> usize {
    match s {
        None => 2,
        Some(b) => 2 + b.len(),
    }
}

pub fn compact_string_size(s: &str) -> usize {
    compact_bytes_size(s.as_bytes())
}

pub fn compact_nullable_string_size(s: Option<&str>) -> usize {
    compact_nullable_bytes_size(s.map(str::as_bytes))
}

pub fn bytes_size(b: &[u8]) -> usize {
    4 + b.len()
}

pub fn nullable_bytes_size(b: Option<&[u8]>) -> usize {
    match b {
        None => 4,
        Some(b) => 4 + b.len(),
    }
}

pub fn compact_bytes_size(b: &[u8]) -> usize {
    uvarint_size(b.len() as u64 + 1) + b.len()
}

pub fn compact_nullable_bytes_size(b: Option<&[u8]>) -> usize {
    match b {
        None => 1,
        Some(b) => uvarint_size(b.len() as u64 + 1) + b.len(),
    }
}

/// Size of the raw tagged fields WITHOUT the leading count varint — used by
/// generated code that interleaves schema-known (typed) tags with raw ones.
pub fn raw_tagged_fields_size(fields: &[(u32, Bytes)]) -> usize {
    fields
        .iter()
        .map(|(tag, data)| {
            // Plain uvarint length prefix + raw data (NOT compact bytes).
            uvarint_size(*tag as u64) + uvarint_size(data.len() as u64) + data.len()
        })
        .sum()
}

pub fn tagged_fields_size(fields: &[(u32, Bytes)]) -> usize {
    uvarint_size(fields.len() as u64) + raw_tagged_fields_size(fields)
}

// ── Encode helpers ────────────────────────────────────────────────────────────
//
// All writers are generic over [`WireBuf`], so the same generated encode code
// drives both the contiguous (`BytesMut`) and zero-copy (`SegmentedBuf`) paths.

pub fn put_i8<B: WireBuf>(buf: &mut B, v: i8) {
    buf.put_u8(v as u8);
}
pub fn put_i16<B: WireBuf>(buf: &mut B, v: i16) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_u16<B: WireBuf>(buf: &mut B, v: u16) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_i32<B: WireBuf>(buf: &mut B, v: i32) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_u32<B: WireBuf>(buf: &mut B, v: u32) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_i64<B: WireBuf>(buf: &mut B, v: i64) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_f64<B: WireBuf>(buf: &mut B, v: f64) {
    buf.put_slice(&v.to_be_bytes());
}
pub fn put_bool<B: WireBuf>(buf: &mut B, v: bool) {
    buf.put_u8(if v { 1 } else { 0 });
}
pub fn put_uuid<B: WireBuf>(buf: &mut B, v: &Uuid) {
    buf.put_slice(v.as_bytes());
}

pub fn put_uvarint<B: WireBuf>(buf: &mut B, mut v: u64) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            buf.put_u8(b);
            break;
        } else {
            buf.put_u8(b | 0x80);
        }
    }
}

/// Zigzag varint. Used by record batch internals.
pub fn put_varint<B: WireBuf>(buf: &mut B, v: i64) {
    put_uvarint(buf, ((v << 1) ^ (v >> 63)) as u64);
}

pub fn put_string<B: WireBuf>(buf: &mut B, s: &str) {
    // A silent `as i16` wrap would emit a corrupt frame; oversized strings are
    // a caller bug (valid decoded frames can never contain one).
    assert!(
        s.len() <= i16::MAX as usize,
        "string field of {} bytes exceeds the legacy wire limit of {}",
        s.len(),
        i16::MAX
    );
    put_i16(buf, s.len() as i16);
    buf.put_slice(s.as_bytes());
}

pub fn put_nullable_string<B: WireBuf>(buf: &mut B, s: Option<&str>) {
    match s {
        None => put_i16(buf, -1),
        Some(b) => put_string(buf, b),
    }
}

pub fn put_compact_string<B: WireBuf>(buf: &mut B, s: &str) {
    put_compact_bytes(buf, s.as_bytes());
}

pub fn put_compact_nullable_string<B: WireBuf>(buf: &mut B, s: Option<&str>) {
    put_compact_nullable_bytes(buf, s.map(str::as_bytes));
}

pub fn put_bytes<B: WireBuf>(buf: &mut B, b: &[u8]) {
    assert!(
        b.len() <= i32::MAX as usize,
        "bytes field of {} bytes exceeds the wire limit of {}",
        b.len(),
        i32::MAX
    );
    put_i32(buf, b.len() as i32);
    buf.put_slice(b);
}

pub fn put_nullable_bytes<B: WireBuf>(buf: &mut B, b: Option<&[u8]>) {
    match b {
        None => put_i32(buf, -1),
        Some(b) => put_bytes(buf, b),
    }
}

pub fn put_compact_bytes<B: WireBuf>(buf: &mut B, b: &[u8]) {
    put_uvarint(buf, b.len() as u64 + 1);
    buf.put_slice(b);
}

pub fn put_compact_nullable_bytes<B: WireBuf>(buf: &mut B, b: Option<&[u8]>) {
    match b {
        None => put_uvarint(buf, 0),
        Some(b) => put_compact_bytes(buf, b),
    }
}

// Zero-copy variants: take the `Bytes` handle so a `SegmentedBuf` sink can
// retain large payloads by reference instead of copying them. The generated
// encoders use these for every `bytes`/`records` field.

pub fn put_bytes_zc<B: WireBuf>(buf: &mut B, b: &Bytes) {
    assert!(
        b.len() <= i32::MAX as usize,
        "bytes field of {} bytes exceeds the wire limit of {}",
        b.len(),
        i32::MAX
    );
    put_i32(buf, b.len() as i32);
    buf.put_shared(b);
}

pub fn put_nullable_bytes_zc<B: WireBuf>(buf: &mut B, b: Option<&Bytes>) {
    match b {
        None => put_i32(buf, -1),
        Some(b) => put_bytes_zc(buf, b),
    }
}

pub fn put_compact_bytes_zc<B: WireBuf>(buf: &mut B, b: &Bytes) {
    put_uvarint(buf, b.len() as u64 + 1);
    buf.put_shared(b);
}

pub fn put_compact_nullable_bytes_zc<B: WireBuf>(buf: &mut B, b: Option<&Bytes>) {
    match b {
        None => put_uvarint(buf, 0),
        Some(b) => put_compact_bytes_zc(buf, b),
    }
}

/// Write one raw tagged field (tag, plain uvarint length, data). Generated
/// code uses this to interleave raw tags with schema-known typed ones in
/// ascending tag order.
pub fn put_raw_tagged_field<B: WireBuf>(buf: &mut B, tag: u32, data: &Bytes) {
    put_uvarint(buf, tag as u64);
    // Plain uvarint length prefix + raw data (NOT compact bytes).
    put_uvarint(buf, data.len() as u64);
    buf.put_shared(data);
}

pub fn put_tagged_fields<B: WireBuf>(buf: &mut B, fields: &[(u32, Bytes)]) {
    // Kafka readers reject out-of-order or duplicate tags; catch the caller
    // bug here instead of emitting a frame the broker will refuse.
    debug_assert!(
        fields.windows(2).all(|w| w[0].0 < w[1].0),
        "tagged fields must have strictly ascending tags"
    );
    put_uvarint(buf, fields.len() as u64);
    for (tag, data) in fields {
        put_raw_tagged_field(buf, *tag, data);
    }
}

// ── Internal ──────────────────────────────────────────────────────────────────

fn ensure(buf: &Bytes, n: usize) -> Result<(), DecodeError> {
    if buf.len() < n {
        Err(DecodeError::UnexpectedEof {
            needed: n,
            available: buf.len(),
        })
    } else {
        Ok(())
    }
}
