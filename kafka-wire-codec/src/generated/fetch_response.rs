#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct FetchableTopicResponse {
    /// The topic name.
    pub topic: Bytes,
    /// The unique topic ID.
    pub topic_id: [u8; 16],
    /// The topic partitions.
    pub partitions: Vec<PartitionData>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchableTopicResponse {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 12 { compact_string_size(&self.topic) } else { string_size(&self.topic) };
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
            if version >= 12 { put_compact_string(buf, &self.topic) } else { put_string(buf, &self.topic) };
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
            msg.topic = (if version >= 12 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
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

#[derive(Debug, Clone)]
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
    /// The aborted transactions.
    pub aborted_transactions: Option<Vec<AbortedTransaction>>,
    /// The preferred read replica for the consumer to use on its next fetch request.
    pub preferred_read_replica: i32,
    /// The record data.
    pub records: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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
            aborted_transactions: Some(Vec::new()),
            preferred_read_replica: -1,
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
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
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
            put_i32(buf, self.preferred_read_replica);
        }
        {
            if version >= 12 { put_compact_nullable_bytes_zc(buf, self.records.as_ref()) } else { put_nullable_bytes_zc(buf, self.records.as_ref()) };
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
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
            msg.preferred_read_replica = get_i32(buf)?;
        }
        {
            msg.records = if version >= 12 { get_compact_bytes(buf)? } else { get_bytes(buf)? };
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct EpochEndOffset {
    /// The largest epoch.
    pub epoch: i32,
    /// The end offset of the epoch.
    pub end_offset: i64,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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
            put_i32(buf, self.leader_id);
        }
        if version >= 12 {
            put_i32(buf, self.leader_epoch);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = LeaderIdAndEpoch::default();
        if version >= 12 {
            msg.leader_id = get_i32(buf)?;
        }
        if version >= 12 {
            msg.leader_epoch = get_i32(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct SnapshotId {
    /// The end offset of the epoch.
    pub end_offset: i64,
    /// The largest epoch.
    pub epoch: i32,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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

#[derive(Debug, Clone, Default)]
pub struct AbortedTransaction {
    /// The producer id associated with the aborted transaction.
    pub producer_id: i64,
    /// The first offset in the aborted transaction.
    pub first_offset: i64,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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
            put_i64(buf, self.producer_id);
        }
        if version >= 4 {
            put_i64(buf, self.first_offset);
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AbortedTransaction::default();
        if version >= 4 {
            msg.producer_id = get_i64(buf)?;
        }
        if version >= 4 {
            msg.first_offset = get_i64(buf)?;
        }
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
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
        if version >= 16 {
            size += 4;
        }
        if version >= 16 {
            size += if version >= 12 { compact_string_size(&self.host) } else { string_size(&self.host) };
        }
        if version >= 16 {
            size += 4;
        }
        if version >= 16 {
            size += if version >= 16 { if version >= 12 { compact_nullable_string_size(self.rack.as_deref()) } else { nullable_string_size(self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 12 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 16 {
            put_i32(buf, self.node_id);
        }
        if version >= 16 {
            if version >= 12 { put_compact_string(buf, &self.host) } else { put_string(buf, &self.host) };
        }
        if version >= 16 {
            put_i32(buf, self.port);
        }
        if version >= 16 {
            if version >= 16 { if version >= 12 { put_compact_nullable_string(buf, self.rack.as_deref()) } else { put_nullable_string(buf, self.rack.as_deref()) } } else { let v = self.rack.as_deref().expect("field rack is None but not nullable at this version"); if version >= 12 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 16 {
            msg.node_id = get_i32(buf)?;
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
#[derive(Debug, Clone, Default)]
pub struct FetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The fetch session ID, or 0 if this is not part of a fetch session.
    pub session_id: i32,
    /// The response topics.
    pub responses: Vec<FetchableTopicResponse>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchResponse {
    pub const API_KEY: i16 = 1;
    pub const VALID_MIN_VERSION: i16 = 4;
    pub const VALID_MAX_VERSION: i16 = 18;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 12;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
        if version >= 12 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
        if version >= 12 { put_tagged_fields(buf, &self.tagged_fields); }
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
        if version >= 12 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for FetchResponse {
    const API_KEY: i16 = 1;
    const VALID_MIN_VERSION: i16 = 4;
    const VALID_MAX_VERSION: i16 = 18;
    const FLEXIBLE_MIN_VERSION: i16 = 12;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FetchResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
