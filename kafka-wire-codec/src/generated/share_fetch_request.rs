#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct FetchTopic {
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The partitions to fetch.
    pub partitions: Vec<FetchPartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchTopic {
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
        let mut msg = FetchTopic::default();
        {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FetchPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FetchPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The maximum bytes to fetch from this partition. 0 when only acknowledgement with no fetching is required. See KIP-74 for cases where this limit may not be honored.
    pub partition_max_bytes: i32,
    /// Record batches to acknowledge.
    pub acknowledgement_batches: Vec<AcknowledgementBatch>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        if version <= 0 {
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
        if version <= 0 {
            put_i32(buf, self.partition_max_bytes);
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
        let mut msg = FetchPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        if version <= 0 {
            msg.partition_max_bytes = get_i32(buf)?;
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

#[derive(Debug, Clone, Default)]
pub struct AcknowledgementBatch {
    /// First offset of batch of records to acknowledge.
    pub first_offset: i64,
    /// Last offset (inclusive) of batch of records to acknowledge.
    pub last_offset: i64,
    /// Array of acknowledge types - 0:Gap,1:Accept,2:Release,3:Reject,4:Renew.
    pub acknowledge_types: Vec<i8>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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

#[derive(Debug, Clone, Default)]
pub struct ForgottenTopic {
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The partitions indexes to forget.
    pub partitions: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ForgottenTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 16;
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
            { let arr = &self.partitions;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i32(buf, *item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ForgottenTopic::default();
        {
            msg.topic_id = get_uuid(buf)?;
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

/// Valid versions: 1-2.
#[derive(Debug, Clone)]
pub struct ShareFetchRequest {
    /// The group identifier.
    pub group_id: Option<GroupId>,
    /// The member ID.
    pub member_id: Option<StrBytes>,
    /// The current share session epoch: 0 to open a share session; -1 to close it; otherwise increments for consecutive requests.
    pub share_session_epoch: i32,
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait_ms: i32,
    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,
    /// The maximum bytes to fetch. See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,
    /// The maximum number of records to fetch. This limit can be exceeded for alignment of batch boundaries.
    pub max_records: i32,
    /// The optimal number of records for batches of acquired records and acknowledgements.
    pub batch_size: i32,
    /// The acquire mode to control the fetch behavior - 0:batch-optimized,1:record-limit.
    pub share_acquire_mode: i8,
    /// Whether Renew type acknowledgements present in AcknowledgementBatches.
    pub is_renew_ack: bool,
    /// The topics to fetch.
    pub topics: Vec<FetchTopic>,
    /// The partitions to remove from this share session.
    pub forgotten_topics_data: Vec<ForgottenTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ShareFetchRequest {
    fn default() -> Self {
        Self {
            group_id: None,
            member_id: Some(StrBytes::new()),
            share_session_epoch: 0,
            max_wait_ms: 0,
            min_bytes: 0,
            max_bytes: 0x7fffffff,
            max_records: 0,
            batch_size: 0,
            share_acquire_mode: 0,
            is_renew_ack: false,
            topics: Vec::new(),
            forgotten_topics_data: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl ShareFetchRequest {
    pub const API_KEY: i16 = 78;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
        {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        if version >= 1 {
            size += 4;
        }
        if version >= 1 {
            size += 4;
        }
        if version >= 2 {
            size += 1;
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
        {
            { let arr = &self.forgotten_topics_data;
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
            put_compact_nullable_string(buf, self.group_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_compact_nullable_string(buf, self.member_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.share_session_epoch);
        }
        {
            put_i32(buf, self.max_wait_ms);
        }
        {
            put_i32(buf, self.min_bytes);
        }
        {
            put_i32(buf, self.max_bytes);
        }
        if version >= 1 {
            put_i32(buf, self.max_records);
        }
        if version >= 1 {
            put_i32(buf, self.batch_size);
        }
        if version >= 2 {
            put_i8(buf, self.share_acquire_mode);
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
        {
            { let arr = &self.forgotten_topics_data;
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
        let mut msg = ShareFetchRequest::default();
        {
            msg.group_id = (get_compact_string(buf)?).map(GroupId);
        }
        {
            msg.member_id = get_compact_string(buf)?;
        }
        {
            msg.share_session_epoch = get_i32(buf)?;
        }
        {
            msg.max_wait_ms = get_i32(buf)?;
        }
        {
            msg.min_bytes = get_i32(buf)?;
        }
        {
            msg.max_bytes = get_i32(buf)?;
        }
        if version >= 1 {
            msg.max_records = get_i32(buf)?;
        }
        if version >= 1 {
            msg.batch_size = get_i32(buf)?;
        }
        if version >= 2 {
            msg.share_acquire_mode = get_i8(buf)?;
        }
        if version >= 2 {
            msg.is_renew_ack = get_bool(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FetchTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ForgottenTopic::decode(version, buf)?); }
            msg.forgotten_topics_data = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for ShareFetchRequest {
    const API_KEY: i16 = 78;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ShareFetchRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
