#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct MetadataResponseBroker {
    /// The broker ID.
    pub node_id: i32,
    /// The broker hostname.
    pub host: Bytes,
    /// The broker port.
    pub port: i32,
    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl MetadataResponseBroker {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += if version >= 9 { compact_string_size(&self.host) } else { string_size(&self.host) };
        }
        {
            size += 4;
        }
        if version >= 1 {
            size += if version >= 1 { if version >= 9 { compact_nullable_string_size(self.rack.as_deref()) } else { nullable_string_size(self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.node_id);
        }
        {
            if version >= 9 { put_compact_string(buf, &self.host) } else { put_string(buf, &self.host) };
        }
        {
            put_i32(buf, self.port);
        }
        if version >= 1 {
            if version >= 1 { if version >= 9 { put_compact_nullable_string(buf, self.rack.as_deref()) } else { put_nullable_string(buf, self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MetadataResponseBroker::default();
        {
            msg.node_id = get_i32(buf)?;
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

#[derive(Debug, Clone)]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name. Null for non-existing topics queried by ID. This is never null when ErrorCode is zero. One of Name and TopicId is always populated.
    pub name: Option<Bytes>,
    /// The topic id. Zero for non-existing topics queried by name. This is never zero when ErrorCode is zero. One of Name and TopicId is always populated.
    pub topic_id: [u8; 16],
    /// True if the topic is internal.
    pub is_internal: bool,
    /// Each partition in the topic.
    pub partitions: Vec<MetadataResponsePartition>,
    /// 32-bit bitfield to represent authorized operations for this topic.
    pub topic_authorized_operations: i32,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponseTopic {
    fn default() -> Self {
        Self {
            error_code: 0,
            name: Some(Bytes::new()),
            topic_id: [0u8; 16],
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
            size += if version >= 12 { if version >= 9 { compact_nullable_string_size(self.name.as_deref()) } else { nullable_string_size(self.name.as_deref()) } } else { let v = self.name.as_deref().expect("field name is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
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
            if version >= 12 { if version >= 9 { put_compact_nullable_string(buf, self.name.as_deref()) } else { put_nullable_string(buf, self.name.as_deref()) } } else { let v = self.name.as_deref().expect("field name is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
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
            msg.name = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 12 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
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

#[derive(Debug, Clone)]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition_index: i32,
    /// The ID of the leader broker.
    pub leader_id: i32,
    /// The leader epoch of this partition.
    pub leader_epoch: i32,
    /// The set of all nodes that host this partition.
    pub replica_nodes: Vec<i32>,
    /// The set of nodes that are in sync with the leader for this partition.
    pub isr_nodes: Vec<i32>,
    /// The set of offline replicas of this partition.
    pub offline_replicas: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_index: 0,
            leader_id: 0,
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
            put_i32(buf, self.leader_id);
        }
        if version >= 7 {
            put_i32(buf, self.leader_epoch);
        }
        {
            { let arr = &self.replica_nodes;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        {
            { let arr = &self.isr_nodes;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 5 {
            { let arr = &self.offline_replicas;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
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
            msg.leader_id = get_i32(buf)?;
        }
        if version >= 7 {
            msg.leader_epoch = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.replica_nodes = items; }
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.isr_nodes = items; }
        }
        if version >= 5 {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.offline_replicas = items; }
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-13.
#[derive(Debug, Clone)]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// A list of brokers present in the cluster.
    pub brokers: Vec<MetadataResponseBroker>,
    /// The cluster ID that responding broker belongs to.
    pub cluster_id: Option<Bytes>,
    /// The ID of the controller broker.
    pub controller_id: i32,
    /// Each topic in the response.
    pub topics: Vec<MetadataResponseTopic>,
    /// 32-bit bitfield to represent authorized operations for this cluster.
    pub cluster_authorized_operations: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            brokers: Vec::new(),
            cluster_id: None,
            controller_id: -1,
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

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
            size += if version >= 2 { if version >= 9 { compact_nullable_string_size(self.cluster_id.as_deref()) } else { nullable_string_size(self.cluster_id.as_deref()) } } else { let v = self.cluster_id.as_deref().expect("field cluster_id is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
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
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
            if version >= 2 { if version >= 9 { put_compact_nullable_string(buf, self.cluster_id.as_deref()) } else { put_nullable_string(buf, self.cluster_id.as_deref()) } } else { let v = self.cluster_id.as_deref().expect("field cluster_id is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 1 {
            put_i32(buf, self.controller_id);
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
            msg.controller_id = get_i32(buf)?;
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
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for MetadataResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
