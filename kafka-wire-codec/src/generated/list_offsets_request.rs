#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListOffsetsTopic {
    /// The topic name.
    pub name: TopicName,
    /// Each partition in the request.
    pub partitions: Vec<ListOffsetsPartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ListOffsetsTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 6 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 6 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ListOffsetsTopic::default();
        {
            msg.name = TopicName((if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ListOffsetsPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListOffsetsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The current leader epoch.
    pub current_leader_epoch: i32,
    /// The current timestamp.
    pub timestamp: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ListOffsetsPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            current_leader_epoch: -1,
            timestamp: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl ListOffsetsPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 4 {
            size += 4;
        }
        {
            size += 8;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        if version >= 4 {
            put_i32(buf, self.current_leader_epoch);
        }
        {
            put_i64(buf, self.timestamp);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ListOffsetsPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        if version >= 4 {
            msg.current_leader_epoch = get_i32(buf)?;
        }
        {
            msg.timestamp = get_i64(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-11.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListOffsetsRequest {
    /// The broker ID of the requester, or -1 if this request is being made by a normal consumer.
    pub replica_id: BrokerId,
    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records.
    pub isolation_level: i8,
    /// Each topic in the request.
    pub topics: Vec<ListOffsetsTopic>,
    /// The timeout to await a response in milliseconds for requests that require reading from remote storage for topics enabled with tiered storage.
    pub timeout_ms: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ListOffsetsRequest {
    pub const API_KEY: i16 = 2;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 11;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 6;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 2 {
            size += 1;
        }
        {
            { let arr = &self.topics;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 10 {
            size += 4;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.replica_id.0);
        }
        if version >= 2 {
            put_i8(buf, self.isolation_level);
        }
        {
            { let arr = &self.topics;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 10 {
            put_i32(buf, self.timeout_ms);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ListOffsetsRequest::default();
        {
            msg.replica_id = BrokerId(get_i32(buf)?);
        }
        if version >= 2 {
            msg.isolation_level = get_i8(buf)?;
        }
        {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ListOffsetsTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 10 {
            msg.timeout_ms = get_i32(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for ListOffsetsRequest {
    const API_KEY: i16 = 2;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 11;
    const FLEXIBLE_MIN_VERSION: i16 = 6;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ListOffsetsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
