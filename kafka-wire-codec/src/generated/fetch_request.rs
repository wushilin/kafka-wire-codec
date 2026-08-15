#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReplicaState {
    /// The replica ID of the follower, or -1 if this request is from a consumer.
    pub replica_id: BrokerId,
    /// The epoch of this follower, or -1 if not available.
    pub replica_epoch: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ReplicaState {
    fn default() -> Self {
        Self {
            replica_id: BrokerId(-1),
            replica_epoch: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl ReplicaState {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 15 {
            size += 4;
        }
        if version >= 15 {
            size += 8;
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 15 {
            put_i32(buf, self.replica_id.0);
        }
        if version >= 15 {
            put_i64(buf, self.replica_epoch);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ReplicaState::default();
        if version >= 15 {
            msg.replica_id = BrokerId(get_i32(buf)?);
        }
        if version >= 15 {
            msg.replica_epoch = get_i64(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FetchTopic {
    /// The name of the topic to fetch.
    pub topic: TopicName,
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The partitions to fetch.
    pub partitions: Vec<FetchPartition>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 12 { compact_string_size(self.topic.as_str()) } else { string_size(self.topic.as_str()) };
        }
        if version >= 13 {
            size += 16;
        }
        {
            { let arr = &self.partitions;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 12 {
            if version >= 12 { put_compact_string(buf, self.topic.as_str()) } else { put_string(buf, self.topic.as_str()) };
        }
        if version >= 13 {
            put_uuid(buf, &self.topic_id);
        }
        {
            { let arr = &self.partitions;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = FetchTopic::default();
        if version <= 12 {
            msg.topic = TopicName((if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 13 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FetchPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchPartition {
    /// The partition index.
    pub partition: i32,
    /// The current leader epoch of the partition.
    pub current_leader_epoch: i32,
    /// The message offset.
    pub fetch_offset: i64,
    /// The epoch of the last fetched record or -1 if there is none.
    pub last_fetched_epoch: i32,
    /// The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
    pub log_start_offset: i64,
    /// The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
    pub partition_max_bytes: i32,
    /// The directory id of the follower fetching.
    /// Tagged field (tag 0, versions 17+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub replica_directory_id: Uuid,
    /// The high-watermark known by the replica. -1 if the high-watermark is not known and 9223372036854775807 if the feature is not supported.
    /// Tagged field (tag 1, versions 18+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub high_watermark: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for FetchPartition {
    fn default() -> Self {
        Self {
            partition: 0,
            current_leader_epoch: -1,
            fetch_offset: 0,
            last_fetched_epoch: -1,
            log_start_offset: -1,
            partition_max_bytes: 0,
            replica_directory_id: Uuid::nil(),
            high_watermark: 9223372036854775807,
            tagged_fields: Vec::new(),
        }
    }
}

impl FetchPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 9 {
            size += 4;
        }
        {
            size += 8;
        }
        if version >= 12 {
            size += 4;
        }
        if version >= 5 {
            size += 8;
        }
        {
            size += 4;
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 17 && (self.replica_directory_id != Uuid::nil()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += 16;
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 18 && (self.high_watermark != 9223372036854775807) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += 8;
                size };
                known_tagged_size += uvarint_size(1u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition);
        }
        if version >= 9 {
            put_i32(buf, self.current_leader_epoch);
        }
        {
            put_i64(buf, self.fetch_offset);
        }
        if version >= 12 {
            put_i32(buf, self.last_fetched_epoch);
        }
        if version >= 5 {
            put_i64(buf, self.log_start_offset);
        }
        {
            put_i32(buf, self.partition_max_bytes);
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 17 && (self.replica_directory_id != Uuid::nil()) { num_tagged += 1; }
            if version >= 18 && (self.high_watermark != 9223372036854775807) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            let mut raw_it = self.tagged_fields.iter().peekable();
            if version >= 17 && (self.replica_directory_id != Uuid::nil()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += 16;
                size };
                put_uvarint(buf, data_len as u64);
            put_uuid(buf, &self.replica_directory_id);
            }
            if version >= 18 && (self.high_watermark != 9223372036854775807) {
                while let Some((t, d)) = raw_it.peek() { if *t < 1 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 1u64);
                let data_len = { let mut size = 0usize;
            size += 8;
                size };
                put_uvarint(buf, data_len as u64);
            put_i64(buf, self.high_watermark);
            }
            for (t, d) in raw_it { put_raw_tagged_field(buf, *t, d); }
        } }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = FetchPartition::default();
        {
            msg.partition = get_i32(buf)?;
        }
        if version >= 9 {
            msg.current_leader_epoch = get_i32(buf)?;
        }
        {
            msg.fetch_offset = get_i64(buf)?;
        }
        if version >= 12 {
            msg.last_fetched_epoch = get_i32(buf)?;
        }
        if version >= 5 {
            msg.log_start_offset = get_i64(buf)?;
        }
        {
            msg.partition_max_bytes = get_i32(buf)?;
        }
        if version >= 12 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 17 => {
                        let buf = &mut data;
            msg.replica_directory_id = get_uuid(buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    1 if version >= 18 => {
                        let buf = &mut data;
            msg.high_watermark = get_i64(buf)?;
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
pub struct ForgottenTopic {
    /// The topic name.
    pub topic: TopicName,
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The partitions indexes to forget.
    pub partitions: Vec<i32>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ForgottenTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 7 && version <= 12 {
            size += if version >= 12 { compact_string_size(self.topic.as_str()) } else { string_size(self.topic.as_str()) };
        }
        if version >= 13 {
            size += 16;
        }
        if version >= 7 {
            { let arr = &self.partitions;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 7 && version <= 12 {
            if version >= 12 { put_compact_string(buf, self.topic.as_str()) } else { put_string(buf, self.topic.as_str()) };
        }
        if version >= 13 {
            put_uuid(buf, &self.topic_id);
        }
        if version >= 7 {
            { let arr = &self.partitions;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ForgottenTopic::default();
        if version >= 7 && version <= 12 {
            msg.topic = TopicName((if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 13 {
            msg.topic_id = get_uuid(buf)?;
        }
        if version >= 7 {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partitions = items; }
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 4-18.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchRequest {
    /// The clusterId if known. This is used to validate metadata fetches prior to broker registration.
    /// Tagged field (tag 0, versions 12+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub cluster_id: Option<StrBytes>,
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: BrokerId,
    /// The state of the replica in the follower.
    /// Tagged field (tag 1, versions 15+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub replica_state: ReplicaState,
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait_ms: i32,
    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,
    /// The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,
    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records.
    pub isolation_level: i8,
    /// The fetch session ID.
    pub session_id: i32,
    /// The fetch session epoch, which is used for ordering requests in a session.
    pub session_epoch: i32,
    /// The topics to fetch.
    pub topics: Vec<FetchTopic>,
    /// In an incremental fetch request, the partitions to remove.
    pub forgotten_topics_data: Vec<ForgottenTopic>,
    /// Rack ID of the consumer making this request.
    pub rack_id: StrBytes,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for FetchRequest {
    fn default() -> Self {
        Self {
            cluster_id: None,
            replica_id: BrokerId(-1),
            replica_state: ReplicaState::default(),
            max_wait_ms: 0,
            min_bytes: 0,
            max_bytes: 0x7fffffff,
            isolation_level: 0,
            session_id: 0,
            session_epoch: -1,
            topics: Vec::new(),
            forgotten_topics_data: Vec::new(),
            rack_id: StrBytes::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl FetchRequest {
    pub const API_KEY: i16 = 1;
    pub const VALID_MIN_VERSION: i16 = 4;
    pub const VALID_MAX_VERSION: i16 = 18;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 12;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version <= 14 {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        if version >= 3 {
            size += 4;
        }
        if version >= 4 {
            size += 1;
        }
        if version >= 7 {
            size += 4;
        }
        if version >= 7 {
            size += 4;
        }
        {
            { let arr = &self.topics;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 7 {
            { let arr = &self.forgotten_topics_data;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 11 {
            size += if version >= 12 { compact_string_size(self.rack_id.as_str()) } else { string_size(self.rack_id.as_str()) };
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 12 && (self.cluster_id.is_some()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += if version >= 12 { if version >= 12 { compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } } else { let v = self.cluster_id.as_ref().expect("field cluster_id is None but not nullable at this version"); if version >= 12 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 15 && (self.replica_state != ReplicaState::default()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.replica_state.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(1u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version <= 14 {
            put_i32(buf, self.replica_id.0);
        }
        {
            put_i32(buf, self.max_wait_ms);
        }
        {
            put_i32(buf, self.min_bytes);
        }
        if version >= 3 {
            put_i32(buf, self.max_bytes);
        }
        if version >= 4 {
            put_i8(buf, self.isolation_level);
        }
        if version >= 7 {
            put_i32(buf, self.session_id);
        }
        if version >= 7 {
            put_i32(buf, self.session_epoch);
        }
        {
            { let arr = &self.topics;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 7 {
            { let arr = &self.forgotten_topics_data;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 11 {
            if version >= 12 { put_compact_string(buf, self.rack_id.as_str()) } else { put_string(buf, self.rack_id.as_str()) };
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 12 && (self.cluster_id.is_some()) { num_tagged += 1; }
            if version >= 15 && (self.replica_state != ReplicaState::default()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            let mut raw_it = self.tagged_fields.iter().peekable();
            if version >= 12 && (self.cluster_id.is_some()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += if version >= 12 { if version >= 12 { compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str())) } } else { let v = self.cluster_id.as_ref().expect("field cluster_id is None but not nullable at this version"); if version >= 12 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
                size };
                put_uvarint(buf, data_len as u64);
            if version >= 12 { if version >= 12 { put_compact_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str())) } } else { let v = self.cluster_id.as_ref().expect("field cluster_id is None but not nullable at this version"); if version >= 12 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
            }
            if version >= 15 && (self.replica_state != ReplicaState::default()) {
                while let Some((t, d)) = raw_it.peek() { if *t < 1 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 1u64);
                let data_len = { let mut size = 0usize;
            size += self.replica_state.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.replica_state.encode(version, buf);
            }
            for (t, d) in raw_it { put_raw_tagged_field(buf, *t, d); }
        } }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = FetchRequest::default();
        if version <= 14 {
            msg.replica_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.max_wait_ms = get_i32(buf)?;
        }
        {
            msg.min_bytes = get_i32(buf)?;
        }
        if version >= 3 {
            msg.max_bytes = get_i32(buf)?;
        }
        if version >= 4 {
            msg.isolation_level = get_i8(buf)?;
        }
        if version >= 7 {
            msg.session_id = get_i32(buf)?;
        }
        if version >= 7 {
            msg.session_epoch = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FetchTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 7 {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ForgottenTopic::decode(version, buf)?); }
            msg.forgotten_topics_data = items; }
        }
        if version >= 11 {
            msg.rack_id = (if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 12 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 12 => {
                        let buf = &mut data;
            msg.cluster_id = { let v = if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 12 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    1 if version >= 15 => {
                        let buf = &mut data;
            msg.replica_state = ReplicaState::decode(version, buf)?;
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

impl crate::Encodable for FetchRequest {
    const API_KEY: i16 = 1;
    const VALID_MIN_VERSION: i16 = 4;
    const VALID_MAX_VERSION: i16 = 18;
    const FLEXIBLE_MIN_VERSION: i16 = 12;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FetchRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
