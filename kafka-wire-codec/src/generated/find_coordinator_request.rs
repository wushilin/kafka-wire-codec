#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-6.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FindCoordinatorRequest {
    /// The coordinator key.
    pub key: StrBytes,
    /// The coordinator key type. (group, transaction, share).
    pub key_type: i8,
    /// The coordinator keys.
    pub coordinator_keys: Vec<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FindCoordinatorRequest {
    pub const API_KEY: i16 = 10;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 6;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version <= 3 {
            size += if version >= 3 { compact_string_size(self.key.as_str()) } else { string_size(self.key.as_str()) };
        }
        if version >= 1 {
            size += 1;
        }
        if version >= 4 {
            { let arr = &self.coordinator_keys;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += if version >= 3 { compact_string_size(item.as_str()) } else { string_size(item.as_str()) };
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version <= 3 {
            if version >= 3 { put_compact_string(buf, self.key.as_str()) } else { put_string(buf, self.key.as_str()) };
        }
        if version >= 1 {
            put_i8(buf, self.key_type);
        }
        if version >= 4 {
            { let arr = &self.coordinator_keys;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { if version >= 3 { put_compact_string(buf, item.as_str()); } else { put_string(buf, item.as_str()); } }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = FindCoordinatorRequest::default();
        if version <= 3 {
            msg.key = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.key_type = get_i8(buf)?;
        }
        if version >= 4 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((if version >= 3 { get_compact_string(buf) } else { get_string(buf) }).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.coordinator_keys = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for FindCoordinatorRequest {
    const API_KEY: i16 = 10;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 6;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FindCoordinatorRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
