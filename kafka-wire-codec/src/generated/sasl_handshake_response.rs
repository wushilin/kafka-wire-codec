#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-1.
#[derive(Debug, Clone, Default)]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The mechanisms enabled in the server.
    pub mechanisms: Vec<StrBytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl SaslHandshakeResponse {
    pub const API_KEY: i16 = 17;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            { let arr = &self.mechanisms;
                size += 4;
                for item in arr {
                    size += string_size(item.as_str());
                }
            }
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.mechanisms;
                put_i32(buf, arr.len() as i32);
                for item in arr { put_string(buf, item.as_str()); }
            }
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = SaslHandshakeResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.mechanisms = items; }
        }
        Ok(msg)
    }
}

impl crate::Encodable for SaslHandshakeResponse {
    const API_KEY: i16 = 17;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for SaslHandshakeResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
