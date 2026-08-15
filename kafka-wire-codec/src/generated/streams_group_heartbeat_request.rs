#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyValue {
    /// key of the config
    pub key: StrBytes,
    /// value of the config
    pub value: StrBytes,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl KeyValue {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.key.as_str());
        }
        {
            size += compact_string_size(self.value.as_str());
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.key.as_str());
        }
        {
            put_compact_string(buf, self.value.as_str());
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = KeyValue::default();
        {
            msg.key = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.value = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicInfo {
    /// The name of the topic.
    pub name: TopicName,
    /// The number of partitions in the topic. Can be 0 if no specific number of partitions is enforced. Always 0 for changelog topics.
    pub partitions: i32,
    /// The replication factor of the topic. Can be 0 if the default replication factor should be used.
    pub replication_factor: i16,
    /// Topic-level configurations as key-value pairs.
    pub topic_configs: Vec<KeyValue>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicInfo {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
        }
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            { let arr = &self.topic_configs;
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
            put_compact_string(buf, self.name.as_str());
        }
        {
            put_i32(buf, self.partitions);
        }
        {
            put_i16(buf, self.replication_factor);
        }
        {
            { let arr = &self.topic_configs;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TopicInfo::default();
        {
            msg.name = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.partitions = get_i32(buf)?;
        }
        {
            msg.replication_factor = get_i16(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(KeyValue::decode(version, buf)?); }
            msg.topic_configs = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Endpoint {
    /// host of the endpoint
    pub host: StrBytes,
    /// port of the endpoint
    pub port: u16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Endpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.host.as_str());
        }
        {
            size += 2;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.host.as_str());
        }
        {
            put_u16(buf, self.port);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Endpoint::default();
        {
            msg.host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.port = get_u16(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskOffset {
    /// The subtopology identifier.
    pub subtopology_id: StrBytes,
    /// The partition.
    pub partition: i32,
    /// The offset.
    pub offset: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TaskOffset {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.subtopology_id.as_str());
        }
        {
            size += 4;
        }
        {
            size += 8;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.subtopology_id.as_str());
        }
        {
            put_i32(buf, self.partition);
        }
        {
            put_i64(buf, self.offset);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TaskOffset::default();
        {
            msg.subtopology_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.partition = get_i32(buf)?;
        }
        {
            msg.offset = get_i64(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TaskIds {
    /// The subtopology identifier.
    pub subtopology_id: StrBytes,
    /// The partitions of the input topics processed by this member.
    pub partitions: Vec<i32>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TaskIds {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.subtopology_id.as_str());
        }
        {
            { let arr = &self.partitions;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 4;
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.subtopology_id.as_str());
        }
        {
            { let arr = &self.partitions;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, *item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TaskIds::default();
        {
            msg.subtopology_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Topology {
    /// The epoch of the topology. Used to check if the topology corresponds to the topology initialized on the brokers.
    pub epoch: i32,
    /// The sub-topologies of the streams application.
    pub subtopologies: Vec<Subtopology>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Topology {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.subtopologies;
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
            put_i32(buf, self.epoch);
        }
        {
            { let arr = &self.subtopologies;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Topology::default();
        {
            msg.epoch = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Subtopology::decode(version, buf)?); }
            msg.subtopologies = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subtopology {
    /// String to uniquely identify the subtopology. Deterministically generated from the topology
    pub subtopology_id: StrBytes,
    /// The topics the topology reads from.
    pub source_topics: Vec<TopicName>,
    /// The regular expressions identifying topics the subtopology reads from.
    pub source_topic_regex: Vec<StrBytes>,
    /// The set of state changelog topics associated with this subtopology. Created automatically.
    pub state_changelog_topics: Vec<TopicInfo>,
    /// The repartition topics the subtopology writes to.
    pub repartition_sink_topics: Vec<TopicName>,
    /// The set of source topics that are internally created repartition topics. Created automatically.
    pub repartition_source_topics: Vec<TopicInfo>,
    /// A subset of source topics that must be copartitioned.
    pub copartition_groups: Vec<CopartitionGroup>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Subtopology {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.subtopology_id.as_str());
        }
        {
            { let arr = &self.source_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
                }
            }
        }
        {
            { let arr = &self.source_topic_regex;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
                }
            }
        }
        {
            { let arr = &self.state_changelog_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.repartition_sink_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
                }
            }
        }
        {
            { let arr = &self.repartition_source_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.copartition_groups;
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
            put_compact_string(buf, self.subtopology_id.as_str());
        }
        {
            { let arr = &self.source_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        {
            { let arr = &self.source_topic_regex;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        {
            { let arr = &self.state_changelog_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.repartition_sink_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        {
            { let arr = &self.repartition_source_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.copartition_groups;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Subtopology::default();
        {
            msg.subtopology_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TopicName)?); }
            msg.source_topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.source_topic_regex = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicInfo::decode(version, buf)?); }
            msg.state_changelog_topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TopicName)?); }
            msg.repartition_sink_topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicInfo::decode(version, buf)?); }
            msg.repartition_source_topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(CopartitionGroup::decode(version, buf)?); }
            msg.copartition_groups = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CopartitionGroup {
    /// The topics the topology reads from. Index into the array on the subtopology level.
    pub source_topics: Vec<i16>,
    /// Regular expressions identifying topics the subtopology reads from. Index into the array on the subtopology level.
    pub source_topic_regex: Vec<i16>,
    /// The set of source topics that are internally created repartition topics. Index into the array on the subtopology level.
    pub repartition_source_topics: Vec<i16>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CopartitionGroup {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            { let arr = &self.source_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 2;
            }
        }
        {
            { let arr = &self.source_topic_regex;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 2;
            }
        }
        {
            { let arr = &self.repartition_source_topics;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 2;
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            { let arr = &self.source_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i16(buf, *item); }
            }
        }
        {
            { let arr = &self.source_topic_regex;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i16(buf, *item); }
            }
        }
        {
            { let arr = &self.repartition_source_topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i16(buf, *item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = CopartitionGroup::default();
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i16(buf)?); }
            msg.source_topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i16(buf)?); }
            msg.source_topic_regex = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i16(buf)?); }
            msg.repartition_source_topics = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamsGroupHeartbeatRequest {
    /// The group identifier.
    pub group_id: GroupId,
    /// The member ID generated by the streams consumer. The member ID must be kept during the entire lifetime of the streams consumer process.
    pub member_id: StrBytes,
    /// The current member epoch; 0 to join the group; -1 to leave the group; -2 to indicate that the static member will rejoin.
    pub member_epoch: i32,
    /// The current endpoint epoch of this client, represents the latest endpoint epoch this client received
    pub endpoint_information_epoch: i32,
    /// null if not provided or if it didn't change since the last heartbeat; the instance ID for static membership otherwise.
    pub instance_id: Option<StrBytes>,
    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of the member otherwise.
    pub rack_id: Option<StrBytes>,
    /// -1 if it didn't change since the last heartbeat; the maximum time in milliseconds that the coordinator will wait on the member to revoke its tasks otherwise.
    pub rebalance_timeout_ms: i32,
    /// The topology metadata of the streams application. Used to initialize the topology of the group and to check if the topology corresponds to the topology initialized for the group. Only sent when memberEpoch = 0, must be non-empty. Null otherwise.
    pub topology: Option<Topology>,
    /// Currently owned active tasks for this client. Null if unchanged since last heartbeat.
    pub active_tasks: Option<Vec<TaskIds>>,
    /// Currently owned standby tasks for this client. Null if unchanged since last heartbeat.
    pub standby_tasks: Option<Vec<TaskIds>>,
    /// Currently owned warm-up tasks for this client. Null if unchanged since last heartbeat.
    pub warmup_tasks: Option<Vec<TaskIds>>,
    /// Identity of the streams instance that may have multiple consumers. Null if unchanged since last heartbeat.
    pub process_id: Option<StrBytes>,
    /// User-defined endpoint for Interactive Queries. Null if unchanged since last heartbeat, or if not defined on the client.
    pub user_endpoint: Option<Endpoint>,
    /// Used for rack-aware assignment algorithm. Null if unchanged since last heartbeat.
    pub client_tags: Option<Vec<KeyValue>>,
    /// Cumulative changelog offsets for tasks. Only updated when a warm-up task has caught up, and according to the task offset interval. Null if unchanged since last heartbeat.
    pub task_offsets: Option<Vec<TaskOffset>>,
    /// Cumulative changelog end-offsets for tasks. Only updated when a warm-up task has caught up, and according to the task offset interval. Null if unchanged since last heartbeat.
    pub task_end_offsets: Option<Vec<TaskOffset>>,
    /// Whether all Streams clients in the group should shut down.
    pub shutdown_application: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for StreamsGroupHeartbeatRequest {
    fn default() -> Self {
        Self {
            group_id: GroupId::default(),
            member_id: StrBytes::new(),
            member_epoch: 0,
            endpoint_information_epoch: 0,
            instance_id: None,
            rack_id: None,
            rebalance_timeout_ms: -1,
            topology: None,
            active_tasks: None,
            standby_tasks: None,
            warmup_tasks: None,
            process_id: None,
            user_endpoint: None,
            client_tags: None,
            task_offsets: None,
            task_end_offsets: None,
            shutdown_application: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl StreamsGroupHeartbeatRequest {
    pub const API_KEY: i16 = 88;
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
            size += compact_string_size(self.group_id.as_str());
        }
        {
            size += compact_string_size(self.member_id.as_str());
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += compact_nullable_string_size(self.instance_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += compact_nullable_string_size(self.rack_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += 4;
        }
        {
            size += 1 + self.topology.as_ref().map_or(0, |v| v.encoded_size(version));
        }
        {
            match &self.active_tasks {
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
        {
            match &self.standby_tasks {
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
        {
            match &self.warmup_tasks {
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
        {
            size += compact_nullable_string_size(self.process_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += 1 + self.user_endpoint.as_ref().map_or(0, |v| v.encoded_size(version));
        }
        {
            match &self.client_tags {
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
        {
            match &self.task_offsets {
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
        {
            match &self.task_end_offsets {
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
        {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_compact_string(buf, self.group_id.as_str());
        }
        {
            put_compact_string(buf, self.member_id.as_str());
        }
        {
            put_i32(buf, self.member_epoch);
        }
        {
            put_i32(buf, self.endpoint_information_epoch);
        }
        {
            put_compact_nullable_string(buf, self.instance_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_nullable_string(buf, self.rack_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.rebalance_timeout_ms);
        }
        {
            match &self.topology { Some(v) => { put_i8(buf, 1); v.encode(version, buf); }, None => put_i8(buf, -1) };
        }
        {
            match &self.active_tasks {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            match &self.standby_tasks {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            match &self.warmup_tasks {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            put_compact_nullable_string(buf, self.process_id.as_ref().map(|v| v.as_str()));
        }
        {
            match &self.user_endpoint { Some(v) => { put_i8(buf, 1); v.encode(version, buf); }, None => put_i8(buf, -1) };
        }
        {
            match &self.client_tags {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            match &self.task_offsets {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            match &self.task_end_offsets {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    put_uvarint(buf, 0);
                }
            }
        }
        {
            put_bool(buf, self.shutdown_application);
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = StreamsGroupHeartbeatRequest::default();
        {
            msg.group_id = GroupId((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.member_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.member_epoch = get_i32(buf)?;
        }
        {
            msg.endpoint_information_epoch = get_i32(buf)?;
        }
        {
            msg.instance_id = get_compact_string(buf)?;
        }
        {
            msg.rack_id = get_compact_string(buf)?;
        }
        {
            msg.rebalance_timeout_ms = get_i32(buf)?;
        }
        {
            msg.topology = if get_i8(buf)? < 0 { None } else { Some(Topology::decode(version, buf)?) };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.active_tasks = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.standby_tasks = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.warmup_tasks = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            msg.process_id = get_compact_string(buf)?;
        }
        {
            msg.user_endpoint = if get_i8(buf)? < 0 { None } else { Some(Endpoint::decode(version, buf)?) };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.client_tags = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(KeyValue::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.task_offsets = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskOffset::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.task_end_offsets = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskOffset::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        {
            msg.shutdown_application = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for StreamsGroupHeartbeatRequest {
    const API_KEY: i16 = 88;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for StreamsGroupHeartbeatRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
