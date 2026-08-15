#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct OffsetCommitRequestTopic {
    /// The topic name.
    pub name: TopicName,
    /// The topic ID.
    pub topic_id: Uuid,
    /// Each partition to commit offsets for.
    pub partitions: Vec<OffsetCommitRequestPartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetCommitRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 9 {
            size += if version >= 8 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 10 {
            size += 16;
        }
        {
            { let arr = &self.partitions;
                if version >= 8 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 9 {
            if version >= 8 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        {
            { let arr = &self.partitions;
                if version >= 8 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetCommitRequestTopic::default();
        if version <= 9 {
            msg.name = TopicName((if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 10 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = if version >= 8 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetCommitRequestPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct OffsetCommitRequestPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// The leader epoch of this partition.
    pub committed_leader_epoch: i32,
    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<StrBytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            committed_metadata: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetCommitRequestPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 8;
        }
        if version >= 6 {
            size += 4;
        }
        {
            size += if version >= 8 { compact_nullable_string_size(self.committed_metadata.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.committed_metadata.as_ref().map(|v| v.as_str())) };
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i64(buf, self.committed_offset);
        }
        if version >= 6 {
            put_i32(buf, self.committed_leader_epoch);
        }
        {
            if version >= 8 { put_compact_nullable_string(buf, self.committed_metadata.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.committed_metadata.as_ref().map(|v| v.as_str())) };
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetCommitRequestPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.committed_offset = get_i64(buf)?;
        }
        if version >= 6 {
            msg.committed_leader_epoch = get_i32(buf)?;
        }
        {
            msg.committed_metadata = if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 2-10.
#[derive(Debug, Clone)]
pub struct OffsetCommitRequest {
    /// The unique group identifier.
    pub group_id: GroupId,
    /// The generation of the group if using the classic group protocol or the member epoch if using the consumer protocol.
    pub generation_id_or_member_epoch: i32,
    /// The member ID assigned by the group coordinator.
    pub member_id: StrBytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<StrBytes>,
    /// The time period in ms to retain the offset.
    pub retention_time_ms: i64,
    /// The topics to commit offsets for.
    pub topics: Vec<OffsetCommitRequestTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetCommitRequest {
    fn default() -> Self {
        Self {
            group_id: GroupId::default(),
            generation_id_or_member_epoch: -1,
            member_id: StrBytes::new(),
            group_instance_id: None,
            retention_time_ms: -1,
            topics: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetCommitRequest {
    pub const API_KEY: i16 = 8;
    pub const VALID_MIN_VERSION: i16 = 2;
    pub const VALID_MAX_VERSION: i16 = 10;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 8;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 8 { compact_string_size(self.group_id.as_str()) } else { string_size(self.group_id.as_str()) };
        }
        if version >= 1 {
            size += 4;
        }
        if version >= 1 {
            size += if version >= 8 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
        }
        if version >= 7 {
            size += if version >= 7 { if version >= 8 { compact_nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 8 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 2 && version <= 4 {
            size += 8;
        }
        {
            { let arr = &self.topics;
                if version >= 8 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 8 { put_compact_string(buf, self.group_id.as_str()) } else { put_string(buf, self.group_id.as_str()) };
        }
        if version >= 1 {
            put_i32(buf, self.generation_id_or_member_epoch);
        }
        if version >= 1 {
            if version >= 8 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 7 {
            if version >= 7 { if version >= 8 { put_compact_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 8 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 2 && version <= 4 {
            put_i64(buf, self.retention_time_ms);
        }
        {
            { let arr = &self.topics;
                if version >= 8 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = OffsetCommitRequest::default();
        {
            msg.group_id = GroupId((if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 1 {
            msg.generation_id_or_member_epoch = get_i32(buf)?;
        }
        if version >= 1 {
            msg.member_id = (if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 7 {
            msg.group_instance_id = { let v = if version >= 8 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 7 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 2 && version <= 4 {
            msg.retention_time_ms = get_i64(buf)?;
        }
        {
            let len_opt = if version >= 8 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetCommitRequestTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 8 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetCommitRequest {
    const API_KEY: i16 = 8;
    const VALID_MIN_VERSION: i16 = 2;
    const VALID_MAX_VERSION: i16 = 10;
    const FLEXIBLE_MIN_VERSION: i16 = 8;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetCommitRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
