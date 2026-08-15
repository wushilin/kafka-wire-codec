#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct ComponentData {
    /// The entity type that the filter component applies to.
    pub entity_type: StrBytes,
    /// How to match the entity {0 = exact name, 1 = default name, 2 = any specified name}.
    pub match_type: i8,
    /// The string to match against, or null if unused for the match type.
    pub r#match: Option<StrBytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ComponentData {
    fn default() -> Self {
        Self {
            entity_type: StrBytes::new(),
            match_type: 0,
            r#match: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl ComponentData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 1 { compact_string_size(self.entity_type.as_str()) } else { string_size(self.entity_type.as_str()) };
        }
        {
            size += 1;
        }
        {
            size += if version >= 1 { compact_nullable_string_size(self.r#match.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.r#match.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 1 { put_compact_string(buf, self.entity_type.as_str()) } else { put_string(buf, self.entity_type.as_str()) };
        }
        {
            put_i8(buf, self.match_type);
        }
        {
            if version >= 1 { put_compact_nullable_string(buf, self.r#match.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.r#match.as_ref().map(|v| v.as_str())) };
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ComponentData::default();
        {
            msg.entity_type = (if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.match_type = get_i8(buf)?;
        }
        {
            msg.r#match = if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-1.
#[derive(Debug, Clone, Default)]
pub struct DescribeClientQuotasRequest {
    /// Filter components to apply to quota entities.
    pub components: Vec<ComponentData>,
    /// Whether the match is strict, i.e. should exclude entities with unspecified entity types.
    pub strict: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeClientQuotasRequest {
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
            { let arr = &self.components;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 1;
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            { let arr = &self.components;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_bool(buf, self.strict);
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeClientQuotasRequest::default();
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ComponentData::decode(version, buf)?); }
            msg.components = items; }
        }
        {
            msg.strict = get_bool(buf)?;
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeClientQuotasRequest {
    const API_KEY: i16 = 48;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 1;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeClientQuotasRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
