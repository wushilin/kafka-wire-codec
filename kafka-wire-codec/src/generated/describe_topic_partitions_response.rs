#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DescribeTopicPartitionsResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name.
    pub name: Option<TopicName>,
    /// The topic id.
    pub topic_id: Uuid,
    /// True if the topic is internal.
    pub is_internal: bool,
    /// Each partition in the topic.
    pub partitions: Vec<DescribeTopicPartitionsResponsePartition>,
    /// 32-bit bitfield to represent authorized operations for this topic.
    pub topic_authorized_operations: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeTopicPartitionsResponseTopic {
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

impl DescribeTopicPartitionsResponseTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += compact_nullable_string_size(self.name.as_ref().map(|v| v.as_str()));
        }
        {
            size += 16;
        }
        {
            size += 1;
        }
        {
            { let arr = &self.partitions;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 4;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            put_compact_nullable_string(buf, self.name.as_ref().map(|v| v.as_str()));
        }
        {
            put_uuid(buf, &self.topic_id);
        }
        {
            put_bool(buf, self.is_internal);
        }
        {
            { let arr = &self.partitions;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_i32(buf, self.topic_authorized_operations);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeTopicPartitionsResponseTopic::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.name = (get_compact_string(buf)?).map(TopicName);
        }
        {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            msg.is_internal = get_bool(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeTopicPartitionsResponsePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        {
            msg.topic_authorized_operations = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DescribeTopicPartitionsResponsePartition {
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
    /// The new eligible leader replicas otherwise.
    pub eligible_leader_replicas: Option<Vec<BrokerId>>,
    /// The last known ELR.
    pub last_known_elr: Option<Vec<BrokerId>>,
    /// The set of offline replicas of this partition.
    pub offline_replicas: Vec<BrokerId>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeTopicPartitionsResponsePartition {
    fn default() -> Self {
        Self {
            error_code: 0,
            partition_index: 0,
            leader_id: BrokerId::default(),
            leader_epoch: -1,
            replica_nodes: Vec::new(),
            isr_nodes: Vec::new(),
            eligible_leader_replicas: None,
            last_known_elr: None,
            offline_replicas: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTopicPartitionsResponsePartition {
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
        {
            size += 4;
        }
        {
            { let arr = &self.replica_nodes;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
            }
        }
        {
            { let arr = &self.isr_nodes;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
            }
        }
        {
            match &self.eligible_leader_replicas {
                Some(arr) => {
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
                }
                None => {
                    size += 1;
                }
            }
        }
        {
            match &self.last_known_elr {
                Some(arr) => {
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
                }
                None => {
                    size += 1;
                }
            }
        }
        {
            { let arr = &self.offline_replicas;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
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
        {
            put_i32(buf, self.leader_epoch);
        }
        {
            { let arr = &self.replica_nodes;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, item.0); }
            }
        }
        {
            { let arr = &self.isr_nodes;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, item.0); }
            }
        }
        {
            match &self.eligible_leader_replicas {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, item.0); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            match &self.last_known_elr {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, item.0); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            { let arr = &self.offline_replicas;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, item.0); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeTopicPartitionsResponsePartition::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.replica_nodes = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.isr_nodes = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.eligible_leader_replicas = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.last_known_elr = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i32(buf)).map(BrokerId)?); }
            msg.offline_replicas = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cursor {
    /// The name for the first topic to process.
    pub topic_name: TopicName,
    /// The partition index to start with.
    pub partition_index: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Cursor {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.topic_name.as_str());
        }
        {
            size += 4;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.topic_name.as_str());
        }
        {
            put_i32(buf, self.partition_index);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Cursor::default();
        {
            msg.topic_name = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.partition_index = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeTopicPartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic in the response.
    pub topics: Vec<DescribeTopicPartitionsResponseTopic>,
    /// The next topic and partition index to fetch details for.
    pub next_cursor: Option<Cursor>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeTopicPartitionsResponse {
    pub const API_KEY: i16 = 75;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 1 + self.next_cursor.as_ref().map_or(0, |v| v.encoded_size(version));
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            match &self.next_cursor { Some(v) => { put_i8(buf, 1); v.encode(version, buf); }, None => put_i8(buf, -1) };
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeTopicPartitionsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeTopicPartitionsResponseTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        {
            msg.next_cursor = if get_i8(buf)? < 0 { None } else { Some(Cursor::decode(version, buf)?) };
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeTopicPartitionsResponse {
    const API_KEY: i16 = 75;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeTopicPartitionsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
