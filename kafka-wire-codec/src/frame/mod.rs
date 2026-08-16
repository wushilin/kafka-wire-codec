use crate::codec::chain::ChunkChain;
use crate::codec::SegmentedBuf;
use crate::error::{DecodeError, EncodeError};
use crate::generated::kinds::{RequestKind, ResponseKind};
use crate::header::{RequestHeader, ResponseHeader};
use crate::message::{Encodable, EncodableZeroCopy};
use crate::supply::{BufferSupplier, ReadStrategy};
use bytes::{Bytes, BytesMut};
use std::io::{Read, Write};

/// Default cap on accepted frame bodies: 100 MiB-ish, matching Kafka's broker
/// default `socket.request.max.bytes` (104857600). The length prefix is
/// attacker-controlled, so an uncapped read is a memory-DoS vector.
pub const DEFAULT_MAX_FRAME_SIZE: usize = 104_857_600;

/// Build a complete length-prefixed request frame: `[len][header][body]`.
///
/// Size-first and single-allocation: the exact frame size is computed up front,
/// one buffer is allocated, the header and body are encoded into it, and the
/// 4-byte length is written without a second pass over the body. The returned
/// [`EncodedFrame`] can then be written to a sync or async stream.
pub fn frame_request<M: Encodable>(
    header: &RequestHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let size = header.encoded_size(header_version) + body.wire_size(api_version)?;
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.write(api_version, &mut buf)?;
    Ok(EncodedFrame::new(buf))
}

/// Build a complete length-prefixed response frame: `[len][header][body]`.
pub fn frame_response<M: Encodable>(
    header: &ResponseHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let size = header.encoded_size(header_version) + body.wire_size(api_version)?;
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.write(api_version, &mut buf)?;
    Ok(EncodedFrame::new(buf))
}

/// Zero-copy, single-pass variant of [`frame_request`]: large `Bytes` payloads
/// (e.g. produce record batches) become refcounted segments of the frame
/// instead of being memcpy'd. No sizing pass is performed — the frame length
/// is the sum of the segment lengths.
pub fn frame_request_zero_copy<M: EncodableZeroCopy>(
    header: &RequestHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.write_segmented(api_version, &mut buf)?;
    Ok(EncodedFrame::from_segments(buf))
}

/// Zero-copy, single-pass variant of [`frame_response`]: large `Bytes` payloads
/// (e.g. fetch record batches) become refcounted segments of the frame instead
/// of being memcpy'd.
pub fn frame_response_zero_copy<M: EncodableZeroCopy>(
    header: &ResponseHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.write_segmented(api_version, &mut buf)?;
    Ok(EncodedFrame::from_segments(buf))
}

/// [`frame_request`] for a [`RequestKind`]: size-first, single-allocation
/// framing straight off the typed dispatch enum.
pub fn frame_request_kind(
    header: &RequestHeader,
    header_version: i16,
    body: &RequestKind,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let size = header.encoded_size(header_version) + body.encoded_size(api_version)?;
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.encode(api_version, &mut buf)?;
    Ok(EncodedFrame::new(buf))
}

/// [`frame_response`] for a [`ResponseKind`].
pub fn frame_response_kind(
    header: &ResponseHeader,
    header_version: i16,
    body: &ResponseKind,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let size = header.encoded_size(header_version) + body.encoded_size(api_version)?;
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.encode(api_version, &mut buf)?;
    Ok(EncodedFrame::new(buf))
}

/// [`frame_request_zero_copy`] for a [`RequestKind`]: large `Bytes` payloads
/// become refcounted frame segments instead of being memcpy'd.
pub fn frame_request_kind_zero_copy(
    header: &RequestHeader,
    header_version: i16,
    body: &RequestKind,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.encode(api_version, &mut buf)?;
    Ok(EncodedFrame::from_segments(buf))
}

/// [`frame_response_zero_copy`] for a [`ResponseKind`].
pub fn frame_response_kind_zero_copy(
    header: &ResponseHeader,
    header_version: i16,
    body: &ResponseKind,
    api_version: i16,
) -> Result<EncodedFrame, EncodeError> {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.encode(api_version, &mut buf)?;
    Ok(EncodedFrame::from_segments(buf))
}

/// Validate an incoming frame length prefix against a cap.
fn checked_len(len: i32, max: usize) -> Result<usize, DecodeError> {
    if len < 0 {
        return Err(DecodeError::UnexpectedEof {
            needed: 0,
            available: 0,
        });
    }
    let len = len as usize;
    if len > max {
        return Err(DecodeError::FrameTooLarge { size: len, max });
    }
    Ok(len)
}

/// Read one length-prefixed Kafka frame from a sync reader, rejecting frames
/// larger than [`DEFAULT_MAX_FRAME_SIZE`]. Frame format: [int32 length][body].
pub fn read_frame<R: Read>(reader: &mut R) -> Result<Bytes, DecodeError> {
    read_frame_with_limit(reader, DEFAULT_MAX_FRAME_SIZE)
}

/// [`read_frame`] with a caller-chosen frame-size cap.
pub fn read_frame_with_limit<R: Read>(reader: &mut R, max: usize) -> Result<Bytes, DecodeError> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = checked_len(i32::from_be_bytes(len_buf), max)?;
    // Read via `take` + `read_to_end` so the buffer is never pre-zeroed.
    let mut body = Vec::with_capacity(len);
    let read = reader.take(len as u64).read_to_end(&mut body)?;
    if read < len {
        return Err(DecodeError::UnexpectedEof {
            needed: len,
            available: read,
        });
    }
    Ok(Bytes::from(body))
}

/// [`read_frame`] into a caller-owned buffer, enabling **buffer reuse across
/// frames**: the frame body is appended to `buf`'s tail and split off as a
/// zero-copy `Bytes` view.
///
/// Once every `Bytes` returned from a given `buf` has been dropped, the next
/// call reclaims the full allocation — steady-state reads allocate nothing.
/// `BytesMut` grows itself on demand, so an undersized buffer is never an
/// error; it just pays one (amortized) regrow. Callers tracking buffer stats
/// can inspect `buf.capacity()` between calls, and pre-size with
/// `BytesMut::with_capacity` / `reserve`.
///
/// Any bytes already in `buf` are left untouched (only the tail is consumed).
pub fn read_frame_into<R: Read>(reader: &mut R, buf: &mut BytesMut) -> Result<Bytes, DecodeError> {
    read_frame_into_with_limit(reader, buf, DEFAULT_MAX_FRAME_SIZE)
}

/// [`read_frame_into`] with a caller-chosen frame-size cap.
pub fn read_frame_into_with_limit<R: Read>(
    reader: &mut R,
    buf: &mut BytesMut,
    max: usize,
) -> Result<Bytes, DecodeError> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = checked_len(i32::from_be_bytes(len_buf), max)?;
    let start = buf.len();
    buf.resize(start + len, 0);
    let mut filled = 0usize;
    while filled < len {
        match reader.read(&mut buf[start + filled..]) {
            Ok(0) => {
                buf.truncate(start);
                return Err(DecodeError::UnexpectedEof {
                    needed: len,
                    available: filled,
                });
            }
            Ok(n) => filled += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => {
                buf.truncate(start);
                return Err(e.into());
            }
        }
    }
    Ok(take_frame(buf, start, len))
}

/// Detach the just-read frame (`buf[start..start+len]`) as a frozen view.
///
/// On the common empty-buffer path this is `split_to`, which leaves the spare
/// capacity with `buf` — that's what makes cross-frame reuse work. (`split_off`
/// would hand the spare capacity to the returned frame instead.)
fn take_frame(buf: &mut BytesMut, start: usize, len: usize) -> Bytes {
    if start == 0 {
        buf.split_to(len).freeze()
    } else {
        buf.split_off(start).freeze()
    }
}

/// A frame body read under a [`BufferSupplier`]'s strategy: either one
/// contiguous buffer (decode with `Message::decode`) or a chain of chunks
/// (decode with the message's `*Shell::decode_chained`).
pub enum SuppliedFrame {
    Contiguous(Bytes),
    Chunked(ChunkChain),
}

impl SuppliedFrame {
    /// Body length in bytes.
    pub fn len(&self) -> usize {
        match self {
            SuppliedFrame::Contiguous(b) => b.len(),
            SuppliedFrame::Chunked(c) => c.remaining(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Read one frame under a [`BufferSupplier`]'s policy: the supplier sees the
/// exact frame length (from the 4-byte prefix, before any body byte is read)
/// and picks contiguous vs. chunked; `acquire` provides every buffer used.
pub fn read_frame_supplied<R: Read, S: BufferSupplier + ?Sized>(
    reader: &mut R,
    supplier: &S,
) -> Result<SuppliedFrame, DecodeError> {
    read_frame_supplied_with_limit(reader, supplier, DEFAULT_MAX_FRAME_SIZE)
}

/// [`read_frame_supplied`] with a caller-chosen frame-size cap.
pub fn read_frame_supplied_with_limit<R: Read, S: BufferSupplier + ?Sized>(
    reader: &mut R,
    supplier: &S,
    max: usize,
) -> Result<SuppliedFrame, DecodeError> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = checked_len(i32::from_be_bytes(len_buf), max)?;
    match supplier.strategy(len) {
        ReadStrategy::Contiguous => {
            let mut buf = supplier.acquire(len);
            read_exact_into(reader, &mut buf, len)?;
            Ok(SuppliedFrame::Contiguous(buf.freeze()))
        }
        ReadStrategy::Chunked { chunk_size } => {
            let chunk_size = chunk_size.max(1);
            let mut chunks: Vec<Bytes> = Vec::with_capacity(len.div_ceil(chunk_size));
            let mut left = len;
            while left > 0 {
                let want = left.min(chunk_size);
                let mut buf = supplier.acquire(want);
                read_exact_into(reader, &mut buf, want)?;
                chunks.push(buf.freeze());
                left -= want;
            }
            Ok(SuppliedFrame::Chunked(ChunkChain::new(chunks)))
        }
    }
}

/// Append exactly `len` bytes from `reader` to `buf` (sync; the buffer tail is
/// zero-initialized before the read, which is cheap relative to the I/O).
fn read_exact_into<R: Read>(
    reader: &mut R,
    buf: &mut BytesMut,
    len: usize,
) -> Result<(), DecodeError> {
    let start = buf.len();
    buf.resize(start + len, 0);
    let mut filled = 0usize;
    while filled < len {
        match reader.read(&mut buf[start + filled..]) {
            Ok(0) => {
                buf.truncate(start);
                return Err(DecodeError::UnexpectedEof {
                    needed: len,
                    available: filled,
                });
            }
            Ok(n) => filled += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => {
                buf.truncate(start);
                return Err(e.into());
            }
        }
    }
    Ok(())
}

/// Async [`read_frame_supplied`].
#[cfg(feature = "async")]
pub async fn read_frame_supplied_async<R, S>(
    reader: &mut R,
    supplier: &S,
) -> Result<SuppliedFrame, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
    S: BufferSupplier + ?Sized,
{
    read_frame_supplied_async_with_limit(reader, supplier, DEFAULT_MAX_FRAME_SIZE).await
}

/// Async [`read_frame_supplied_with_limit`]. Chunk reads append into spare
/// capacity via `read_buf` — no pre-zeroing.
#[cfg(feature = "async")]
pub async fn read_frame_supplied_async_with_limit<R, S>(
    reader: &mut R,
    supplier: &S,
    max: usize,
) -> Result<SuppliedFrame, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
    S: BufferSupplier + ?Sized,
{
    use tokio::io::AsyncReadExt;
    let len = checked_len(reader.read_i32().await?, max)?;
    match supplier.strategy(len) {
        ReadStrategy::Contiguous => {
            let mut buf = supplier.acquire(len);
            fill_async(reader, &mut buf, len).await?;
            Ok(SuppliedFrame::Contiguous(buf.freeze()))
        }
        ReadStrategy::Chunked { chunk_size } => {
            let chunk_size = chunk_size.max(1);
            let mut chunks: Vec<Bytes> = Vec::with_capacity(len.div_ceil(chunk_size));
            let mut left = len;
            while left > 0 {
                let want = left.min(chunk_size);
                let mut buf = supplier.acquire(want);
                fill_async(reader, &mut buf, want).await?;
                chunks.push(buf.freeze());
                left -= want;
            }
            Ok(SuppliedFrame::Chunked(ChunkChain::new(chunks)))
        }
    }
}

/// Append exactly `len` bytes from `reader` to `buf` without pre-zeroing.
#[cfg(feature = "async")]
async fn fill_async<R>(reader: &mut R, buf: &mut BytesMut, len: usize) -> Result<(), DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
{
    use tokio::io::AsyncReadExt;
    let start = buf.len();
    buf.reserve(len);
    let mut limited = reader.take(len as u64);
    while buf.len() - start < len {
        if limited.read_buf(buf).await? == 0 {
            let available = buf.len() - start;
            buf.truncate(start);
            return Err(DecodeError::UnexpectedEof {
                needed: len,
                available,
            });
        }
    }
    Ok(())
}

/// Write one length-prefixed Kafka frame to a sync writer.
pub fn write_frame<W: Write>(writer: &mut W, body: &[u8]) -> Result<(), DecodeError> {
    let len = (body.len() as i32).to_be_bytes();
    writer.write_all(&len)?;
    writer.write_all(body)?;
    Ok(())
}

/// An encoded message body held as one or more `Bytes` segments, writable to
/// sync or async streams as `[int32 length][segments...]`.
///
/// The contiguous encode path ([`frame_request`]) produces a single segment;
/// the zero-copy path ([`frame_request_zero_copy`]) produces one segment per
/// large shared payload, with the small scalar runs between them. Segments are
/// refcounted `Bytes`, so cloning the frame is cheap and writing never copies.
pub struct EncodedFrame {
    segments: Vec<Bytes>,
    len: usize,
}

impl EncodedFrame {
    /// Wrap a pre-encoded contiguous body (without the length prefix).
    pub fn new(body: BytesMut) -> Self {
        let body = body.freeze();
        Self {
            len: body.len(),
            segments: vec![body],
        }
    }

    /// Finish a zero-copy encode into a frame.
    pub fn from_segments(buf: SegmentedBuf) -> Self {
        let segments = buf.into_segments();
        let len = segments.iter().map(|s| s.len()).sum();
        Self { segments, len }
    }

    /// The body segments, in wire order (length prefix not included).
    pub fn segments(&self) -> &[Bytes] {
        &self.segments
    }

    /// Consume the frame, returning the body segments (e.g. to feed a custom
    /// vectored writer).
    pub fn into_segments(self) -> Vec<Bytes> {
        self.segments
    }

    /// Body length in bytes (without the 4-byte length prefix).
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Copy the body into one contiguous `Bytes`. Single-segment frames return
    /// a cheap refcounted clone; multi-segment frames pay one copy — intended
    /// for tests and inspection, not the hot path.
    pub fn to_contiguous(&self) -> Bytes {
        match self.segments.as_slice() {
            [] => Bytes::new(),
            [one] => one.clone(),
            many => {
                let mut buf = BytesMut::with_capacity(self.len);
                for seg in many {
                    buf.extend_from_slice(seg);
                }
                buf.freeze()
            }
        }
    }

    /// Write [int32 length][body] to a sync writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
        let len = (self.len as i32).to_be_bytes();
        writer.write_all(&len)?;
        for seg in &self.segments {
            writer.write_all(seg)?;
        }
        Ok(())
    }

    /// Write [int32 length][body] to an async writer.
    #[cfg(feature = "async")]
    pub async fn write_to_async<W>(&self, writer: &mut W) -> Result<(), DecodeError>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        use tokio::io::AsyncWriteExt;
        let len = (self.len as i32).to_be_bytes();
        writer.write_all(&len).await?;
        for seg in &self.segments {
            writer.write_all(seg).await?;
        }
        Ok(())
    }
}

/// Read one length-prefixed Kafka frame from an async reader, rejecting frames
/// larger than [`DEFAULT_MAX_FRAME_SIZE`].
#[cfg(feature = "async")]
pub async fn read_frame_async<R>(reader: &mut R) -> Result<Bytes, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
{
    read_frame_async_with_limit(reader, DEFAULT_MAX_FRAME_SIZE).await
}

/// [`read_frame_async`] with a caller-chosen frame-size cap.
#[cfg(feature = "async")]
pub async fn read_frame_async_with_limit<R>(
    reader: &mut R,
    max: usize,
) -> Result<Bytes, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut body = BytesMut::new();
    read_frame_into_async_with_limit(reader, &mut body, max).await
}

/// [`read_frame_async`] into a caller-owned buffer, enabling **buffer reuse
/// across frames**: the frame body is appended to `buf`'s tail (via `read_buf`,
/// no pre-zeroing) and split off as a zero-copy `Bytes` view.
///
/// Once every `Bytes` returned from a given `buf` has been dropped, the next
/// call's `reserve` reclaims the full allocation — steady-state reads allocate
/// nothing. `BytesMut` grows itself on demand, so an undersized buffer is never
/// an error; it just pays one (amortized) regrow. Callers tracking buffer stats
/// can inspect `buf.capacity()` between calls, and pre-size with
/// `BytesMut::with_capacity` / `reserve`.
///
/// Any bytes already in `buf` are left untouched (only the tail is consumed).
#[cfg(feature = "async")]
pub async fn read_frame_into_async<R>(
    reader: &mut R,
    buf: &mut BytesMut,
) -> Result<Bytes, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
{
    read_frame_into_async_with_limit(reader, buf, DEFAULT_MAX_FRAME_SIZE).await
}

/// [`read_frame_into_async`] with a caller-chosen frame-size cap.
#[cfg(feature = "async")]
pub async fn read_frame_into_async_with_limit<R>(
    reader: &mut R,
    buf: &mut BytesMut,
    max: usize,
) -> Result<Bytes, DecodeError>
where
    R: tokio::io::AsyncRead + Unpin,
{
    use tokio::io::AsyncReadExt;
    let len = checked_len(reader.read_i32().await?, max)?;
    let start = buf.len();
    // Reclaims the allocation when the previously returned `Bytes` views have
    // been dropped; grows (amortized) otherwise.
    buf.reserve(len);
    // The `take` adapter guarantees we never read past this frame into the next.
    let mut limited = reader.take(len as u64);
    while buf.len() - start < len {
        if limited.read_buf(buf).await? == 0 {
            let available = buf.len() - start;
            buf.truncate(start);
            return Err(DecodeError::UnexpectedEof {
                needed: len,
                available,
            });
        }
    }
    Ok(take_frame(buf, start, len))
}
