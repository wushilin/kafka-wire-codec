#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone)]
pub struct CreatableTopicResult {
    /// The topic name.
    pub name: Bytes,
    /// The unique topic ID.
    pub topic_id: [u8; 16],
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<Bytes>,
    /// Number of partitions of the topic.
    pub num_partitions: i32,
    /// Replication factor of the topic.
    pub replication_factor: i16,
    /// Configuration of the topic.
    pub configs: Option<Vec<CreatableTopicConfigs>>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for CreatableTopicResult {
    fn default() -> Self {
        Self {
            name: Bytes::new(),
            topic_id: [0u8; 16],
            error_code: 0,
            error_message: Some(Bytes::new()),
            num_partitions: -1,
            replication_factor: -1,
            configs: Some(Vec::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl CreatableTopicResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 5 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version >= 7 {
            size += 16;
        }
        {
            size += 2;
        }
        if version >= 1 {
            size += if version >= 5 { compact_nullable_string_size(self.error_message.as_deref()) } else { nullable_string_size(self.error_message.as_deref()) };
        }
        if version >= 5 {
            size += 4;
        }
        if version >= 5 {
            size += 2;
        }
        if version >= 5 {
            match &self.configs {
                Some(arr) => {
                if version >= 5 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    assert!(version >= 5, "field configs is None but not nullable at version {}", version);
                    if version >= 5 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 5 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version >= 7 {
            put_uuid(buf, &self.topic_id);
        }
        {
            put_i16(buf, self.error_code);
        }
        if version >= 1 {
            if version >= 5 { put_compact_nullable_string(buf, self.error_message.as_deref()) } else { put_nullable_string(buf, self.error_message.as_deref()) };
        }
        if version >= 5 {
            put_i32(buf, self.num_partitions);
        }
        if version >= 5 {
            put_i16(buf, self.replication_factor);
        }
        if version >= 5 {
            match &self.configs {
                Some(arr) => {
                if version >= 5 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    assert!(version >= 5, "field configs is None but not nullable at version {}", version);
                    if version >= 5 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = CreatableTopicResult::default();
        {
            msg.name = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 7 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 1 {
            msg.error_message = if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 5 {
            msg.num_partitions = get_i32(buf)?;
        }
        if version >= 5 {
            msg.replication_factor = get_i16(buf)?;
        }
        if version >= 5 {
            let len_opt = if version >= 5 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.configs = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(CreatableTopicConfigs::decode(version, buf)?); }
                Some(items)
                }
                None => { if version >= 5 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct CreatableTopicConfigs {
    /// The configuration name.
    pub name: Bytes,
    /// The configuration value.
    pub value: Option<Bytes>,
    /// True if the configuration is read-only.
    pub read_only: bool,
    /// The configuration source.
    pub config_source: i8,
    /// True if this configuration is sensitive.
    pub is_sensitive: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for CreatableTopicConfigs {
    fn default() -> Self {
        Self {
            name: Bytes::new(),
            value: Some(Bytes::new()),
            read_only: false,
            config_source: -1,
            is_sensitive: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl CreatableTopicConfigs {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 5 {
            size += if version >= 5 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 5 { compact_nullable_string_size(self.value.as_deref()) } else { nullable_string_size(self.value.as_deref()) } } else { let v = self.value.as_deref().expect("field value is None but not nullable at this version"); if version >= 5 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 5 {
            size += 1;
        }
        if version >= 5 {
            size += 1;
        }
        if version >= 5 {
            size += 1;
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 5 {
            if version >= 5 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version >= 5 {
            if version >= 5 { if version >= 5 { put_compact_nullable_string(buf, self.value.as_deref()) } else { put_nullable_string(buf, self.value.as_deref()) } } else { let v = self.value.as_deref().expect("field value is None but not nullable at this version"); if version >= 5 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 5 {
            put_bool(buf, self.read_only);
        }
        if version >= 5 {
            put_i8(buf, self.config_source);
        }
        if version >= 5 {
            put_bool(buf, self.is_sensitive);
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = CreatableTopicConfigs::default();
        if version >= 5 {
            msg.name = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 5 {
            msg.value = { let v = if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 5 {
            msg.read_only = get_bool(buf)?;
        }
        if version >= 5 {
            msg.config_source = get_i8(buf)?;
        }
        if version >= 5 {
            msg.is_sensitive = get_bool(buf)?;
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 2-7.
#[derive(Debug, Clone, Default)]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Results for each topic we tried to create.
    pub topics: Vec<CreatableTopicResult>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CreateTopicsResponse {
    pub const API_KEY: i16 = 19;
    pub const VALID_MIN_VERSION: i16 = 2;
    pub const VALID_MAX_VERSION: i16 = 7;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 5;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 2 {
            size += 4;
        }
        {
            { let arr = &self.topics;
                if version >= 5 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 2 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.topics;
                if version >= 5 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = CreateTopicsResponse::default();
        if version >= 2 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 5 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(CreatableTopicResult::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for CreateTopicsResponse {
    const API_KEY: i16 = 19;
    const VALID_MIN_VERSION: i16 = 2;
    const VALID_MAX_VERSION: i16 = 7;
    const FLEXIBLE_MIN_VERSION: i16 = 5;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for CreateTopicsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
