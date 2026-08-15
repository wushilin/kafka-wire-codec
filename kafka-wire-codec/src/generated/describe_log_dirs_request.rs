#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct DescribableLogDirTopic {
    /// The topic name.
    pub topic: TopicName,
    /// The partition indexes.
    pub partitions: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribableLogDirTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.topic.as_str()) } else { string_size(self.topic.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.topic.as_str()) } else { put_string(buf, self.topic.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribableLogDirTopic::default();
        {
            msg.topic = TopicName((if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partitions = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-5.
#[derive(Debug, Clone)]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    pub topics: Option<Vec<DescribableLogDirTopic>>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeLogDirsRequest {
    fn default() -> Self {
        Self {
            topics: Some(Vec::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsRequest {
    pub const API_KEY: i16 = 35;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            match &self.topics {
                Some(arr) => {
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    if version >= 2 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            match &self.topics {
                Some(arr) => {
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    if version >= 2 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeLogDirsRequest::default();
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.topics = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribableLogDirTopic::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeLogDirsRequest {
    const API_KEY: i16 = 35;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeLogDirsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
