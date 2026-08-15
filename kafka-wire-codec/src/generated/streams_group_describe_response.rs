#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

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
pub struct Assignment {
    /// Active tasks for this client.
    pub active_tasks: Vec<TaskIds>,
    /// Standby tasks for this client.
    pub standby_tasks: Vec<TaskIds>,
    /// Warm-up tasks for this client.
    pub warmup_tasks: Vec<TaskIds>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Assignment {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            { let arr = &self.active_tasks;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.standby_tasks;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.warmup_tasks;
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
            { let arr = &self.active_tasks;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.standby_tasks;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.warmup_tasks;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Assignment::default();
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
            msg.active_tasks = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
            msg.standby_tasks = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskIds::decode(version, buf)?); }
            msg.warmup_tasks = items; }
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

#[derive(Debug, Clone, PartialEq)]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// The group ID string.
    pub group_id: GroupId,
    /// The group state string, or the empty string.
    pub group_state: StrBytes,
    /// The group epoch.
    pub group_epoch: i32,
    /// The assignment epoch.
    pub assignment_epoch: i32,
    /// The topology metadata currently initialized for the streams application. Can be null in case of a describe error.
    pub topology: Option<Topology>,
    /// The members.
    pub members: Vec<Member>,
    /// 32-bit bitfield to represent authorized operations for this group.
    pub authorized_operations: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribedGroup {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: None,
            group_id: GroupId::default(),
            group_state: StrBytes::new(),
            group_epoch: 0,
            assignment_epoch: 0,
            topology: None,
            members: Vec::new(),
            authorized_operations: -2147483648,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribedGroup {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            size += compact_string_size(self.group_id.as_str());
        }
        {
            size += compact_string_size(self.group_state.as_str());
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += 1 + self.topology.as_ref().map_or(0, |v| v.encoded_size(version));
        }
        {
            { let arr = &self.members;
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
            put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_string(buf, self.group_id.as_str());
        }
        {
            put_compact_string(buf, self.group_state.as_str());
        }
        {
            put_i32(buf, self.group_epoch);
        }
        {
            put_i32(buf, self.assignment_epoch);
        }
        {
            match &self.topology { Some(v) => { put_i8(buf, 1); v.encode(version, buf); }, None => put_i8(buf, -1) };
        }
        {
            { let arr = &self.members;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_i32(buf, self.authorized_operations);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribedGroup::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = get_compact_string(buf)?;
        }
        {
            msg.group_id = GroupId((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.group_state = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.group_epoch = get_i32(buf)?;
        }
        {
            msg.assignment_epoch = get_i32(buf)?;
        }
        {
            msg.topology = if get_i8(buf)? < 0 { None } else { Some(Topology::decode(version, buf)?) };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Member::decode(version, buf)?); }
            msg.members = items; }
        }
        {
            msg.authorized_operations = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Topology {
    /// The epoch of the currently initialized topology for this group.
    pub epoch: i32,
    /// The subtopologies of the streams application. This contains the configured subtopologies, where the number of partitions are set and any regular expressions are resolved to actual topics. Null if the group is uninitialized, source topics are missing or incorrectly partitioned.
    pub subtopologies: Option<Vec<Subtopology>>,
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
            match &self.subtopologies {
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
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.epoch);
        }
        {
            match &self.subtopologies {
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
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Topology::default();
        {
            msg.epoch = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.subtopologies = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Subtopology::decode(version, buf)?); }
                Some(items)
                }
                None => None,
            };
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subtopology {
    /// String to uniquely identify the subtopology.
    pub subtopology_id: StrBytes,
    /// The topics the subtopology reads from.
    pub source_topics: Vec<TopicName>,
    /// The repartition topics the subtopology writes to.
    pub repartition_sink_topics: Vec<TopicName>,
    /// The set of state changelog topics associated with this subtopology. Created automatically.
    pub state_changelog_topics: Vec<TopicInfo>,
    /// The set of source topics that are internally created repartition topics. Created automatically.
    pub repartition_source_topics: Vec<TopicInfo>,
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
            { let arr = &self.repartition_sink_topics;
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
            { let arr = &self.repartition_source_topics;
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
            { let arr = &self.repartition_sink_topics;
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
            { let arr = &self.repartition_source_topics;
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
                for _ in 0..count { items.push(((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TopicName)?); }
            msg.repartition_sink_topics = items; }
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
                for _ in 0..count { items.push(TopicInfo::decode(version, buf)?); }
            msg.repartition_source_topics = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Member {
    /// The member ID.
    pub member_id: StrBytes,
    /// The member epoch.
    pub member_epoch: i32,
    /// The member instance ID for static membership.
    pub instance_id: Option<StrBytes>,
    /// The rack ID.
    pub rack_id: Option<StrBytes>,
    /// The client ID.
    pub client_id: StrBytes,
    /// The client host.
    pub client_host: StrBytes,
    /// The epoch of the topology on the client.
    pub topology_epoch: i32,
    /// Identity of the streams instance that may have multiple clients.
    pub process_id: StrBytes,
    /// User-defined endpoint for Interactive Queries. Null if not defined for this client.
    pub user_endpoint: Option<Endpoint>,
    /// Used for rack-aware assignment algorithm.
    pub client_tags: Vec<KeyValue>,
    /// Cumulative changelog offsets for tasks.
    pub task_offsets: Vec<TaskOffset>,
    /// Cumulative changelog end offsets for tasks.
    pub task_end_offsets: Vec<TaskOffset>,
    /// The current assignment.
    pub assignment: Assignment,
    /// The target assignment.
    pub target_assignment: Assignment,
    /// True for classic members that have not been upgraded yet.
    pub is_classic: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Member {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.member_id.as_str());
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
            size += compact_string_size(self.client_id.as_str());
        }
        {
            size += compact_string_size(self.client_host.as_str());
        }
        {
            size += 4;
        }
        {
            size += compact_string_size(self.process_id.as_str());
        }
        {
            size += 1 + self.user_endpoint.as_ref().map_or(0, |v| v.encoded_size(version));
        }
        {
            { let arr = &self.client_tags;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.task_offsets;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.task_end_offsets;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += self.assignment.encoded_size(version);
        }
        {
            size += self.target_assignment.encoded_size(version);
        }
        {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.member_id.as_str());
        }
        {
            put_i32(buf, self.member_epoch);
        }
        {
            put_compact_nullable_string(buf, self.instance_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_nullable_string(buf, self.rack_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_string(buf, self.client_id.as_str());
        }
        {
            put_compact_string(buf, self.client_host.as_str());
        }
        {
            put_i32(buf, self.topology_epoch);
        }
        {
            put_compact_string(buf, self.process_id.as_str());
        }
        {
            match &self.user_endpoint { Some(v) => { put_i8(buf, 1); v.encode(version, buf); }, None => put_i8(buf, -1) };
        }
        {
            { let arr = &self.client_tags;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.task_offsets;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.task_end_offsets;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            self.assignment.encode(version, buf);
        }
        {
            self.target_assignment.encode(version, buf);
        }
        {
            put_bool(buf, self.is_classic);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Member::default();
        {
            msg.member_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.member_epoch = get_i32(buf)?;
        }
        {
            msg.instance_id = get_compact_string(buf)?;
        }
        {
            msg.rack_id = get_compact_string(buf)?;
        }
        {
            msg.client_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.client_host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.topology_epoch = get_i32(buf)?;
        }
        {
            msg.process_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.user_endpoint = if get_i8(buf)? < 0 { None } else { Some(Endpoint::decode(version, buf)?) };
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(KeyValue::decode(version, buf)?); }
            msg.client_tags = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskOffset::decode(version, buf)?); }
            msg.task_offsets = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TaskOffset::decode(version, buf)?); }
            msg.task_end_offsets = items; }
        }
        {
            msg.assignment = Assignment::decode(version, buf)?;
        }
        {
            msg.target_assignment = Assignment::decode(version, buf)?;
        }
        {
            msg.is_classic = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StreamsGroupDescribeResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    pub groups: Vec<DescribedGroup>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl StreamsGroupDescribeResponse {
    pub const API_KEY: i16 = 89;
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
            { let arr = &self.groups;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
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
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.groups;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = StreamsGroupDescribeResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribedGroup::decode(version, buf)?); }
            msg.groups = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for StreamsGroupDescribeResponse {
    const API_KEY: i16 = 89;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for StreamsGroupDescribeResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
