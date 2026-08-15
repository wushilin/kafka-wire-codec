#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

/// Valid versions: 1-3.
#[derive(Debug, Clone)]
pub struct DescribeAclsRequest {
    /// The resource type.
    pub resource_type_filter: i8,
    /// The resource name, or null to match any resource name.
    pub resource_name_filter: Option<Bytes>,
    /// The resource pattern to match.
    pub pattern_type_filter: i8,
    /// The principal to match, or null to match any principal.
    pub principal_filter: Option<Bytes>,
    /// The host to match, or null to match any host.
    pub host_filter: Option<Bytes>,
    /// The operation to match.
    pub operation: i8,
    /// The permission type to match.
    pub permission_type: i8,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeAclsRequest {
    fn default() -> Self {
        Self {
            resource_type_filter: 0,
            resource_name_filter: Some(Bytes::new()),
            pattern_type_filter: 3,
            principal_filter: Some(Bytes::new()),
            host_filter: Some(Bytes::new()),
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

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 1;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.resource_name_filter.as_deref()) } else { nullable_string_size(self.resource_name_filter.as_deref()) };
        }
        if version >= 1 {
            size += 1;
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.principal_filter.as_deref()) } else { nullable_string_size(self.principal_filter.as_deref()) };
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.host_filter.as_deref()) } else { nullable_string_size(self.host_filter.as_deref()) };
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
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i8(buf, self.resource_type_filter);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.resource_name_filter.as_deref()) } else { put_nullable_string(buf, self.resource_name_filter.as_deref()) };
        }
        if version >= 1 {
            put_i8(buf, self.pattern_type_filter);
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.principal_filter.as_deref()) } else { put_nullable_string(buf, self.principal_filter.as_deref()) };
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.host_filter.as_deref()) } else { put_nullable_string(buf, self.host_filter.as_deref()) };
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
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeAclsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
