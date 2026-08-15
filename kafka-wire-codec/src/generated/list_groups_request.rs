#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

/// Valid versions: 0-5.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListGroupsRequest {
    /// The states of the groups we want to list. If empty, all groups are returned with their state.
    pub states_filter: Vec<StrBytes>,
    /// The types of the groups we want to list. If empty, all groups are returned with their type.
    pub types_filter: Vec<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ListGroupsRequest {
    pub const API_KEY: i16 = 16;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 4 {
            { let arr = &self.states_filter;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += if version >= 3 { compact_string_size(item.as_str()) } else { string_size(item.as_str()) };
                }
            }
        }
        if version >= 5 {
            { let arr = &self.types_filter;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += if version >= 3 { compact_string_size(item.as_str()) } else { string_size(item.as_str()) };
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 4 {
            { let arr = &self.states_filter;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { if version >= 3 { put_compact_string(buf, item.as_str()); } else { put_string(buf, item.as_str()); } }
            }
        }
        if version >= 5 {
            { let arr = &self.types_filter;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { if version >= 3 { put_compact_string(buf, item.as_str()); } else { put_string(buf, item.as_str()); } }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ListGroupsRequest::default();
        if version >= 4 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((if version >= 3 { get_compact_string(buf) } else { get_string(buf) }).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.states_filter = items; }
        }
        if version >= 5 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((if version >= 3 { get_compact_string(buf) } else { get_string(buf) }).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.types_filter = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for ListGroupsRequest {
    const API_KEY: i16 = 16;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ListGroupsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
