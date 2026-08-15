#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct AlterConfigsResource {
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    pub resource_name: StrBytes,
    /// The configurations.
    pub configs: Vec<AlterableConfig>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AlterConfigsResource {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 1;
        }
        {
            size += if version >= 2 { compact_string_size(self.resource_name.as_str()) } else { string_size(self.resource_name.as_str()) };
        }
        {
            { let arr = &self.configs;
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
            put_i8(buf, self.resource_type);
        }
        {
            if version >= 2 { put_compact_string(buf, self.resource_name.as_str()) } else { put_string(buf, self.resource_name.as_str()) };
        }
        {
            { let arr = &self.configs;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AlterConfigsResource::default();
        {
            msg.resource_type = get_i8(buf)?;
        }
        {
            msg.resource_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AlterableConfig::decode(version, buf)?); }
            msg.configs = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct AlterableConfig {
    /// The configuration key name.
    pub name: StrBytes,
    /// The value to set for the configuration key.
    pub value: Option<StrBytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for AlterableConfig {
    fn default() -> Self {
        Self {
            name: StrBytes::new(),
            value: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl AlterableConfig {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        {
            size += if version >= 2 { compact_nullable_string_size(self.value.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.value.as_ref().map(|v| v.as_str())) };
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        {
            if version >= 2 { put_compact_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) };
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AlterableConfig::default();
        {
            msg.name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.value = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-2.
#[derive(Debug, Clone, Default)]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    pub resources: Vec<AlterConfigsResource>,
    /// True if we should validate the request, but not change the configurations.
    pub validate_only: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AlterConfigsRequest {
    pub const API_KEY: i16 = 33;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            { let arr = &self.resources;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
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
            { let arr = &self.resources;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_bool(buf, self.validate_only);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = AlterConfigsRequest::default();
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AlterConfigsResource::decode(version, buf)?); }
            msg.resources = items; }
        }
        {
            msg.validate_only = get_bool(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for AlterConfigsRequest {
    const API_KEY: i16 = 33;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for AlterConfigsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
