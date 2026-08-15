#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FetchableTopicResponse {
    /// The topic name.
    pub topic: TopicName,
    /// The unique topic ID.
    pub topic_id: Uuid,
    /// The topic partitions.
    pub partitions: Vec<PartitionData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchableTopicResponse {
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
        let mut msg = FetchableTopicResponse::default();
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
                for _ in 0..count { items.push(PartitionData::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no fetch error.
    pub error_code: i16,
    /// The current high water mark.
    pub high_watermark: i64,
    /// The last stable offset (or LSO) of the partition. This is the last offset such that the state of all transactional records prior to this offset have been decided (ABORTED or COMMITTED).
    pub last_stable_offset: i64,
    /// The current log start offset.
    pub log_start_offset: i64,
    /// In case divergence is detected based on the `LastFetchedEpoch` and `FetchOffset` in the request, this field indicates the largest epoch and its end offset such that subsequent records are known to diverge.
    /// Tagged field (tag 0, versions 12+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub diverging_epoch: EpochEndOffset,
    /// The current leader of the partition.
    /// Tagged field (tag 1, versions 12+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub current_leader: LeaderIdAndEpoch,
    /// In the case of fetching an offset less than the LogStartOffset, this is the end offset and epoch that should be used in the FetchSnapshot request.
    /// Tagged field (tag 2, versions 12+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub snapshot_id: SnapshotId,
    /// The aborted transactions.
    pub aborted_transactions: Option<Vec<AbortedTransaction>>,
    /// The preferred read replica for the consumer to use on its next fetch request.
    pub preferred_read_replica: BrokerId,
    /// The record data.
    pub records: Option<Bytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for PartitionData {
    fn default() -> Self {
        Self {
            partition_index: 0,
            error_code: 0,
            high_watermark: 0,
            last_stable_offset: -1,
            log_start_offset: -1,
            diverging_epoch: EpochEndOffset::default(),
            current_leader: LeaderIdAndEpoch::default(),
            snapshot_id: SnapshotId::default(),
            aborted_transactions: Some(Vec::new()),
            preferred_read_replica: BrokerId(-1),
            records: Some(Bytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
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
        if version >= 4 {
            size += 8;
        }
        if version >= 5 {
            size += 8;
        }
        if version >= 4 {
            match &self.aborted_transactions {
                Some(arr) => {
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    assert!(version >= 4, "field aborted_transactions is None but not nullable at version {}", version);
                    if version >= 12 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 11 {
            size += 4;
        }
        {
            size += if version >= 12 { compact_nullable_bytes_size(self.records.as_deref()) } else { nullable_bytes_size(self.records.as_deref()) };
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 12 && (self.diverging_epoch != EpochEndOffset::default()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.diverging_epoch.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 12 && (self.current_leader != LeaderIdAndEpoch::default()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(1u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 12 && (self.snapshot_id != SnapshotId::default()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.snapshot_id.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(2u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i64(buf, self.high_watermark);
        }
        if version >= 4 {
            put_i64(buf, self.last_stable_offset);
        }
        if version >= 5 {
            put_i64(buf, self.log_start_offset);
        }
        if version >= 4 {
            match &self.aborted_transactions {
                Some(arr) => {
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    assert!(version >= 4, "field aborted_transactions is None but not nullable at version {}", version);
                    if version >= 12 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 11 {
            put_i32(buf, self.preferred_read_replica.0);
        }
        {
            if version >= 12 { put_compact_nullable_bytes_zc(buf, self.records.as_ref()) } else { put_nullable_bytes_zc(buf, self.records.as_ref()) };
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 12 && (self.diverging_epoch != EpochEndOffset::default()) { num_tagged += 1; }
            if version >= 12 && (self.current_leader != LeaderIdAndEpoch::default()) { num_tagged += 1; }
            if version >= 12 && (self.snapshot_id != SnapshotId::default()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            let mut raw_it = self.tagged_fields.iter().peekable();
            if version >= 12 && (self.diverging_epoch != EpochEndOffset::default()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += self.diverging_epoch.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.diverging_epoch.encode(version, buf);
            }
            if version >= 12 && (self.current_leader != LeaderIdAndEpoch::default()) {
                while let Some((t, d)) = raw_it.peek() { if *t < 1 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 1u64);
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.current_leader.encode(version, buf);
            }
            if version >= 12 && (self.snapshot_id != SnapshotId::default()) {
                while let Some((t, d)) = raw_it.peek() { if *t < 2 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 2u64);
                let data_len = { let mut size = 0usize;
            size += self.snapshot_id.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.snapshot_id.encode(version, buf);
            }
            for (t, d) in raw_it { put_raw_tagged_field(buf, *t, d); }
        } }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionData::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.high_watermark = get_i64(buf)?;
        }
        if version >= 4 {
            msg.last_stable_offset = get_i64(buf)?;
        }
        if version >= 5 {
            msg.log_start_offset = get_i64(buf)?;
        }
        if version >= 4 {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.aborted_transactions = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AbortedTransaction::decode(version, buf)?); }
                Some(items)
                }
                None => { if version >= 4 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
        }
        if version >= 11 {
            msg.preferred_read_replica = BrokerId(get_i32(buf)?);
        }
        {
            msg.records = if version >= 12 { get_compact_bytes(buf)? } else { get_bytes(buf)? };
        }
        if version >= 12 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 12 => {
                        let buf = &mut data;
            msg.diverging_epoch = EpochEndOffset::decode(version, buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    1 if version >= 12 => {
                        let buf = &mut data;
            msg.current_leader = LeaderIdAndEpoch::decode(version, buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    2 if version >= 12 => {
                        let buf = &mut data;
            msg.snapshot_id = SnapshotId::decode(version, buf)?;
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

#[derive(Debug, Clone, PartialEq)]
pub struct EpochEndOffset {
    /// The largest epoch.
    pub epoch: i32,
    /// The end offset of the epoch.
    pub end_offset: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for EpochEndOffset {
    fn default() -> Self {
        Self {
            epoch: -1,
            end_offset: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl EpochEndOffset {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 12 {
            size += 4;
        }
        if version >= 12 {
            size += 8;
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 12 {
            put_i32(buf, self.epoch);
        }
        if version >= 12 {
            put_i64(buf, self.end_offset);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = EpochEndOffset::default();
        if version >= 12 {
            msg.epoch = get_i32(buf)?;
        }
        if version >= 12 {
            msg.end_offset = get_i64(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
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
        if version >= 12 {
            size += 4;
        }
        if version >= 12 {
            size += 4;
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 12 {
            put_i32(buf, self.leader_id.0);
        }
        if version >= 12 {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = LeaderIdAndEpoch::default();
        if version >= 12 {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        if version >= 12 {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotId {
    /// The end offset of the epoch.
    pub end_offset: i64,
    /// The largest epoch.
    pub epoch: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for SnapshotId {
    fn default() -> Self {
        Self {
            end_offset: -1,
            epoch: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl SnapshotId {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 8;
        }
        {
            size += 4;
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i64(buf, self.end_offset);
        }
        {
            put_i32(buf, self.epoch);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = SnapshotId::default();
        {
            msg.end_offset = get_i64(buf)?;
        }
        {
            msg.epoch = get_i32(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbortedTransaction {
    /// The producer id associated with the aborted transaction.
    pub producer_id: ProducerId,
    /// The first offset in the aborted transaction.
    pub first_offset: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AbortedTransaction {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 4 {
            size += 8;
        }
        if version >= 4 {
            size += 8;
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 4 {
            put_i64(buf, self.producer_id.0);
        }
        if version >= 4 {
            put_i64(buf, self.first_offset);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AbortedTransaction::default();
        if version >= 4 {
            msg.producer_id = ProducerId(get_i64(buf)?);
        }
        if version >= 4 {
            msg.first_offset = get_i64(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
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
        if version >= 16 {
            size += 4;
        }
        if version >= 16 {
            size += if version >= 12 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        if version >= 16 {
            size += 4;
        }
        if version >= 16 {
            size += if version >= 16 { if version >= 12 { compact_nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 12 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 16 {
            put_i32(buf, self.node_id.0);
        }
        if version >= 16 {
            if version >= 12 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        if version >= 16 {
            put_i32(buf, self.port);
        }
        if version >= 16 {
            if version >= 16 { if version >= 12 { put_compact_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str())) } } else { let v = self.rack.as_ref().expect("field rack is None but not nullable at this version"); if version >= 12 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 16 {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        if version >= 16 {
            msg.host = (if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 16 {
            msg.port = get_i32(buf)?;
        }
        if version >= 16 {
            msg.rack = { let v = if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 16 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 4-18.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The fetch session ID, or 0 if this is not part of a fetch session.
    pub session_id: i32,
    /// The response topics.
    pub responses: Vec<FetchableTopicResponse>,
    /// Endpoints for all current-leaders enumerated in PartitionData, with errors NOT_LEADER_OR_FOLLOWER & FENCED_LEADER_EPOCH.
    /// Tagged field (tag 0, versions 16+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchResponse {
    pub const API_KEY: i16 = 1;
    pub const VALID_MIN_VERSION: i16 = 4;
    pub const VALID_MAX_VERSION: i16 = 18;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 12;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        if version >= 7 {
            size += 2;
        }
        if version >= 7 {
            size += 4;
        }
        {
            { let arr = &self.responses;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 16 && (!self.node_endpoints.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 7 {
            put_i16(buf, self.error_code);
        }
        if version >= 7 {
            put_i32(buf, self.session_id);
        }
        {
            { let arr = &self.responses;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 12 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 16 && (!self.node_endpoints.is_empty()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 16 && (!self.node_endpoints.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                if version >= 12 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.node_endpoints;
                if version >= 12 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        } }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = FetchResponse::default();
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 7 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 7 {
            msg.session_id = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FetchableTopicResponse::decode(version, buf)?); }
            msg.responses = items; }
        }
        if version >= 12 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 16 => {
                        let buf = &mut data;
            let len_opt = if version >= 12 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
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

impl crate::Encodable for FetchResponse {
    const API_KEY: i16 = 1;
    const VALID_MIN_VERSION: i16 = 4;
    const VALID_MAX_VERSION: i16 = 18;
    const FLEXIBLE_MIN_VERSION: i16 = 12;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FetchResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
