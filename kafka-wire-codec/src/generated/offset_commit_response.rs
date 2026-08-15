#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct OffsetCommitResponseTopic {
    /// The topic name.
    pub name: TopicName,
    /// The topic ID.
    pub topic_id: Uuid,
    /// The responses for each partition in the topic.
    pub partitions: Vec<OffsetCommitResponsePartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetCommitResponseTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 9 {
            size += if version >= 8 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 10 {
            size += 16;
        }
        {
            { let arr = &self.partitions;
                if version >= 8 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 9 {
            if version >= 8 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        {
            { let arr = &self.partitions;
                if version >= 8 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetCommitResponseTopic::default();
        if version <= 9 {
            msg.name = TopicName((if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 10 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = if version >= 8 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetCommitResponsePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OffsetCommitResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetCommitResponsePartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i16(buf, self.error_code);
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetCommitResponsePartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 2-10.
#[derive(Debug, Clone, Default)]
pub struct OffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each topic.
    pub topics: Vec<OffsetCommitResponseTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetCommitResponse {
    pub const API_KEY: i16 = 8;
    pub const VALID_MIN_VERSION: i16 = 2;
    pub const VALID_MAX_VERSION: i16 = 10;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 8;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 3 {
            size += 4;
        }
        {
            { let arr = &self.topics;
                if version >= 8 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 3 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.topics;
                if version >= 8 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = OffsetCommitResponse::default();
        if version >= 3 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 8 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetCommitResponseTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetCommitResponse {
    const API_KEY: i16 = 8;
    const VALID_MIN_VERSION: i16 = 2;
    const VALID_MAX_VERSION: i16 = 10;
    const FLEXIBLE_MIN_VERSION: i16 = 8;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetCommitResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
