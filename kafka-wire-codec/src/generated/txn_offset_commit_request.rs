#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    pub name: TopicName,
    /// The partitions inside the topic that we want to commit offsets for.
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TxnOffsetCommitRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 3 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 3 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TxnOffsetCommitRequestTopic::default();
        {
            msg.name = TopicName((if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TxnOffsetCommitRequestPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// The leader epoch of the last consumed record.
    pub committed_leader_epoch: i32,
    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for TxnOffsetCommitRequestPartition {
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

impl TxnOffsetCommitRequestPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 8;
        }
        if version >= 2 {
            size += 4;
        }
        {
            size += if version >= 3 { compact_nullable_string_size(self.committed_metadata.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.committed_metadata.as_ref().map(|v| v.as_str())) };
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i64(buf, self.committed_offset);
        }
        if version >= 2 {
            put_i32(buf, self.committed_leader_epoch);
        }
        {
            if version >= 3 { put_compact_nullable_string(buf, self.committed_metadata.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.committed_metadata.as_ref().map(|v| v.as_str())) };
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TxnOffsetCommitRequestPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.committed_offset = get_i64(buf)?;
        }
        if version >= 2 {
            msg.committed_leader_epoch = get_i32(buf)?;
        }
        {
            msg.committed_metadata = if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-5.
#[derive(Debug, Clone, PartialEq)]
pub struct TxnOffsetCommitRequest {
    /// The ID of the transaction.
    pub transactional_id: TransactionalId,
    /// The ID of the group.
    pub group_id: GroupId,
    /// The current producer ID in use by the transactional ID.
    pub producer_id: ProducerId,
    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,
    /// The generation of the consumer.
    pub generation_id: i32,
    /// The member ID assigned by the group coordinator.
    pub member_id: StrBytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<StrBytes>,
    /// Each topic that we want to commit offsets for.
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for TxnOffsetCommitRequest {
    fn default() -> Self {
        Self {
            transactional_id: TransactionalId::default(),
            group_id: GroupId::default(),
            producer_id: ProducerId::default(),
            producer_epoch: 0,
            generation_id: -1,
            member_id: StrBytes::new(),
            group_instance_id: None,
            topics: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl TxnOffsetCommitRequest {
    pub const API_KEY: i16 = 28;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += if version >= 3 { compact_string_size(self.transactional_id.as_str()) } else { string_size(self.transactional_id.as_str()) };
        }
        {
            size += if version >= 3 { compact_string_size(self.group_id.as_str()) } else { string_size(self.group_id.as_str()) };
        }
        {
            size += 8;
        }
        {
            size += 2;
        }
        if version >= 3 {
            size += 4;
        }
        if version >= 3 {
            size += if version >= 3 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 3 { compact_nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 3 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            { let arr = &self.topics;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            if version >= 3 { put_compact_string(buf, self.transactional_id.as_str()) } else { put_string(buf, self.transactional_id.as_str()) };
        }
        {
            if version >= 3 { put_compact_string(buf, self.group_id.as_str()) } else { put_string(buf, self.group_id.as_str()) };
        }
        {
            put_i64(buf, self.producer_id.0);
        }
        {
            put_i16(buf, self.producer_epoch);
        }
        if version >= 3 {
            put_i32(buf, self.generation_id);
        }
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 3 {
            if version >= 3 { if version >= 3 { put_compact_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 3 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            { let arr = &self.topics;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = TxnOffsetCommitRequest::default();
        {
            msg.transactional_id = TransactionalId((if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.group_id = GroupId((if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.producer_id = ProducerId(get_i64(buf)?);
        }
        {
            msg.producer_epoch = get_i16(buf)?;
        }
        if version >= 3 {
            msg.generation_id = get_i32(buf)?;
        }
        if version >= 3 {
            msg.member_id = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.group_instance_id = { let v = if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TxnOffsetCommitRequestTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for TxnOffsetCommitRequest {
    const API_KEY: i16 = 28;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for TxnOffsetCommitRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
