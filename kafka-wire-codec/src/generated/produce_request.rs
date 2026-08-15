#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct TopicProduceData {
    /// The topic name.
    pub name: TopicName,
    /// The unique topic ID
    pub topic_id: Uuid,
    /// Each partition to produce to.
    pub partition_data: Vec<PartitionProduceData>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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

#[derive(Debug, Clone)]
pub struct PartitionProduceData {
    /// The partition index.
    pub index: i32,
    /// The record data to be produced.
    pub records: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
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
#[derive(Debug, Clone, Default)]
pub struct ProduceRequest {
    /// The transactional ID, or null if the producer is not transactional.
    pub transactional_id: Option<TransactionalId>,
    /// The number of acknowledgments the producer requires the leader to have received before considering a request complete. Allowed values: 0 for no acknowledgments, 1 for only the leader and -1 for the full ISR.
    pub acks: i16,
    /// The timeout to await a response in milliseconds.
    pub timeout_ms: i32,
    /// Each topic to produce to.
    pub topic_data: Vec<TopicProduceData>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ProduceRequest {
    pub const API_KEY: i16 = 0;
    pub const VALID_MIN_VERSION: i16 = 3;
    pub const VALID_MAX_VERSION: i16 = 13;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 9;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
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
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ProduceRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
