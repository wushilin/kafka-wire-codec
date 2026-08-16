#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::chain::*;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicProduceData {
    /// The topic name.
    pub name: TopicName,
    /// The unique topic ID
    pub topic_id: Uuid,
    /// Each partition to produce to.
    pub partition_data: Vec<PartitionProduceData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicProduceData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 9 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 13 {
            size += 16;
        }
        {
            { let arr = &self.partition_data;
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
            { let arr = &self.partition_data;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TopicProduceData::default();
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
                for _ in 0..count { items.push(PartitionProduceData::decode(version, buf)?); }
            msg.partition_data = items; }
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartitionProduceData {
    /// The partition index.
    pub index: i32,
    /// The record data to be produced.
    pub records: Option<Bytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for PartitionProduceData {
    fn default() -> Self {
        Self {
            index: 0,
            records: Some(Bytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl PartitionProduceData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += if version >= 9 { compact_nullable_bytes_size(self.records.as_deref()) } else { nullable_bytes_size(self.records.as_deref()) };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.index);
        }
        {
            if version >= 9 { put_compact_nullable_bytes_zc(buf, self.records.as_ref()) } else { put_nullable_bytes_zc(buf, self.records.as_ref()) };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = PartitionProduceData::default();
        {
            msg.index = get_i32(buf)?;
        }
        {
            msg.records = if version >= 9 { get_compact_bytes(buf)? } else { get_bytes(buf)? };
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 3-13.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    pub transactional_id: Option<TransactionalId>,
    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    pub acks: i16,
    /// The timeout to await a response in milliseconds.
    pub timeout_ms: i32,
    /// Each topic to produce to.
    pub topic_data: Vec<TopicProduceData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ProduceRequest {
    pub const API_KEY: i16 = 0;
    pub const VALID_MIN_VERSION: i16 = 3;
    pub const VALID_MAX_VERSION: i16 = 13;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 9;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { if version >= 9 { compact_nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) } } else { let v = self.transactional_id.as_ref().expect("field transactional_id is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            size += 2;
        }
        {
            size += 4;
        }
        {
            { let arr = &self.topic_data;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 3 {
            if version >= 3 { if version >= 9 { put_compact_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) } } else { let v = self.transactional_id.as_ref().expect("field transactional_id is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            put_i16(buf, self.acks);
        }
        {
            put_i32(buf, self.timeout_ms);
        }
        {
            { let arr = &self.topic_data;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ProduceRequest::default();
        if version >= 3 {
            msg.transactional_id = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } }.map(TransactionalId);
        }
        {
            msg.acks = get_i16(buf)?;
        }
        {
            msg.timeout_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicProduceData::decode(version, buf)?); }
            msg.topic_data = items; }
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for ProduceRequest {
    const API_KEY: i16 = 0;
    const VALID_MIN_VERSION: i16 = 3;
    const VALID_MAX_VERSION: i16 = 13;
    const FLEXIBLE_MIN_VERSION: i16 = 9;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ProduceRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}

// ── Shell (chunked-payload) variants ─────────────────────────────────────────
// Records payloads decode as zero-copy chunk chains (`RecordsChunks`) from a
// `ChunkChain`, so payload-heavy frames never need one contiguous buffer.

/// Shell (chunked-payload) variant of [`TopicProduceData`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicProduceDataShell {
    /// The topic name.
    pub name: TopicName,
    /// The unique topic ID
    pub topic_id: Uuid,
    /// Each partition to produce to.
    pub partition_data: Vec<PartitionProduceDataShell>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicProduceDataShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        let mut msg = TopicProduceDataShell::default();
        if version <= 12 {
            msg.name = TopicName((if version >= 9 { ch_get_compact_string(ch)? } else { ch_get_string(ch)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        if version >= 13 {
            msg.topic_id = ch_get_uuid(ch)?;
        }
        {
            let len_opt = if version >= 9 { { let n = ch_get_uvarint32(ch)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = ch_get_i32(ch)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(ch.remaining()));
                for _ in 0..count { items.push(PartitionProduceDataShell::decode_chained(version, ch)?); }
            msg.partition_data = items; }
        }
        if version >= 9 { msg.tagged_fields = ch_get_tagged_fields(ch)?; }
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 12 {
            size += if version >= 9 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 13 {
            size += 16;
        }
        {
            { let arr = &self.partition_data;
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
            { let arr = &self.partition_data;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }
}

/// Shell (chunked-payload) variant of [`PartitionProduceData`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionProduceDataShell {
    /// The partition index.
    pub index: i32,
    /// The record data to be produced.
    pub records: Option<RecordsChunks>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for PartitionProduceDataShell {
    fn default() -> Self {
        Self {
            index: 0,
            records: Some(RecordsChunks::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl PartitionProduceDataShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        let mut msg = PartitionProduceDataShell::default();
        {
            msg.index = ch_get_i32(ch)?;
        }
        {
            msg.records = if version >= 9 { ch_get_compact_records(ch)? } else { ch_get_records(ch)? };
        }
        if version >= 9 { msg.tagged_fields = ch_get_tagged_fields(ch)?; }
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += if version >= 9 { compact_nullable_records_chunks_size(self.records.as_ref()) } else { nullable_records_chunks_size(self.records.as_ref()) };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.index);
        }
        {
            if version >= 9 { put_compact_nullable_records_chunks_zc(buf, self.records.as_ref()) } else { put_nullable_records_chunks_zc(buf, self.records.as_ref()) };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }
}

/// Shell (chunked-payload) variant of [`ProduceRequest`]: identical except records
/// payloads are `RecordsChunks` chunk chains instead of contiguous `Bytes`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProduceRequestShell {
    /// The transactional ID, or null if the producer is not transactional.
    pub transactional_id: Option<TransactionalId>,
    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    pub acks: i16,
    /// The timeout to await a response in milliseconds.
    pub timeout_ms: i32,
    /// Each topic to produce to.
    pub topic_data: Vec<TopicProduceDataShell>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ProduceRequestShell {
    /// Decode from a chunk chain; records payloads come out as zero-copy
    /// chunk slices (see `frame::read_frame_supplied`).
    pub fn decode_chained(version: i16, ch: &mut ChunkChain) -> Result<Self, DecodeError> {
        if !(ProduceRequest::VALID_MIN_VERSION..=ProduceRequest::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: ProduceRequest::API_KEY, version });
        }
        let mut msg = ProduceRequestShell::default();
        if version >= 3 {
            msg.transactional_id = { let v = if version >= 9 { ch_get_compact_string(ch)? } else { ch_get_string(ch)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } }.map(TransactionalId);
        }
        {
            msg.acks = ch_get_i16(ch)?;
        }
        {
            msg.timeout_ms = ch_get_i32(ch)?;
        }
        {
            let len_opt = if version >= 9 { { let n = ch_get_uvarint32(ch)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = ch_get_i32(ch)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(ch.remaining()));
                for _ in 0..count { items.push(TopicProduceDataShell::decode_chained(version, ch)?); }
            msg.topic_data = items; }
        }
        if version >= 9 { msg.tagged_fields = ch_get_tagged_fields(ch)?; }
        Ok(msg)
    }

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(ProduceRequest::VALID_MIN_VERSION..=ProduceRequest::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: ProduceRequest::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { if version >= 9 { compact_nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) } } else { let v = self.transactional_id.as_ref().expect("field transactional_id is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            size += 2;
        }
        {
            size += 4;
        }
        {
            { let arr = &self.topic_data;
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    /// Encode; each records chunk becomes a shared segment on a zero-copy sink.
    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(ProduceRequest::VALID_MIN_VERSION..=ProduceRequest::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: ProduceRequest::API_KEY, version });
        }
        if version >= 3 {
            if version >= 3 { if version >= 9 { put_compact_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) } } else { let v = self.transactional_id.as_ref().expect("field transactional_id is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            put_i16(buf, self.acks);
        }
        {
            put_i32(buf, self.timeout_ms);
        }
        {
            { let arr = &self.topic_data;
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }
}

