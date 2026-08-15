#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetDeleteRequestTopic {
    /// The topic name.
    pub name: TopicName,
    /// Each partition to delete offsets for.
    pub partitions: Vec<OffsetDeleteRequestPartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += string_size(self.name.as_str());
        }
        {
            { let arr = &self.partitions;
                size += 4;
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_string(buf, self.name.as_str());
        }
        {
            { let arr = &self.partitions;
                put_i32(buf, arr.len() as i32);
                for item in arr { item.encode(version, buf); }
            }
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetDeleteRequestTopic::default();
        {
            msg.name = TopicName((get_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetDeleteRequestPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetDeleteRequestPartition {
    /// The partition index.
    pub partition_index: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteRequestPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetDeleteRequestPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OffsetDeleteRequest {
    /// The unique group identifier.
    pub group_id: GroupId,
    /// The topics to delete offsets for.
    pub topics: Vec<OffsetDeleteRequestTopic>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteRequest {
    pub const API_KEY: i16 = 47;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += string_size(self.group_id.as_str());
        }
        {
            { let arr = &self.topics;
                size += 4;
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_string(buf, self.group_id.as_str());
        }
        {
            { let arr = &self.topics;
                put_i32(buf, arr.len() as i32);
                for item in arr { item.encode(version, buf); }
            }
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = OffsetDeleteRequest::default();
        {
            msg.group_id = GroupId((get_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetDeleteRequestTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetDeleteRequest {
    const API_KEY: i16 = 47;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetDeleteRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
