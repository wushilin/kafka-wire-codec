#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteAclsFilterResult {
    /// The error code, or 0 if the filter succeeded.
    pub error_code: i16,
    /// The error message, or null if the filter succeeded.
    pub error_message: Option<StrBytes>,
    /// The ACLs which matched this filter.
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DeleteAclsFilterResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(StrBytes::new()),
            matching_acls: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsFilterResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            { let arr = &self.matching_acls;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            { let arr = &self.matching_acls;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DeleteAclsFilterResult::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DeleteAclsMatchingAcl::decode(version, buf)?); }
            msg.matching_acls = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteAclsMatchingAcl {
    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// The deletion error message, or null if the deletion succeeded.
    pub error_message: Option<StrBytes>,
    /// The ACL resource type.
    pub resource_type: i8,
    /// The ACL resource name.
    pub resource_name: StrBytes,
    /// The ACL resource pattern type.
    pub pattern_type: i8,
    /// The ACL principal.
    pub principal: StrBytes,
    /// The ACL host.
    pub host: StrBytes,
    /// The ACL operation.
    pub operation: i8,
    /// The ACL permission type.
    pub permission_type: i8,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DeleteAclsMatchingAcl {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(StrBytes::new()),
            resource_type: 0,
            resource_name: StrBytes::new(),
            pattern_type: 3,
            principal: StrBytes::new(),
            host: StrBytes::new(),
            operation: 0,
            permission_type: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsMatchingAcl {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            size += 1;
        }
        {
            size += if version >= 2 { compact_string_size(self.resource_name.as_str()) } else { string_size(self.resource_name.as_str()) };
        }
        if version >= 1 {
            size += 1;
        }
        {
            size += if version >= 2 { compact_string_size(self.principal.as_str()) } else { string_size(self.principal.as_str()) };
        }
        {
            size += if version >= 2 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        {
            size += 1;
        }
        {
            size += 1;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            put_i8(buf, self.resource_type);
        }
        {
            if version >= 2 { put_compact_string(buf, self.resource_name.as_str()) } else { put_string(buf, self.resource_name.as_str()) };
        }
        if version >= 1 {
            put_i8(buf, self.pattern_type);
        }
        {
            if version >= 2 { put_compact_string(buf, self.principal.as_str()) } else { put_string(buf, self.principal.as_str()) };
        }
        {
            if version >= 2 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        {
            put_i8(buf, self.operation);
        }
        {
            put_i8(buf, self.permission_type);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DeleteAclsMatchingAcl::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            msg.resource_type = get_i8(buf)?;
        }
        {
            msg.resource_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.pattern_type = get_i8(buf)?;
        }
        {
            msg.principal = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.host = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.operation = get_i8(buf)?;
        }
        {
            msg.permission_type = get_i8(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-3.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeleteAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each filter.
    pub filter_results: Vec<DeleteAclsFilterResult>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DeleteAclsResponse {
    pub const API_KEY: i16 = 31;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 3;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.filter_results;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.filter_results;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DeleteAclsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DeleteAclsFilterResult::decode(version, buf)?); }
            msg.filter_results = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DeleteAclsResponse {
    const API_KEY: i16 = 31;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DeleteAclsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
