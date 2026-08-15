#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    pub error_code: i16,
    /// The error message, or null if we were able to successfully describe the configurations.
    pub error_message: Option<StrBytes>,
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    pub resource_name: StrBytes,
    /// Each listed configuration.
    pub configs: Vec<DescribeConfigsResourceResult>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeConfigsResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(StrBytes::new()),
            resource_type: 0,
            resource_name: StrBytes::new(),
            configs: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeConfigsResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += if version >= 4 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            size += 1;
        }
        {
            size += if version >= 4 { compact_string_size(self.resource_name.as_str()) } else { string_size(self.resource_name.as_str()) };
        }
        {
            { let arr = &self.configs;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 4 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            put_i8(buf, self.resource_type);
        }
        {
            if version >= 4 { put_compact_string(buf, self.resource_name.as_str()) } else { put_string(buf, self.resource_name.as_str()) };
        }
        {
            { let arr = &self.configs;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeConfigsResult::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            msg.resource_type = get_i8(buf)?;
        }
        {
            msg.resource_name = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeConfigsResourceResult::decode(version, buf)?); }
            msg.configs = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    pub name: StrBytes,
    /// The configuration value.
    pub value: Option<StrBytes>,
    /// True if the configuration is read-only.
    pub read_only: bool,
    /// The configuration source.
    pub config_source: i8,
    /// True if this configuration is sensitive.
    pub is_sensitive: bool,
    /// The synonyms for this configuration key.
    pub synonyms: Vec<DescribeConfigsSynonym>,
    /// The configuration data type. Type can be one of the following values - BOOLEAN, STRING, INT, SHORT, LONG, DOUBLE, LIST, CLASS, PASSWORD.
    pub config_type: i8,
    /// The configuration documentation.
    pub documentation: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeConfigsResourceResult {
    fn default() -> Self {
        Self {
            name: StrBytes::new(),
            value: Some(StrBytes::new()),
            read_only: false,
            config_source: -1,
            is_sensitive: false,
            synonyms: Vec::new(),
            config_type: 0,
            documentation: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeConfigsResourceResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 4 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        {
            size += if version >= 4 { compact_nullable_string_size(self.value.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.value.as_ref().map(|v| v.as_str())) };
        }
        {
            size += 1;
        }
        if version >= 1 {
            size += 1;
        }
        {
            size += 1;
        }
        if version >= 1 {
            { let arr = &self.synonyms;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 {
            size += 1;
        }
        if version >= 3 {
            size += if version >= 4 { compact_nullable_string_size(self.documentation.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.documentation.as_ref().map(|v| v.as_str())) };
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 4 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        {
            if version >= 4 { put_compact_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) };
        }
        {
            put_bool(buf, self.read_only);
        }
        if version >= 1 {
            put_i8(buf, self.config_source);
        }
        {
            put_bool(buf, self.is_sensitive);
        }
        if version >= 1 {
            { let arr = &self.synonyms;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 {
            put_i8(buf, self.config_type);
        }
        if version >= 3 {
            if version >= 4 { put_compact_nullable_string(buf, self.documentation.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.documentation.as_ref().map(|v| v.as_str())) };
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeConfigsResourceResult::default();
        {
            msg.name = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.value = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            msg.read_only = get_bool(buf)?;
        }
        if version >= 1 {
            msg.config_source = get_i8(buf)?;
        }
        {
            msg.is_sensitive = get_bool(buf)?;
        }
        if version >= 1 {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeConfigsSynonym::decode(version, buf)?); }
            msg.synonyms = items; }
        }
        if version >= 3 {
            msg.config_type = get_i8(buf)?;
        }
        if version >= 3 {
            msg.documentation = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    pub name: StrBytes,
    /// The synonym value.
    pub value: Option<StrBytes>,
    /// The synonym source.
    pub source: i8,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeConfigsSynonym {
    fn default() -> Self {
        Self {
            name: StrBytes::new(),
            value: Some(StrBytes::new()),
            source: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeConfigsSynonym {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 1 {
            size += if version >= 4 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 1 {
            size += if version >= 4 { compact_nullable_string_size(self.value.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.value.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 {
            size += 1;
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 1 {
            if version >= 4 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        if version >= 1 {
            if version >= 4 { put_compact_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.value.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 {
            put_i8(buf, self.source);
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeConfigsSynonym::default();
        if version >= 1 {
            msg.name = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.value = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 1 {
            msg.source = get_i8(buf)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-4.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each resource.
    pub results: Vec<DescribeConfigsResult>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeConfigsResponse {
    pub const API_KEY: i16 = 32;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.results;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
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
            { let arr = &self.results;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeConfigsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeConfigsResult::decode(version, buf)?); }
            msg.results = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeConfigsResponse {
    const API_KEY: i16 = 32;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeConfigsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
