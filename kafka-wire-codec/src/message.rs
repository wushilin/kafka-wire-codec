use crate::error::DecodeError;
use bytes::{Bytes, BytesMut};

/// Implemented by every generated top-level request/response message.
///
/// This enables generic, **size-first** encoding: compute the exact wire size,
/// allocate once, then write — no reallocation, and the resulting buffer can be
/// flushed to a socket in a single write. See [`crate::frame`] for framing
/// helpers that prepend the 4-byte length and write to sync/async streams.
///
/// # Version-range contract
///
/// Decoding ([`Self::read`]) is fed by the network, so an out-of-range version
/// is a runtime condition and returns `Err(DecodeError::UnsupportedVersion)`.
/// Encoding ([`Self::wire_size`] / [`Self::write`]) is driven by the caller,
/// so an out-of-range version there is a **programmer error and panics** —
/// there is no partial frame to salvage once sizing and writing disagree.
/// Callers negotiating versions dynamically should gate on
/// [`Self::supports_version`] (or the `VALID_MIN/MAX_VERSION` constants)
/// before encoding.
///
/// `wire_size`/`write` forward to the inherent `encoded_size`/`encode` methods on
/// each generated type (kept under different names to avoid a self-referential
/// method-resolution clash).
pub trait Encodable: Sized {
    /// Kafka API key for this message (request and response share it).
    const API_KEY: i16;

    /// Lowest protocol version supported by this generated message.
    const VALID_MIN_VERSION: i16;

    /// Highest protocol version supported by this generated message.
    const VALID_MAX_VERSION: i16;

    /// First flexible (tagged-fields/compact-encoding) version of this
    /// message; `i16::MAX` if the message is never flexible.
    const FLEXIBLE_MIN_VERSION: i16;

    /// Whether `version` can be encoded/decoded by this message. Encoding at
    /// an unsupported version panics — check this first when the version is
    /// not one you just decoded.
    fn supports_version(version: i16) -> bool {
        (Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version)
    }

    /// Exact encoded size in bytes at `version` (first pass).
    /// Panics if `version` is out of range — see the trait docs.
    fn wire_size(&self, version: i16) -> usize;

    /// Encode the body into `buf` at `version` (second pass).
    /// Panics if `version` is out of range — see the trait docs.
    fn write(&self, version: i16, buf: &mut BytesMut);

    /// Decode a body from `buf` at `version`, consuming the bytes read.
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError>;

    /// Size-first encode into a freshly allocated, exact-capacity buffer.
    /// One allocation, no reallocation; ready to write in a single syscall.
    fn to_bytes(&self, version: i16) -> BytesMut {
        let mut buf = BytesMut::with_capacity(self.wire_size(version));
        self.write(version, &mut buf);
        buf
    }

    /// Decode a body and require that every byte was consumed. Prefer this over
    /// [`Self::read`] when the input is exactly one message body — a decode at
    /// the wrong version usually surfaces as leftover bytes, which `read`
    /// silently ignores.
    fn read_all(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let msg = Self::read(version, buf)?;
        if !buf.is_empty() {
            return Err(DecodeError::TrailingBytes {
                remaining: buf.len(),
            });
        }
        Ok(msg)
    }
}

/// Zero-copy encoding, implemented by every generated message alongside
/// [`Encodable`]. Kept as a separate trait so `Encodable` stays non-generic.
pub trait EncodableZeroCopy: Encodable {
    /// Encode into a segmented sink: large `Bytes` payloads (record batches,
    /// raw bytes fields) are appended as refcounted segments — never copied.
    fn write_segmented(&self, version: i16, buf: &mut crate::codec::SegmentedBuf);

    /// Zero-copy, single-pass encode. No sizing pass is needed — the total
    /// length is the sum of the segment lengths.
    fn to_segments(&self, version: i16) -> crate::codec::SegmentedBuf {
        let mut buf = crate::codec::SegmentedBuf::new();
        self.write_segmented(version, &mut buf);
        buf
    }
}
