#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 1-2.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RenewDelegationTokenRequest {
    /// The HMAC of the delegation token to be renewed.
    pub hmac: Bytes,
    /// The renewal time period in milliseconds.
    pub renew_period_ms: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl RenewDelegationTokenRequest {
    pub const API_KEY: i16 = 39;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_bytes_size(&self.hmac) } else { bytes_size(&self.hmac) };
        }
        {
            size += 8;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 2 { put_compact_bytes_zc(buf, &self.hmac) } else { put_bytes_zc(buf, &self.hmac) };
        }
        {
            put_i64(buf, self.renew_period_ms);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = RenewDelegationTokenRequest::default();
        {
            msg.hmac = (if version >= 2 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.renew_period_ms = get_i64(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for RenewDelegationTokenRequest {
    const API_KEY: i16 = 39;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for RenewDelegationTokenRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
