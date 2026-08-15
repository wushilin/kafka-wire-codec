#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    pub topic: TopicName,
    /// Each partition to get offsets for.
    pub partitions: Vec<OffsetForLeaderPartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetForLeaderTopic {
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
        let mut msg = OffsetForLeaderTopic::default();
        {
            msg.topic = TopicName((if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetForLeaderPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition: i32,
    /// An epoch used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    pub current_leader_epoch: i32,
    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        Self {
            partition: 0,
            current_leader_epoch: -1,
            leader_epoch: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetForLeaderPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 2 {
            size += 4;
        }
        {
            size += 4;
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition);
        }
        if version >= 2 {
            put_i32(buf, self.current_leader_epoch);
        }
        {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetForLeaderPartition::default();
        {
            msg.partition = get_i32(buf)?;
        }
        if version >= 2 {
            msg.current_leader_epoch = get_i32(buf)?;
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 2-4.
#[derive(Debug, Clone, PartialEq)]
pub struct OffsetForLeaderEpochRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: BrokerId,
    /// Each topic to get offsets for.
    pub topics: Vec<OffsetForLeaderTopic>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        Self {
            replica_id: BrokerId(-2),
            topics: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetForLeaderEpochRequest {
    pub const API_KEY: i16 = 23;
    pub const VALID_MIN_VERSION: i16 = 2;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 3 {
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
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 3 {
            put_i32(buf, self.replica_id.0);
        }
        {
            { let arr = &self.topics;
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
        let mut msg = OffsetForLeaderEpochRequest::default();
        if version >= 3 {
            msg.replica_id = BrokerId(get_i32(buf)?);
        }
        {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetForLeaderTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetForLeaderEpochRequest {
    const API_KEY: i16 = 23;
    const VALID_MIN_VERSION: i16 = 2;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetForLeaderEpochRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
