#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

/// Valid versions: 0-5.
#[derive(Debug, Clone, Default)]
pub struct SyncGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The group protocol type.
    pub protocol_type: Option<Bytes>,
    /// The group protocol name.
    pub protocol_name: Option<Bytes>,
    /// The member assignment.
    pub assignment: Bytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl SyncGroupResponse {
    pub const API_KEY: i16 = 14;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        {
            size += 2;
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 4 { compact_nullable_string_size(self.protocol_type.as_deref()) } else { nullable_string_size(self.protocol_type.as_deref()) } } else { let v = self.protocol_type.as_deref().expect("field protocol_type is None but not nullable at this version"); if version >= 4 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 4 { compact_nullable_string_size(self.protocol_name.as_deref()) } else { nullable_string_size(self.protocol_name.as_deref()) } } else { let v = self.protocol_name.as_deref().expect("field protocol_name is None but not nullable at this version"); if version >= 4 { compact_string_size(v) } else { string_size(v) } };
        }
        {
            size += if version >= 4 { compact_bytes_size(&self.assignment) } else { bytes_size(&self.assignment) };
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            put_i16(buf, self.error_code);
        }
        if version >= 5 {
            if version >= 5 { if version >= 4 { put_compact_nullable_string(buf, self.protocol_type.as_deref()) } else { put_nullable_string(buf, self.protocol_type.as_deref()) } } else { let v = self.protocol_type.as_deref().expect("field protocol_type is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 5 {
            if version >= 5 { if version >= 4 { put_compact_nullable_string(buf, self.protocol_name.as_deref()) } else { put_nullable_string(buf, self.protocol_name.as_deref()) } } else { let v = self.protocol_name.as_deref().expect("field protocol_name is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            if version >= 4 { put_compact_bytes_zc(buf, &self.assignment) } else { put_bytes_zc(buf, &self.assignment) };
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = SyncGroupResponse::default();
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 5 {
            msg.protocol_type = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 5 {
            msg.protocol_name = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.assignment = (if version >= 4 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for SyncGroupResponse {
    const API_KEY: i16 = 14;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for SyncGroupResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
