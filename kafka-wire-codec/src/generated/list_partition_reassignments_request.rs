#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListPartitionReassignmentsTopics {
    /// The topic name.
    pub name: TopicName,
    /// The partitions to list partition reassignments for.
    pub partition_indexes: Vec<i32>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ListPartitionReassignmentsTopics {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
        }
        {
            { let arr = &self.partition_indexes;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.name.as_str());
        }
        {
            { let arr = &self.partition_indexes;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, *item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ListPartitionReassignmentsTopics::default();
        {
            msg.name = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partition_indexes = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq)]
pub struct ListPartitionReassignmentsRequest {
    /// The time in ms to wait for the request to complete.
    pub timeout_ms: i32,
    /// The topics to list partition reassignments for, or null to list everything.
    pub topics: Option<Vec<ListPartitionReassignmentsTopics>>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ListPartitionReassignmentsRequest {
    fn default() -> Self {
        Self {
            timeout_ms: 60000,
            topics: None,
            tagged_fields: Vec::new(),
        }
    }
}

impl ListPartitionReassignmentsRequest {
    pub const API_KEY: i16 = 46;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            match &self.topics {
                Some(arr) => {
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    size += 1;
                }
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i32(buf, self.timeout_ms);
        }
        {
            match &self.topics {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ListPartitionReassignmentsRequest::default();
        {
            msg.timeout_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.topics = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ListPartitionReassignmentsTopics::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for ListPartitionReassignmentsRequest {
    const API_KEY: i16 = 46;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ListPartitionReassignmentsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
