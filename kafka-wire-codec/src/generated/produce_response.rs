#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct TopicProduceResponse {
    /// The topic name.
    pub name: Bytes,
    /// The unique topic ID
    pub topic_id: [u8; 16],
    /// Each partition that we produced to within the topic.
    pub partition_responses: Vec<PartitionProduceResponse>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicProduceResponse {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 9 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version >= 13 {
            size += 16;
        }
        {
            { let arr = &self.partition_responses;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 12 {
            if version >= 9 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version >= 13 {
            put_uuid(buf, &self.topic_id);
        }
        {
            { let arr = &self.partition_responses;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TopicProduceResponse::default();
        if version <= 12 {
            msg.name = (if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 13 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(PartitionProduceResponse::decode(version, buf)?); }
            msg.partition_responses = items; }
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct PartitionProduceResponse {
    /// The partition index.
    pub index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The base offset.
    pub base_offset: i64,
    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    pub log_append_time_ms: i64,
    /// The log start offset.
    pub log_start_offset: i64,
    /// The batch indices of records that caused the batch to be dropped.
    pub record_errors: Vec<BatchIndexAndErrorMessage>,
    /// The global error message summarizing the common root cause of the records that caused the batch to be dropped.
    pub error_message: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        Self {
            index: 0,
            error_code: 0,
            base_offset: 0,
            log_append_time_ms: -1,
            log_start_offset: -1,
            record_errors: Vec::new(),
            error_message: None,
            tagged_fields: Vec::new(),
        }
    }
}

impl PartitionProduceResponse {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += 8;
        }
        if version >= 2 {
            size += 8;
        }
        if version >= 5 {
            size += 8;
        }
        if version >= 8 {
            { let arr = &self.record_errors;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 {
            size += if version >= 8 { if version >= 9 { compact_nullable_string_size(self.error_message.as_deref()) } else { nullable_string_size(self.error_message.as_deref()) } } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.index);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i64(buf, self.base_offset);
        }
        if version >= 2 {
            put_i64(buf, self.log_append_time_ms);
        }
        if version >= 5 {
            put_i64(buf, self.log_start_offset);
        }
        if version >= 8 {
            { let arr = &self.record_errors;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 {
            if version >= 8 { if version >= 9 { put_compact_nullable_string(buf, self.error_message.as_deref()) } else { put_nullable_string(buf, self.error_message.as_deref()) } } else { let v = self.error_message.as_deref().expect("field error_message is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionProduceResponse::default();
        {
            msg.index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.base_offset = get_i64(buf)?;
        }
        if version >= 2 {
            msg.log_append_time_ms = get_i64(buf)?;
        }
        if version >= 5 {
            msg.log_start_offset = get_i64(buf)?;
        }
        if version >= 8 {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(BatchIndexAndErrorMessage::decode(version, buf)?); }
            msg.record_errors = items; }
        }
        if version >= 8 {
            msg.error_message = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 8 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BatchIndexAndErrorMessage {
    /// The batch index of the record that caused the batch to be dropped.
    pub batch_index: i32,
    /// The error message of the record that caused the batch to be dropped.
    pub batch_index_error_message: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl BatchIndexAndErrorMessage {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 {
            size += 4;
        }
        if version >= 8 {
            size += if version >= 8 { if version >= 9 { compact_nullable_string_size(self.batch_index_error_message.as_deref()) } else { nullable_string_size(self.batch_index_error_message.as_deref()) } } else { let v = self.batch_index_error_message.as_deref().expect("field batch_index_error_message is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 {
            put_i32(buf, self.batch_index);
        }
        if version >= 8 {
            if version >= 8 { if version >= 9 { put_compact_nullable_string(buf, self.batch_index_error_message.as_deref()) } else { put_nullable_string(buf, self.batch_index_error_message.as_deref()) } } else { let v = self.batch_index_error_message.as_deref().expect("field batch_index_error_message is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = BatchIndexAndErrorMessage::default();
        if version >= 8 {
            msg.batch_index = get_i32(buf)?;
        }
        if version >= 8 {
            msg.batch_index_error_message = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 8 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct LeaderIdAndEpoch {
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for LeaderIdAndEpoch {
    fn default() -> Self {
        Self {
            leader_id: -1,
            leader_epoch: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl LeaderIdAndEpoch {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 10 {
            size += 4;
        }
        if version >= 10 {
            size += 4;
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 10 {
            put_i32(buf, self.leader_id);
        }
        if version >= 10 {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = LeaderIdAndEpoch::default();
        if version >= 10 {
            msg.leader_id = get_i32(buf)?;
        }
        if version >= 10 {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeEndpoint {
    /// The ID of the associated node.
    pub node_id: i32,
    /// The node's hostname.
    pub host: Bytes,
    /// The node's port.
    pub port: i32,
    /// The rack of the node, or null if it has not been assigned to a rack.
    pub rack: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl NodeEndpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 10 {
            size += 4;
        }
        if version >= 10 {
            size += if version >= 9 { compact_string_size(&self.host) } else { string_size(&self.host) };
        }
        if version >= 10 {
            size += 4;
        }
        if version >= 10 {
            size += if version >= 10 { if version >= 9 { compact_nullable_string_size(self.rack.as_deref()) } else { nullable_string_size(self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 9 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 10 {
            put_i32(buf, self.node_id);
        }
        if version >= 10 {
            if version >= 9 { put_compact_string(buf, &self.host) } else { put_string(buf, &self.host) };
        }
        if version >= 10 {
            put_i32(buf, self.port);
        }
        if version >= 10 {
            if version >= 10 { if version >= 9 { put_compact_nullable_string(buf, self.rack.as_deref()) } else { put_nullable_string(buf, self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 10 {
            msg.node_id = get_i32(buf)?;
        }
        if version >= 10 {
            msg.host = (if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 10 {
            msg.port = get_i32(buf)?;
        }
        if version >= 10 {
            msg.rack = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 10 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 3-13.
#[derive(Debug, Clone, Default)]
pub struct ProduceResponse {
    /// Each produce response.
    pub responses: Vec<TopicProduceResponse>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ProduceResponse {
    pub const API_KEY: i16 = 0;
    pub const VALID_MIN_VERSION: i16 = 3;
    pub const VALID_MAX_VERSION: i16 = 13;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 9;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            { let arr = &self.responses;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 {
            size += 4;
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            { let arr = &self.responses;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ProduceResponse::default();
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicProduceResponse::decode(version, buf)?); }
            msg.responses = items; }
        }
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for ProduceResponse {
    const API_KEY: i16 = 0;
    const VALID_MIN_VERSION: i16 = 3;
    const VALID_MAX_VERSION: i16 = 13;
    const FLEXIBLE_MIN_VERSION: i16 = 9;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ProduceResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
