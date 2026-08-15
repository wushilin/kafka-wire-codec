#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone)]
pub struct ReplicaState {
    /// The ID of the replica.
    pub replica_id: i32,
    /// The replica directory ID of the replica.
    pub replica_directory_id: [u8; 16],
    /// The last known log end offset of the follower or -1 if it is unknown.
    pub log_end_offset: i64,
    /// The last known leader wall clock time time when a follower fetched from the leader. This is reported as -1 both for the current leader or if it is unknown for a voter.
    pub last_fetch_timestamp: i64,
    /// The leader wall clock append time of the offset for which the follower made the most recent fetch request. This is reported as the current time for the leader and -1 if unknown for a voter.
    pub last_caught_up_timestamp: i64,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ReplicaState {
    fn default() -> Self {
        Self {
            replica_id: 0,
            replica_directory_id: [0u8; 16],
            log_end_offset: 0,
            last_fetch_timestamp: -1,
            last_caught_up_timestamp: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl ReplicaState {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 2 {
            size += 16;
        }
        {
            size += 8;
        }
        if version >= 1 {
            size += 8;
        }
        if version >= 1 {
            size += 8;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.replica_id);
        }
        if version >= 2 {
            put_uuid(buf, &self.replica_directory_id);
        }
        {
            put_i64(buf, self.log_end_offset);
        }
        if version >= 1 {
            put_i64(buf, self.last_fetch_timestamp);
        }
        if version >= 1 {
            put_i64(buf, self.last_caught_up_timestamp);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ReplicaState::default();
        {
            msg.replica_id = get_i32(buf)?;
        }
        if version >= 2 {
            msg.replica_directory_id = get_uuid(buf)?;
        }
        {
            msg.log_end_offset = get_i64(buf)?;
        }
        if version >= 1 {
            msg.last_fetch_timestamp = get_i64(buf)?;
        }
        if version >= 1 {
            msg.last_caught_up_timestamp = get_i64(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TopicData {
    /// The topic name.
    pub topic_name: Bytes,
    /// The partition data.
    pub partitions: Vec<PartitionData>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(&self.topic_name);
        }
        {
            { let arr = &self.partitions;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, &self.topic_name);
        }
        {
            { let arr = &self.partitions;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TopicData::default();
        {
            msg.topic_name = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(PartitionData::decode(version, buf)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The partition error code.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<Bytes>,
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// The high water mark.
    pub high_watermark: i64,
    /// The current voters of the partition.
    pub current_voters: Vec<ReplicaState>,
    /// The observers of the partition.
    pub observers: Vec<ReplicaState>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            error_message: Some(Bytes::new()),
            leader_id: 0,
            leader_epoch: 0,
            high_watermark: 0,
            current_voters: Vec::new(),
            observers: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
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
        if version >= 2 {
            size += if version >= 2 { compact_nullable_string_size(self.error_message.as_deref()) } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); compact_string_size(v) };
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += 8;
        }
        {
            { let arr = &self.current_voters;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.observers;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i16(buf, self.error_code);
        }
        if version >= 2 {
            if version >= 2 { put_compact_nullable_string(buf, self.error_message.as_deref()) } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); put_compact_string(buf, v) };
        }
        {
            put_i32(buf, self.leader_id);
        }
        {
            put_i32(buf, self.leader_epoch);
        }
        {
            put_i64(buf, self.high_watermark);
        }
        {
            { let arr = &self.current_voters;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.observers;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionData::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 2 {
            msg.error_message = { let v = get_compact_string(buf)?; if version >= 2 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.leader_id = get_i32(buf)?;
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        {
            msg.high_watermark = get_i64(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ReplicaState::decode(version, buf)?); }
            msg.current_voters = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ReplicaState::decode(version, buf)?); }
            msg.observers = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Node {
    /// The ID of the associated node.
    pub node_id: i32,
    /// The listeners of this controller.
    pub listeners: Vec<Listener>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Node {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 2 {
            size += 4;
        }
        if version >= 2 {
            { let arr = &self.listeners;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 2 {
            put_i32(buf, self.node_id);
        }
        if version >= 2 {
            { let arr = &self.listeners;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Node::default();
        if version >= 2 {
            msg.node_id = get_i32(buf)?;
        }
        if version >= 2 {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Listener::decode(version, buf)?); }
            msg.listeners = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Listener {
    /// The name of the endpoint.
    pub name: Bytes,
    /// The hostname.
    pub host: Bytes,
    /// The port.
    pub port: u16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Listener {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 2 {
            size += compact_string_size(&self.name);
        }
        if version >= 2 {
            size += compact_string_size(&self.host);
        }
        if version >= 2 {
            size += 2;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 2 {
            put_compact_string(buf, &self.name);
        }
        if version >= 2 {
            put_compact_string(buf, &self.host);
        }
        if version >= 2 {
            put_u16(buf, self.port);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Listener::default();
        if version >= 2 {
            msg.name = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 2 {
            msg.host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 2 {
            msg.port = get_u16(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-2.
#[derive(Debug, Clone)]
pub struct DescribeQuorumResponse {
    /// The top level error code.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<Bytes>,
    /// The response from the describe quorum API.
    pub topics: Vec<TopicData>,
    /// The nodes in the quorum.
    pub nodes: Vec<Node>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeQuorumResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(Bytes::new()),
            topics: Vec::new(),
            nodes: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeQuorumResponse {
    pub const API_KEY: i16 = 55;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 2;
        }
        if version >= 2 {
            size += if version >= 2 { compact_nullable_string_size(self.error_message.as_deref()) } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); compact_string_size(v) };
        }
        {
            { let arr = &self.topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 {
            { let arr = &self.nodes;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i16(buf, self.error_code);
        }
        if version >= 2 {
            if version >= 2 { put_compact_nullable_string(buf, self.error_message.as_deref()) } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); put_compact_string(buf, v) };
        }
        {
            { let arr = &self.topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 {
            { let arr = &self.nodes;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeQuorumResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 2 {
            msg.error_message = { let v = get_compact_string(buf)?; if version >= 2 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicData::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 2 {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Node::decode(version, buf)?); }
            msg.nodes = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeQuorumResponse {
    const API_KEY: i16 = 55;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeQuorumResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
