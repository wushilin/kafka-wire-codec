#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    pub topic: TopicName,
    /// Each partition in the topic we fetched offsets for.
    pub partitions: Vec<EpochEndOffset>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetForLeaderTopicResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 4 { compact_string_size(self.topic.as_str()) } else { string_size(self.topic.as_str()) };
        }
        {
            { let arr = &self.partitions;
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
            if version >= 4 { put_compact_string(buf, self.topic.as_str()) } else { put_string(buf, self.topic.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetForLeaderTopicResult::default();
        {
            msg.topic = TopicName((if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(EpochEndOffset::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpochEndOffset {
    /// The error code 0, or if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition: i32,
    /// The leader epoch of the partition.
    pub leader_epoch: i32,
    /// The end offset of the epoch.
    pub end_offset: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for EpochEndOffset {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition: 0,
            leader_epoch: -1,
            end_offset: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl EpochEndOffset {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += 4;
        }
        if version >= 1 {
            size += 4;
        }
        {
            size += 8;
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i32(buf, self.partition);
        }
        if version >= 1 {
            put_i32(buf, self.leader_epoch);
        }
        {
            put_i64(buf, self.end_offset);
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = EpochEndOffset::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.partition = get_i32(buf)?;
        }
        if version >= 1 {
            msg.leader_epoch = get_i32(buf)?;
        }
        {
            msg.end_offset = get_i64(buf)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 2-4.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic we fetched offsets for.
    pub topics: Vec<OffsetForLeaderTopicResult>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetForLeaderEpochResponse {
    pub const API_KEY: i16 = 23;
    pub const VALID_MIN_VERSION: i16 = 2;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 2 {
            size += 4;
        }
        {
            { let arr = &self.topics;
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
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 2 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.topics;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = OffsetForLeaderEpochResponse::default();
        if version >= 2 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetForLeaderTopicResult::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetForLeaderEpochResponse {
    const API_KEY: i16 = 23;
    const VALID_MIN_VERSION: i16 = 2;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetForLeaderEpochResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
