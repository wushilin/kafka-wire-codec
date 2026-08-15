#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    pub name: Bytes,
    /// The partitions inside the topic that we want to commit offsets for.
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TxnOffsetCommitRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 3 { compact_string_size(&self.name) } else { string_size(&self.name) };
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
            if version >= 3 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
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
            msg.name = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
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

#[derive(Debug, Clone)]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// The leader epoch of the last consumed record.
    pub committed_leader_epoch: i32,
    /// Any associated metadata the client wants to keep.
    pub committed_metadata: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for TxnOffsetCommitRequestPartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            committed_metadata: Some(Bytes::new()),
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
            size += if version >= 3 { compact_nullable_string_size(self.committed_metadata.as_deref()) } else { nullable_string_size(self.committed_metadata.as_deref()) };
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
            if version >= 3 { put_compact_nullable_string(buf, self.committed_metadata.as_deref()) } else { put_nullable_string(buf, self.committed_metadata.as_deref()) };
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
#[derive(Debug, Clone)]
pub struct TxnOffsetCommitRequest {
    /// The ID of the transaction.
    pub transactional_id: Bytes,
    /// The ID of the group.
    pub group_id: Bytes,
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,
    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,
    /// The generation of the consumer.
    pub generation_id: i32,
    /// The member ID assigned by the group coordinator.
    pub member_id: Bytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<Bytes>,
    /// Each topic that we want to commit offsets for.
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for TxnOffsetCommitRequest {
    fn default() -> Self {
        Self {
            transactional_id: Bytes::new(),
            group_id: Bytes::new(),
            producer_id: 0,
            producer_epoch: 0,
            generation_id: -1,
            member_id: Bytes::new(),
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

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 3 { compact_string_size(&self.transactional_id) } else { string_size(&self.transactional_id) };
        }
        {
            size += if version >= 3 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
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
            size += if version >= 3 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 3 { compact_nullable_string_size(self.group_instance_id.as_deref()) } else { nullable_string_size(self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 3 { compact_string_size(v) } else { string_size(v) } };
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
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 3 { put_compact_string(buf, &self.transactional_id) } else { put_string(buf, &self.transactional_id) };
        }
        {
            if version >= 3 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        {
            put_i64(buf, self.producer_id);
        }
        {
            put_i16(buf, self.producer_epoch);
        }
        if version >= 3 {
            put_i32(buf, self.generation_id);
        }
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        if version >= 3 {
            if version >= 3 { if version >= 3 { put_compact_nullable_string(buf, self.group_instance_id.as_deref()) } else { put_nullable_string(buf, self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 3 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            { let arr = &self.topics;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = TxnOffsetCommitRequest::default();
        {
            msg.transactional_id = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.group_id = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.producer_id = get_i64(buf)?;
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
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for TxnOffsetCommitRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
