#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicProduceResponse {
    /// The topic name.
    pub name: TopicName,
    /// The unique topic ID
    pub topic_id: Uuid,
    /// Each partition that we produced to within the topic.
    pub partition_responses: Vec<PartitionProduceResponse>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicProduceResponse {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 9 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
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
            if version >= 9 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
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
            msg.name = TopicName((if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
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

#[derive(Debug, Clone, PartialEq)]
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
    pub error_message: Option<StrBytes>,
    /// The leader broker that the producer should use for future requests.
    /// Tagged field (tag 0, versions 10+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub current_leader: LeaderIdAndEpoch,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
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
            current_leader: LeaderIdAndEpoch::default(),
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
            size += if version >= 8 { if version >= 9 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 9 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 10 && (self.current_leader != LeaderIdAndEpoch::default()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
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
            if version >= 8 { if version >= 9 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 9 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 10 && (self.current_leader != LeaderIdAndEpoch::default()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 10 && (self.current_leader != LeaderIdAndEpoch::default()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.current_leader.encode(version, buf);
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        } }
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
        if version >= 9 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 10 => {
                        let buf = &mut data;
            msg.current_leader = LeaderIdAndEpoch::decode(version, buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        } }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BatchIndexAndErrorMessage {
    /// The batch index of the record that caused the batch to be dropped.
    pub batch_index: i32,
    /// The error message of the record that caused the batch to be dropped.
    pub batch_index_error_message: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl BatchIndexAndErrorMessage {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 {
            size += 4;
        }
        if version >= 8 {
            size += if version >= 8 { if version >= 9 { compact_nullable_string_size(self.batch_index_error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.batch_index_error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.batch_index_error_message.as_ref().expect("field batch_index_error_message is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 {
            put_i32(buf, self.batch_index);
        }
        if version >= 8 {
            if version >= 8 { if version >= 9 { put_compact_nullable_string(buf, self.batch_index_error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.batch_index_error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.batch_index_error_message.as_ref().expect("field batch_index_error_message is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
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

#[derive(Debug, Clone, PartialEq)]
pub struct LeaderIdAndEpoch {
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: BrokerId,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for LeaderIdAndEpoch {
    fn default() -> Self {
        Self {
            leader_id: BrokerId(-1),
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
            put_i32(buf, self.leader_id.0);
        }
        if version >= 10 {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = LeaderIdAndEpoch::default();
        if version >= 10 {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        if version >= 10 {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeEndpoint {
    /// The ID of the associated node.
    pub node_id: BrokerId,
    /// The node's hostname.
    pub host: StrBytes,
    /// The node's port.
    pub port: i32,
    /// The rack of the node, or null if it has not been assigned to a rack.
    pub rack: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl NodeEndpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 10 {
            size += 4;
        }
        if version >= 10 {
            size += if version >= 9 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        if version >= 10 {
            size += 4;
        }
        if version >= 10 {
            size += if version >= 10 { if version >= 9 { compact_nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 10 {
            put_i32(buf, self.node_id.0);
        }
        if version >= 10 {
            if version >= 9 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        if version >= 10 {
            put_i32(buf, self.port);
        }
        if version >= 10 {
            if version >= 10 { if version >= 9 { put_compact_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 10 {
            msg.node_id = BrokerId(get_i32(buf)?);
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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProduceResponse {
    /// Each produce response.
    pub responses: Vec<TopicProduceResponse>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Endpoints for all current-leaders enumerated in PartitionProduceResponses, with errors NOT_LEADER_OR_FOLLOWER.
    /// Tagged field (tag 0, versions 10+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
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
        if version >= 9 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 10 && (!self.node_endpoints.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
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
        if version >= 9 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 10 && (!self.node_endpoints.is_empty()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 10 && (!self.node_endpoints.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.node_endpoints;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        } }
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
        if version >= 9 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 10 => {
                        let buf = &mut data;
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(NodeEndpoint::decode(version, buf)?); }
            msg.node_endpoints = items; }
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        } }
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
