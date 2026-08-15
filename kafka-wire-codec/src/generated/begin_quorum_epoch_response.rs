#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicData {
    /// The topic name.
    pub topic_name: TopicName,
    /// The partition data.
    pub partitions: Vec<PartitionData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 1 { compact_string_size(self.topic_name.as_str()) } else { string_size(self.topic_name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 1 { put_compact_string(buf, self.topic_name.as_str()) } else { put_string(buf, self.topic_name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TopicData::default();
        {
            msg.topic_name = TopicName((if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(PartitionData::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The error code for this partition.
    pub error_code: i16,
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: BrokerId,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl PartitionData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i32(buf, self.leader_id.0);
        }
        {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionData::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeEndpoint {
    /// The ID of the associated node.
    pub node_id: BrokerId,
    /// The node's hostname.
    pub host: StrBytes,
    /// The node's port.
    pub port: u16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl NodeEndpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        if version >= 1 {
            size += if version >= 1 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        if version >= 1 {
            size += 2;
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 1 {
            put_i32(buf, self.node_id.0);
        }
        if version >= 1 {
            if version >= 1 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        if version >= 1 {
            put_u16(buf, self.port);
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 1 {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        if version >= 1 {
            msg.host = (if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.port = get_u16(buf)?;
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-1.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BeginQuorumEpochResponse {
    /// The top level error code.
    pub error_code: i16,
    /// The topic data.
    pub topics: Vec<TopicData>,
    /// Endpoints for all leaders enumerated in PartitionData.
    /// Tagged field (tag 0, versions 1+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl BeginQuorumEpochResponse {
    pub const API_KEY: i16 = 53;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 1;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            { let arr = &self.topics;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.topics;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 1 && (!self.node_endpoints.is_empty()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.node_endpoints;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        } }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = BeginQuorumEpochResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicData::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 1 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 1 => {
                        let buf = &mut data;
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(NodeEndpoint::decode(version, buf)?); }
            msg.node_endpoints = items; }
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        } }
        Ok(msg)
    }
}

impl crate::Encodable for BeginQuorumEpochResponse {
    const API_KEY: i16 = 53;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 1;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for BeginQuorumEpochResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
