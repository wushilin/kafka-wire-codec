#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::chain::*;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicSnapshot {
    /// The name of the topic to fetch.
    pub name: TopicName,
    /// The partitions to fetch.
    pub partitions: Vec<PartitionSnapshot>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicSnapshot {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
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
            put_compact_string(buf, self.name.as_str());
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
        let mut msg = TopicSnapshot::default();
        {
            msg.name = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(PartitionSnapshot::decode(version, buf)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartitionSnapshot {
    /// The partition index.
    pub index: i32,
    /// The error code, or 0 if there was no fetch error.
    pub error_code: i16,
    /// The snapshot endOffset and epoch fetched.
    pub snapshot_id: SnapshotId,
    /// The leader of the partition at the time of the snapshot.
    /// Tagged field (tag 0, versions 0+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub current_leader: LeaderIdAndEpoch,
    /// The total size of the snapshot.
    pub size: i64,
    /// The starting byte position within the snapshot included in the Bytes field.
    pub position: i64,
    /// Snapshot data in records format which may not be aligned on an offset boundary.
    pub unaligned_records: Bytes,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl PartitionSnapshot {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += self.snapshot_id.encoded_size(version);
        }
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += compact_bytes_size(&self.unaligned_records);
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if self.current_leader != LeaderIdAndEpoch::default() {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        }
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
            self.snapshot_id.encode(version, buf);
        }
        {
            put_i64(buf, self.size);
        }
        {
            put_i64(buf, self.position);
        }
        {
            put_compact_bytes_zc(buf, &self.unaligned_records);
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            if self.current_leader != LeaderIdAndEpoch::default() { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if self.current_leader != LeaderIdAndEpoch::default() {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.current_leader.encode(version, buf);
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionSnapshot::default();
        {
            msg.index = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.snapshot_id = SnapshotId::decode(version, buf)?;
        }
        {
            msg.size = get_i64(buf)?;
        }
        {
            msg.position = get_i64(buf)?;
        }
        {
            msg.unaligned_records = (get_compact_bytes(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 => {
                        let buf = &mut data;
            msg.current_leader = LeaderIdAndEpoch::decode(version, buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SnapshotId {
    /// The snapshot end offset.
    pub end_offset: i64,
    /// The snapshot epoch.
    pub epoch: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
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
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i64(buf, self.end_offset);
        }
        {
            put_i32(buf, self.epoch);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = SnapshotId::default();
        {
            msg.end_offset = get_i64(buf)?;
        }
        {
            msg.epoch = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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

impl LeaderIdAndEpoch {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 4;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.leader_id.0);
        }
        {
            put_i32(buf, self.leader_epoch);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = LeaderIdAndEpoch::default();
        {
            msg.leader_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.leader_epoch = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
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
    pub port: u16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl NodeEndpoint {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        if version >= 1 {
            size += compact_string_size(self.host.as_str());
        }
        if version >= 1 {
            size += 2;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 1 {
            put_i32(buf, self.node_id.0);
        }
        if version >= 1 {
            put_compact_string(buf, self.host.as_str());
        }
        if version >= 1 {
            put_u16(buf, self.port);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = NodeEndpoint::default();
        if version >= 1 {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        if version >= 1 {
            msg.host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.port = get_u16(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-1.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FetchSnapshotResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The topics to fetch.
    pub topics: Vec<TopicSnapshot>,
    /// Endpoints for all current-leaders enumerated in PartitionSnapshot.
    /// Tagged field (tag 0, versions 1+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchSnapshotResponse {
    pub const API_KEY: i16 = 59;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
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
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 1 && (!self.node_endpoints.is_empty()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.node_endpoints;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = FetchSnapshotResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicSnapshot::decode(version, buf)?); }
            msg.topics = items; }
        }
        {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 1 => {
                        let buf = &mut data;
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
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
        }
        Ok(msg)
    }
}

impl crate::Encodable for FetchSnapshotResponse {
    const API_KEY: i16 = 59;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FetchSnapshotResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}

// ── Shell (chunked-payload) variants ─────────────────────────────────────────
// Records payloads decode as zero-copy chunk chains (`RecordsChunks`) from a
// `ChunkChain`, so payload-heavy frames never need one contiguous buffer.

/// Shell (chunked-payload) variant of [`TopicSnapshot`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicSnapshotShell {
    /// The name of the topic to fetch.
    pub name: TopicName,
    /// The partitions to fetch.
    pub partitions: Vec<PartitionSnapshotShell>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicSnapshotShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        let mut msg = TopicSnapshotShell::default();
        {
            msg.name = TopicName((ch_get_compact_string(ch)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = { let n = ch_get_uvarint32(ch)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(ch.remaining()));
                for _ in 0..count { items.push(PartitionSnapshotShell::decode_chained(version, ch)?); }
            msg.partitions = items; }
        }
        msg.tagged_fields = ch_get_tagged_fields(ch)?;
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
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
            put_compact_string(buf, self.name.as_str());
        }
        {
            { let arr = &self.partitions;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }
}

/// Shell (chunked-payload) variant of [`PartitionSnapshot`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartitionSnapshotShell {
    /// The partition index.
    pub index: i32,
    /// The error code, or 0 if there was no fetch error.
    pub error_code: i16,
    /// The snapshot endOffset and epoch fetched.
    pub snapshot_id: SnapshotId,
    /// The leader of the partition at the time of the snapshot.
    pub current_leader: LeaderIdAndEpoch,
    /// The total size of the snapshot.
    pub size: i64,
    /// The starting byte position within the snapshot included in the Bytes field.
    pub position: i64,
    /// Snapshot data in records format which may not be aligned on an offset boundary.
    pub unaligned_records: RecordsChunks,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl PartitionSnapshotShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        let mut msg = PartitionSnapshotShell::default();
        {
            msg.index = ch_get_i32(ch)?;
        }
        {
            msg.error_code = ch_get_i16(ch)?;
        }
        {
            msg.snapshot_id = SnapshotId::decode_chained(version, ch)?;
        }
        {
            msg.size = ch_get_i64(ch)?;
        }
        {
            msg.position = ch_get_i64(ch)?;
        }
        {
            msg.unaligned_records = (ch_get_compact_records(ch)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let count = ch_get_uvarint32(ch)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(ch.remaining() / 2));
            for _ in 0..count {
                let (tag, mut data) = ch_get_tagged_field(ch)?;
                match tag {
                    0 => {
                        let buf = &mut data;
            msg.current_leader = LeaderIdAndEpoch::decode(version, buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        }
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += self.snapshot_id.encoded_size(version);
        }
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += compact_records_chunks_size(&self.unaligned_records);
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if self.current_leader != LeaderIdAndEpoch::default() {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        }
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
            self.snapshot_id.encode(version, buf);
        }
        {
            put_i64(buf, self.size);
        }
        {
            put_i64(buf, self.position);
        }
        {
            put_compact_records_chunks_zc(buf, &self.unaligned_records);
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            if self.current_leader != LeaderIdAndEpoch::default() { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if self.current_leader != LeaderIdAndEpoch::default() {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            size += self.current_leader.encoded_size(version);
                size };
                put_uvarint(buf, data_len as u64);
            self.current_leader.encode(version, buf);
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        }
    }
}

impl SnapshotId {
    /// Decode from a chunk chain (shell path); identical result to `decode`.
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        let mut msg = SnapshotId::default();
        {
            msg.end_offset = ch_get_i64(ch)?;
        }
        {
            msg.epoch = ch_get_i32(ch)?;
        }
        msg.tagged_fields = ch_get_tagged_fields(ch)?;
        Ok(msg)
    }
}

/// Shell (chunked-payload) variant of [`FetchSnapshotResponse`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FetchSnapshotResponseShell {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The topics to fetch.
    pub topics: Vec<TopicSnapshotShell>,
    /// Endpoints for all current-leaders enumerated in PartitionSnapshot.
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FetchSnapshotResponseShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        if !(FetchSnapshotResponse::VALID_MIN_VERSION..=FetchSnapshotResponse::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: FetchSnapshotResponse::API_KEY, version });
        }
        let mut msg = FetchSnapshotResponseShell::default();
        {
            msg.throttle_time_ms = ch_get_i32(ch)?;
        }
        {
            msg.error_code = ch_get_i16(ch)?;
        }
        {
            let len_opt = { let n = ch_get_uvarint32(ch)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(ch.remaining()));
                for _ in 0..count { items.push(TopicSnapshotShell::decode_chained(version, ch)?); }
            msg.topics = items; }
        }
        {
            let count = ch_get_uvarint32(ch)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(ch.remaining() / 2));
            for _ in 0..count {
                let (tag, mut data) = ch_get_tagged_field(ch)?;
                match tag {
                    0 if version >= 1 => {
                        let buf = &mut data;
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
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
        }
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(FetchSnapshotResponse::VALID_MIN_VERSION..=FetchSnapshotResponse::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: FetchSnapshotResponse::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 2;
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
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        }
        Ok(size)
    }

    /// Encode; each records chunk becomes a shared segment on a zero-copy sink.
    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(FetchSnapshotResponse::VALID_MIN_VERSION..=FetchSnapshotResponse::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: FetchSnapshotResponse::API_KEY, version });
        }
        {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 1 && (!self.node_endpoints.is_empty()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            if version >= 1 && (!self.node_endpoints.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.node_endpoints;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.node_endpoints;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
            }
            for (t, d) in &self.tagged_fields { put_raw_tagged_field(buf, *t, d); }
        }
        Ok(())
    }
}

