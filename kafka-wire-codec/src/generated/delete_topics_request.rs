#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeleteTopicState {
    /// The topic name.
    pub name: Option<TopicName>,
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DeleteTopicState {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 6 {
            size += if version >= 6 { if version >= 4 { compact_nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 4 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 6 {
            size += 16;
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 6 {
            if version >= 6 { if version >= 4 { put_compact_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 6 {
            put_uuid(buf, &self.topic_id);
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DeleteTopicState::default();
        if version >= 6 {
            msg.name = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 6 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } }.map(TopicName);
        }
        if version >= 6 {
            msg.topic_id = get_uuid(buf)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-6.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeleteTopicsRequest {
    /// The name or topic ID of the topic.
    pub topics: Vec<DeleteTopicState>,
    /// The names of the topics to delete.
    pub topic_names: Vec<TopicName>,
    /// The length of time in milliseconds to wait for the deletions to complete.
    pub timeout_ms: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DeleteTopicsRequest {
    pub const API_KEY: i16 = 20;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 6;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 6 {
            { let arr = &self.topics;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version <= 5 {
            { let arr = &self.topic_names;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += if version >= 4 { compact_string_size(item.as_str()) } else { string_size(item.as_str()) };
                }
            }
        }
        {
            size += 4;
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 6 {
            { let arr = &self.topics;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version <= 5 {
            { let arr = &self.topic_names;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { if version >= 4 { put_compact_string(buf, item.as_str()); } else { put_string(buf, item.as_str()); } }
            }
        }
        {
            put_i32(buf, self.timeout_ms);
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DeleteTopicsRequest::default();
        if version >= 6 {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DeleteTopicState::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version <= 5 {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(((if version >= 4 { get_compact_string(buf) } else { get_string(buf) }).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TopicName)?); }
            msg.topic_names = items; }
        }
        {
            msg.timeout_ms = get_i32(buf)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DeleteTopicsRequest {
    const API_KEY: i16 = 20;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 6;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DeleteTopicsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
