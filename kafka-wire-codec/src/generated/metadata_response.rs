#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetadataResponseBroker {
    /// The broker ID.
    pub node_id: BrokerId,
    /// The broker hostname.
    pub host: StrBytes,
    /// The broker port.
    pub port: i32,
    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl MetadataResponseBroker {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += if version >= 9 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        {
            size += 4;
        }
        if version >= 1 {
            size += if version >= 1 { if version >= 9 { compact_nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.node_id.0);
        }
        {
            if version >= 9 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        {
            put_i32(buf, self.port);
        }
        if version >= 1 {
            if version >= 1 { if version >= 9 { put_compact_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MetadataResponseBroker::default();
        {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.host = (if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.port = get_i32(buf)?;
        }
        if version >= 1 {
            msg.rack = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 1 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name. Null for non-existing topics queried by ID. This is never null when ErrorCode is zero. One of Name and TopicId is always populated.
    pub name: Option<TopicName>,
    /// The topic id. Zero for non-existing topics queried by name. This is never zero when ErrorCode is zero. One of Name and TopicId is always populated.
    pub topic_id: Uuid,
    /// True if the topic is internal.
    pub is_internal: bool,
    /// Each partition in the topic.
    pub partitions: Vec<MetadataResponsePartition>,
    /// 32-bit bitfield to represent authorized operations for this topic.
    pub topic_authorized_operations: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponseTopic {
    fn default() -> Self {
        Self {
            error_code: 0,
            name: Some(TopicName::default()),
            topic_id: Uuid::nil(),
            is_internal: false,
            partitions: Vec::new(),
            topic_authorized_operations: -2147483648,
            tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponseTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += if version >= 12 { if version >= 9 { compact_nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 10 {
            size += 16;
        }
        if version >= 1 {
            size += 1;
        }
        {
            { let arr = &self.partitions;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 {
            size += 4;
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 12 { if version >= 9 { put_compact_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        if version >= 1 {
            put_bool(buf, self.is_internal);
        }
        {
            { let arr = &self.partitions;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 {
            put_i32(buf, self.topic_authorized_operations);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MetadataResponseTopic::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.name = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 12 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } }.map(TopicName);
        }
        if version >= 10 {
            msg.topic_id = get_uuid(buf)?;
        }
        if version >= 1 {
            msg.is_internal = get_bool(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(MetadataResponsePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 8 {
            msg.topic_authorized_operations = get_i32(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition_index: i32,
    /// The ID of the leader broker.
    pub leader_id: BrokerId,
    /// The leader epoch of this partition.
    pub leader_epoch: i32,
    /// The set of all nodes that host this partition.
    pub replica_nodes: Vec<BrokerId>,
    /// The set of nodes that are in sync with the leader for this partition.
    pub isr_nodes: Vec<BrokerId>,
    /// The set of offline replicas of this partition.
    pub offline_replicas: Vec<BrokerId>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_index: 0,
            leader_id: BrokerId::default(),
            leader_epoch: -1,
            replica_nodes: Vec::new(),
            isr_nodes: Vec::new(),
            offline_replicas: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponsePartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        if version >= 7 {
            size += 4;
        }
        {
            { let arr = &self.replica_nodes;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        {
            { let arr = &self.isr_nodes;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 5 {
            { let arr = &self.offline_replicas;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i32(buf, self.leader_id.0);
        }
        if version >= 7 {
            put_i32(buf, self.leader_epoch);
        }
        {
            { let arr = &self.replica_nodes;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, item.0); }
            }
        }
        {
            { let arr = &self.isr_nodes;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, item.0); }
            }
        }
        if version >= 5 {
            { let arr = &self.offline_replicas;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, item.0); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MetadataResponsePartition::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        if version >= 7 {
            msg.leader_epoch = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.replica_nodes = items; }
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.isr_nodes = items; }
        }
        if version >= 5 {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.offline_replicas = items; }
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-13.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// A list of brokers present in the cluster.
    pub brokers: Vec<MetadataResponseBroker>,
    /// The cluster ID that responding broker belongs to.
    pub cluster_id: Option<StrBytes>,
    /// The ID of the controller broker.
    pub controller_id: BrokerId,
    /// Each topic in the response.
    pub topics: Vec<MetadataResponseTopic>,
    /// 32-bit bitfield to represent authorized operations for this cluster.
    pub cluster_authorized_operations: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            brokers: Vec::new(),
            cluster_id: None,
            controller_id: BrokerId(-1),
            topics: Vec::new(),
            cluster_authorized_operations: -2147483648,
            error_code: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponse {
    pub const API_KEY: i16 = 3;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 13;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 9;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 3 {
            size += 4;
        }
        {
            { let arr = &self.brokers;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 {
            size += if version >= 2 { if version >= 9 { compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } } else { let v = self.cluster_id.as_ref().expect("field cluster_id is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 1 {
            size += 4;
        }
        {
            { let arr = &self.topics;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 && version <= 10 {
            size += 4;
        }
        if version >= 13 {
            size += 2;
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 3 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.brokers;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 {
            if version >= 2 { if version >= 9 { put_compact_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) } } else { let v = self.cluster_id.as_ref().expect("field cluster_id is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 1 {
            put_i32(buf, self.controller_id.0);
        }
        {
            { let arr = &self.topics;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 && version <= 10 {
            put_i32(buf, self.cluster_authorized_operations);
        }
        if version >= 13 {
            put_i16(buf, self.error_code);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = MetadataResponse::default();
        if version >= 3 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(MetadataResponseBroker::decode(version, buf)?); }
            msg.brokers = items; }
        }
        if version >= 2 {
            msg.cluster_id = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 2 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 1 {
            msg.controller_id = BrokerId(get_i32(buf)?);
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(MetadataResponseTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 8 && version <= 10 {
            msg.cluster_authorized_operations = get_i32(buf)?;
        }
        if version >= 13 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for MetadataResponse {
    const API_KEY: i16 = 3;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 13;
    const FLEXIBLE_MIN_VERSION: i16 = 9;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for MetadataResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
