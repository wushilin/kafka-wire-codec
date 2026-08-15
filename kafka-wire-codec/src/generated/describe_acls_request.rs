#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

/// Valid versions: 1-3.
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeAclsRequest {
    /// The resource type.
    pub resource_type_filter: i8,
    /// The resource name, or null to match any resource name.
    pub resource_name_filter: Option<StrBytes>,
    /// The resource pattern to match.
    pub pattern_type_filter: i8,
    /// The principal to match, or null to match any principal.
    pub principal_filter: Option<StrBytes>,
    /// The host to match, or null to match any host.
    pub host_filter: Option<StrBytes>,
    /// The operation to match.
    pub operation: i8,
    /// The permission type to match.
    pub permission_type: i8,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeAclsRequest {
    fn default() -> Self {
        Self {
            resource_type_filter: 0,
            resource_name_filter: Some(StrBytes::new()),
            pattern_type_filter: 3,
            principal_filter: Some(StrBytes::new()),
            host_filter: Some(StrBytes::new()),
            operation: 0,
            permission_type: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeAclsRequest {
    pub const API_KEY: i16 = 29;
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
            size += 1;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.resource_name_filter.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.resource_name_filter.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 {
            size += 1;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.principal_filter.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.principal_filter.as_ref().map(|v| v.as_str())) };
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.host_filter.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.host_filter.as_ref().map(|v| v.as_str())) };
        }
        {
            size += 1;
        }
        {
            size += 1;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i8(buf, self.resource_type_filter);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.resource_name_filter.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.resource_name_filter.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 {
            put_i8(buf, self.pattern_type_filter);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.principal_filter.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.principal_filter.as_ref().map(|v| v.as_str())) };
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.host_filter.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.host_filter.as_ref().map(|v| v.as_str())) };
        }
        {
            put_i8(buf, self.operation);
        }
        {
            put_i8(buf, self.permission_type);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeAclsRequest::default();
        {
            msg.resource_type_filter = get_i8(buf)?;
        }
        {
            msg.resource_name_filter = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 1 {
            msg.pattern_type_filter = get_i8(buf)?;
        }
        {
            msg.principal_filter = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            msg.host_filter = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
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

impl crate::Encodable for DescribeAclsRequest {
    const API_KEY: i16 = 29;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeAclsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
