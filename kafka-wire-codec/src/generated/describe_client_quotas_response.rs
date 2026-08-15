#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EntryData {
    /// The quota entity description.
    pub entity: Vec<EntityData>,
    /// The quota values for the entity.
    pub values: Vec<ValueData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl EntryData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            { let arr = &self.entity;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.values;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            { let arr = &self.entity;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.values;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = EntryData::default();
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(EntityData::decode(version, buf)?); }
            msg.entity = items; }
        }
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ValueData::decode(version, buf)?); }
            msg.values = items; }
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityData {
    /// The entity type.
    pub entity_type: StrBytes,
    /// The entity name, or null if the default.
    pub entity_name: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_type: StrBytes::new(),
            entity_name: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl EntityData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 1 { compact_string_size(self.entity_type.as_str()) } else { string_size(self.entity_type.as_str()) };
        }
        {
            size += if version >= 1 { compact_nullable_string_size(self.entity_name.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.entity_name.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 1 { put_compact_string(buf, self.entity_type.as_str()) } else { put_string(buf, self.entity_type.as_str()) };
        }
        {
            if version >= 1 { put_compact_nullable_string(buf, self.entity_name.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.entity_name.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = EntityData::default();
        {
            msg.entity_type = (if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.entity_name = if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValueData {
    /// The quota configuration key.
    pub key: StrBytes,
    /// The quota configuration value.
    pub value: f64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ValueData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 1 { compact_string_size(self.key.as_str()) } else { string_size(self.key.as_str()) };
        }
        {
            size += 8;
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 1 { put_compact_string(buf, self.key.as_str()) } else { put_string(buf, self.key.as_str()) };
        }
        {
            put_f64(buf, self.value);
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ValueData::default();
        {
            msg.key = (if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.value = get_f64(buf)?;
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-1.
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeClientQuotasResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or `0` if the quota description succeeded.
    pub error_code: i16,
    /// The error message, or `null` if the quota description succeeded.
    pub error_message: Option<StrBytes>,
    /// A result entry.
    pub entries: Option<Vec<EntryData>>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeClientQuotasResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(StrBytes::new()),
            entries: Some(Vec::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClientQuotasResponse {
    pub const API_KEY: i16 = 48;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 1;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += if version >= 1 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            match &self.entries {
                Some(arr) => {
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    if version >= 1 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 1 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            match &self.entries {
                Some(arr) => {
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    if version >= 1 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeClientQuotasResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.entries = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(EntryData::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeClientQuotasResponse {
    const API_KEY: i16 = 48;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 1;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeClientQuotasResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
