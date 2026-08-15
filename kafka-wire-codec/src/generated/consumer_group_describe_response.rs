#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicPartitions {
    /// The topic ID.
    pub topic_id: Uuid,
    /// The topic name.
    pub topic_name: TopicName,
    /// The partitions.
    pub partitions: Vec<i32>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicPartitions {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 16;
        }
        {
            size += compact_string_size(self.topic_name.as_str());
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
            put_uuid(buf, &self.topic_id);
        }
        {
            put_compact_string(buf, self.topic_name.as_str());
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
        let mut msg = TopicPartitions::default();
        {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            msg.topic_name = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
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
pub struct Assignment {
    /// The assigned topic-partitions to the member.
    pub topic_partitions: Vec<TopicPartitions>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Assignment {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            { let arr = &self.topic_partitions;
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
            { let arr = &self.topic_partitions;
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
                for _ in 0..count { items.push(TopicPartitions::decode(version, buf)?); }
            msg.topic_partitions = items; }
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
    /// The selected assignor.
    pub assignor_name: StrBytes,
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
            assignor_name: StrBytes::new(),
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
            size += compact_string_size(self.assignor_name.as_str());
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
            put_compact_string(buf, self.assignor_name.as_str());
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
            msg.assignor_name = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The member ID.
    pub member_id: StrBytes,
    /// The member instance ID.
    pub instance_id: Option<StrBytes>,
    /// The member rack ID.
    pub rack_id: Option<StrBytes>,
    /// The current member epoch.
    pub member_epoch: i32,
    /// The client ID.
    pub client_id: StrBytes,
    /// The client host.
    pub client_host: StrBytes,
    /// The subscribed topic names.
    pub subscribed_topic_names: Vec<TopicName>,
    /// the subscribed topic regex otherwise or null of not provided.
    pub subscribed_topic_regex: Option<StrBytes>,
    /// The current assignment.
    pub assignment: Assignment,
    /// The target assignment.
    pub target_assignment: Assignment,
    /// -1 for unknown. 0 for classic member. +1 for consumer member.
    pub member_type: i8,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for Member {
    fn default() -> Self {
        Self {
            member_id: StrBytes::new(),
            instance_id: None,
            rack_id: None,
            member_epoch: 0,
            client_id: StrBytes::new(),
            client_host: StrBytes::new(),
            subscribed_topic_names: Vec::new(),
            subscribed_topic_regex: None,
            assignment: Assignment::default(),
            target_assignment: Assignment::default(),
            member_type: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl Member {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.member_id.as_str());
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
            size += compact_string_size(self.client_id.as_str());
        }
        {
            size += compact_string_size(self.client_host.as_str());
        }
        {
            { let arr = &self.subscribed_topic_names;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
                }
            }
        }
        {
            size += compact_nullable_string_size(self.subscribed_topic_regex.as_ref().map(|v| v.as_str()));
        }
        {
            size += self.assignment.encoded_size(version);
        }
        {
            size += self.target_assignment.encoded_size(version);
        }
        if version >= 1 {
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
            put_compact_nullable_string(buf, self.instance_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_nullable_string(buf, self.rack_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.member_epoch);
        }
        {
            put_compact_string(buf, self.client_id.as_str());
        }
        {
            put_compact_string(buf, self.client_host.as_str());
        }
        {
            { let arr = &self.subscribed_topic_names;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        {
            put_compact_nullable_string(buf, self.subscribed_topic_regex.as_ref().map(|v| v.as_str()));
        }
        {
            self.assignment.encode(version, buf);
        }
        {
            self.target_assignment.encode(version, buf);
        }
        if version >= 1 {
            put_i8(buf, self.member_type);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Member::default();
        {
            msg.member_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.instance_id = get_compact_string(buf)?;
        }
        {
            msg.rack_id = get_compact_string(buf)?;
        }
        {
            msg.member_epoch = get_i32(buf)?;
        }
        {
            msg.client_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.client_host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TopicName)?); }
            msg.subscribed_topic_names = items; }
        }
        {
            msg.subscribed_topic_regex = get_compact_string(buf)?;
        }
        {
            msg.assignment = Assignment::decode(version, buf)?;
        }
        {
            msg.target_assignment = Assignment::decode(version, buf)?;
        }
        if version >= 1 {
            msg.member_type = get_i8(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-1.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConsumerGroupDescribeResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    pub groups: Vec<DescribedGroup>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ConsumerGroupDescribeResponse {
    pub const API_KEY: i16 = 69;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
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
            { let arr = &self.groups;
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
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.groups;
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
        let mut msg = ConsumerGroupDescribeResponse::default();
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

impl crate::Encodable for ConsumerGroupDescribeResponse {
    const API_KEY: i16 = 69;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ConsumerGroupDescribeResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
