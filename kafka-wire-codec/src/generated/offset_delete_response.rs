#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct OffsetDeleteResponseTopic {
    /// The topic name.
    pub name: TopicName,
    /// The responses for each partition in the topic.
    pub partitions: Vec<OffsetDeleteResponsePartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteResponseTopic {
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
        let mut msg = OffsetDeleteResponseTopic::default();
        {
            msg.name = TopicName((get_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetDeleteResponsePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OffsetDeleteResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteResponsePartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i16(buf, self.error_code);
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetDeleteResponsePartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, Default)]
pub struct OffsetDeleteResponse {
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each topic.
    pub topics: Vec<OffsetDeleteResponseTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetDeleteResponse {
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
            size += 2;
        }
        {
            size += 4;
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
            put_i16(buf, self.error_code);
        }
        {
            put_i32(buf, self.throttle_time_ms);
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
        let mut msg = OffsetDeleteResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetDeleteResponseTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetDeleteResponse {
    const API_KEY: i16 = 47;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetDeleteResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
