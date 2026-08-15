#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicData {
    /// The topic name.
    pub topic_name: TopicName,
    /// The partitions.
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
    /// The current leader ID that is resigning.
    pub leader_id: BrokerId,
    /// The current epoch.
    pub leader_epoch: i32,
    /// A sorted list of preferred successors to start the election.
    pub preferred_successors: Vec<i32>,
    /// A sorted list of preferred candidates to start the election.
    pub preferred_candidates: Vec<ReplicaInfo>,
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
            size += 4;
        }
        {
            size += 4;
        }
        if version <= 0 {
            { let arr = &self.preferred_successors;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 1 {
            { let arr = &self.preferred_candidates;
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
            put_i32(buf, self.partition_index);
        }
        {
            put_i32(buf, self.leader_id.0);
        }
        {
            put_i32(buf, self.leader_epoch);
        }
        if version <= 0 {
            { let arr = &self.preferred_successors;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 1 {
            { let arr = &self.preferred_candidates;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionData::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version <= 0 {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.preferred_successors = items; }
        }
        if version >= 1 {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ReplicaInfo::decode(version, buf)?); }
            msg.preferred_candidates = items; }
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReplicaInfo {
    /// The ID of the candidate replica.
    pub candidate_id: BrokerId,
    /// The directory ID of the candidate replica.
    pub candidate_directory_id: Uuid,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ReplicaInfo {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        if version >= 1 {
            size += 16;
        }
        if version >= 1 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 1 {
            put_i32(buf, self.candidate_id.0);
        }
        if version >= 1 {
            put_uuid(buf, &self.candidate_directory_id);
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ReplicaInfo::default();
        if version >= 1 {
            msg.candidate_id = BrokerId(get_i32(buf)?);
        }
        if version >= 1 {
            msg.candidate_directory_id = get_uuid(buf)?;
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LeaderEndpoint {
    /// The name of the endpoint.
    pub name: StrBytes,
    /// The node's hostname.
    pub host: StrBytes,
    /// The node's port.
    pub port: u16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl LeaderEndpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 1 {
            size += if version >= 1 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
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
            if version >= 1 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
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
        let mut msg = LeaderEndpoint::default();
        if version >= 1 {
            msg.name = (if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
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
pub struct EndQuorumEpochRequest {
    /// The cluster id.
    pub cluster_id: Option<StrBytes>,
    /// The topics.
    pub topics: Vec<TopicData>,
    /// Endpoints for the leader.
    pub leader_endpoints: Vec<LeaderEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl EndQuorumEpochRequest {
    pub const API_KEY: i16 = 54;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 1;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 1 { compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) };
        }
        {
            { let arr = &self.topics;
                if version >= 1 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 {
            { let arr = &self.leader_endpoints;
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
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 1 { put_compact_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) };
        }
        {
            { let arr = &self.topics;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 {
            { let arr = &self.leader_endpoints;
                if version >= 1 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = EndQuorumEpochRequest::default();
        {
            msg.cluster_id = if version >= 1 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicData::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 1 {
            let len_opt = if version >= 1 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(LeaderEndpoint::decode(version, buf)?); }
            msg.leader_endpoints = items; }
        }
        if version >= 1 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for EndQuorumEpochRequest {
    const API_KEY: i16 = 54;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 1;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for EndQuorumEpochRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
