#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-4.
#[derive(Debug, Clone, Default)]
pub struct ApiVersionsRequest {
    /// The name of the client.
    pub client_software_name: StrBytes,
    /// The version of the client.
    pub client_software_version: StrBytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ApiVersionsRequest {
    pub const API_KEY: i16 = 18;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { compact_string_size(self.client_software_name.as_str()) } else { string_size(self.client_software_name.as_str()) };
        }
        if version >= 3 {
            size += if version >= 3 { compact_string_size(self.client_software_version.as_str()) } else { string_size(self.client_software_version.as_str()) };
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, self.client_software_name.as_str()) } else { put_string(buf, self.client_software_name.as_str()) };
        }
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, self.client_software_version.as_str()) } else { put_string(buf, self.client_software_version.as_str()) };
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ApiVersionsRequest::default();
        if version >= 3 {
            msg.client_software_name = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.client_software_version = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for ApiVersionsRequest {
    const API_KEY: i16 = 18;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ApiVersionsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
