use crate::codec::SegmentedBuf;
use crate::error::DecodeError;
use crate::header::{RequestHeader, ResponseHeader};
use crate::message::{Encodable, EncodableZeroCopy};
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
) -> EncodedFrame {
    let size = header.encoded_size(header_version) + body.wire_size(api_version);
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.write(api_version, &mut buf);
    EncodedFrame::new(buf)
}

/// Build a complete length-prefixed response frame: `[len][header][body]`.
pub fn frame_response<M: Encodable>(
    header: &ResponseHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> EncodedFrame {
    let size = header.encoded_size(header_version) + body.wire_size(api_version);
    let mut buf = BytesMut::with_capacity(size);
    header.encode(&mut buf, header_version);
    body.write(api_version, &mut buf);
    EncodedFrame::new(buf)
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
) -> EncodedFrame {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.write_segmented(api_version, &mut buf);
    EncodedFrame::from_segments(buf)
}

/// Zero-copy, single-pass variant of [`frame_response`]: large `Bytes` payloads
/// (e.g. fetch record batches) become refcounted segments of the frame instead
/// of being memcpy'd.
pub fn frame_response_zero_copy<M: EncodableZeroCopy>(
    header: &ResponseHeader,
    header_version: i16,
    body: &M,
    api_version: i16,
) -> EncodedFrame {
    let mut buf = SegmentedBuf::new();
    header.encode(&mut buf, header_version);
    body.write_segmented(api_version, &mut buf);
    EncodedFrame::from_segments(buf)
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
    use tokio::io::AsyncReadExt;
    let len = checked_len(reader.read_i32().await?, max)?;
    // `read_buf` appends into spare capacity without pre-zeroing; the `take`
    // adapter guarantees we never read past this frame into the next one.
    let mut body = BytesMut::with_capacity(len);
    let mut limited = reader.take(len as u64);
    while body.len() < len {
        if limited.read_buf(&mut body).await? == 0 {
            return Err(DecodeError::UnexpectedEof {
                needed: len,
                available: body.len(),
            });
        }
    }
    Ok(body.freeze())
}
