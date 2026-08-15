#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AcknowledgeTopic {
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The partitions containing records to acknowledge.
    pub partitions: Vec<AcknowledgePartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AcknowledgeTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 16;
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
            put_uuid(buf, &self.topic_id);
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
        let mut msg = AcknowledgeTopic::default();
        {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AcknowledgePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AcknowledgePartition {
    /// The partition index.
    pub partition_index: i32,
    /// Record batches to acknowledge.
    pub acknowledgement_batches: Vec<AcknowledgementBatch>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AcknowledgePartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.acknowledgement_batches;
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
            { let arr = &self.acknowledgement_batches;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AcknowledgePartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AcknowledgementBatch::decode(version, buf)?); }
            msg.acknowledgement_batches = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AcknowledgementBatch {
    /// First offset of batch of records to acknowledge.
    pub first_offset: i64,
    /// Last offset (inclusive) of batch of records to acknowledge.
    pub last_offset: i64,
    /// Array of acknowledge types - 0:Gap,1:Accept,2:Release,3:Reject,4:Renew.
    pub acknowledge_types: Vec<i8>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AcknowledgementBatch {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            { let arr = &self.acknowledge_types;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len();
            }
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i64(buf, self.first_offset);
        }
        {
            put_i64(buf, self.last_offset);
        }
        {
            { let arr = &self.acknowledge_types;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i8(buf, *item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AcknowledgementBatch::default();
        {
            msg.first_offset = get_i64(buf)?;
        }
        {
            msg.last_offset = get_i64(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i8(buf)?); }
            msg.acknowledge_types = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 1-2.
#[derive(Debug, Clone, PartialEq)]
pub struct ShareAcknowledgeRequest {
    /// The group identifier.
    pub group_id: Option<GroupId>,
    /// The member ID.
    pub member_id: Option<StrBytes>,
    /// The current share session epoch: 0 to open a share session; -1 to close it; otherwise increments for consecutive requests.
    pub share_session_epoch: i32,
    /// Whether Renew type acknowledgements present in AcknowledgementBatches.
    pub is_renew_ack: bool,
    /// The topics containing records to acknowledge.
    pub topics: Vec<AcknowledgeTopic>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ShareAcknowledgeRequest {
    fn default() -> Self {
        Self {
            group_id: None,
            member_id: Some(StrBytes::new()),
            share_session_epoch: 0,
            is_renew_ack: false,
            topics: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl ShareAcknowledgeRequest {
    pub const API_KEY: i16 = 79;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += compact_nullable_string_size(self.group_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += compact_nullable_string_size(self.member_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += 4;
        }
        if version >= 2 {
            size += 1;
        }
        {
            { let arr = &self.topics;
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
            put_compact_nullable_string(buf, self.group_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_nullable_string(buf, self.member_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.share_session_epoch);
        }
        if version >= 2 {
            put_bool(buf, self.is_renew_ack);
        }
        {
            { let arr = &self.topics;
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
        let mut msg = ShareAcknowledgeRequest::default();
        {
            msg.group_id = (get_compact_string(buf)?).map(GroupId);
        }
        {
            msg.member_id = get_compact_string(buf)?;
        }
        {
            msg.share_session_epoch = get_i32(buf)?;
        }
        if version >= 2 {
            msg.is_renew_ack = get_bool(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AcknowledgeTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for ShareAcknowledgeRequest {
    const API_KEY: i16 = 79;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ShareAcknowledgeRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
