#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct AclCreation {
    /// The type of the resource.
    pub resource_type: i8,
    /// The resource name for the ACL.
    pub resource_name: StrBytes,
    /// The pattern type for the ACL.
    pub resource_pattern_type: i8,
    /// The principal for the ACL.
    pub principal: StrBytes,
    /// The host for the ACL.
    pub host: StrBytes,
    /// The operation type for the ACL (read, write, etc.).
    pub operation: i8,
    /// The permission type for the ACL (allow, deny, etc.).
    pub permission_type: i8,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for AclCreation {
    fn default() -> Self {
        Self {
            resource_type: 0,
            resource_name: StrBytes::new(),
            resource_pattern_type: 3,
            principal: StrBytes::new(),
            host: StrBytes::new(),
            operation: 0,
            permission_type: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl AclCreation {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
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
            put_i8(buf, self.resource_type);
        }
        {
            if version >= 2 { put_compact_string(buf, self.resource_name.as_str()) } else { put_string(buf, self.resource_name.as_str()) };
        }
        if version >= 1 {
            put_i8(buf, self.resource_pattern_type);
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
        let mut msg = AclCreation::default();
        {
            msg.resource_type = get_i8(buf)?;
        }
        {
            msg.resource_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.resource_pattern_type = get_i8(buf)?;
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
#[derive(Debug, Clone, Default)]
pub struct CreateAclsRequest {
    /// The ACLs that we want to create.
    pub creations: Vec<AclCreation>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CreateAclsRequest {
    pub const API_KEY: i16 = 30;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 3;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            { let arr = &self.creations;
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
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            { let arr = &self.creations;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = CreateAclsRequest::default();
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AclCreation::decode(version, buf)?); }
            msg.creations = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for CreateAclsRequest {
    const API_KEY: i16 = 30;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for CreateAclsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
