#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct AddPartitionsToTxnTopic {
    /// The name of the topic.
    pub name: Bytes,
    /// The partition indexes to add to the transaction.
    pub partitions: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AddPartitionsToTxnTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 3 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        {
            { let arr = &self.partitions;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 3 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        {
            { let arr = &self.partitions;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AddPartitionsToTxnTopic::default();
        {
            msg.name = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partitions = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AddPartitionsToTxnTransaction {
    /// The transactional id corresponding to the transaction.
    pub transactional_id: Bytes,
    /// Current producer id in use by the transactional id.
    pub producer_id: i64,
    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,
    /// Boolean to signify if we want to check if the partition is in the transaction rather than add it.
    pub verify_only: bool,
    /// The partitions to add to the transaction.
    pub topics: Vec<AddPartitionsToTxnTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AddPartitionsToTxnTransaction {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 4 {
            size += if version >= 3 { compact_string_size(&self.transactional_id) } else { string_size(&self.transactional_id) };
        }
        if version >= 4 {
            size += 8;
        }
        if version >= 4 {
            size += 2;
        }
        if version >= 4 {
            size += 1;
        }
        if version >= 4 {
            { let arr = &self.topics;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 4 {
            if version >= 3 { put_compact_string(buf, &self.transactional_id) } else { put_string(buf, &self.transactional_id) };
        }
        if version >= 4 {
            put_i64(buf, self.producer_id);
        }
        if version >= 4 {
            put_i16(buf, self.producer_epoch);
        }
        if version >= 4 {
            put_bool(buf, self.verify_only);
        }
        if version >= 4 {
            { let arr = &self.topics;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = AddPartitionsToTxnTransaction::default();
        if version >= 4 {
            msg.transactional_id = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 4 {
            msg.producer_id = get_i64(buf)?;
        }
        if version >= 4 {
            msg.producer_epoch = get_i16(buf)?;
        }
        if version >= 4 {
            msg.verify_only = get_bool(buf)?;
        }
        if version >= 4 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AddPartitionsToTxnTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-5.
#[derive(Debug, Clone, Default)]
pub struct AddPartitionsToTxnRequest {
    /// List of transactions to add partitions to.
    pub transactions: Vec<AddPartitionsToTxnTransaction>,
    /// The transactional id corresponding to the transaction.
    pub v3_and_below_transactional_id: Bytes,
    /// Current producer id in use by the transactional id.
    pub v3_and_below_producer_id: i64,
    /// Current epoch associated with the producer id.
    pub v3_and_below_producer_epoch: i16,
    /// The partitions to add to the transaction.
    pub v3_and_below_topics: Vec<AddPartitionsToTxnTopic>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl AddPartitionsToTxnRequest {
    pub const API_KEY: i16 = 24;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 4 {
            { let arr = &self.transactions;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version <= 3 {
            size += if version >= 3 { compact_string_size(&self.v3_and_below_transactional_id) } else { string_size(&self.v3_and_below_transactional_id) };
        }
        if version <= 3 {
            size += 8;
        }
        if version <= 3 {
            size += 2;
        }
        if version <= 3 {
            { let arr = &self.v3_and_below_topics;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 4 {
            { let arr = &self.transactions;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version <= 3 {
            if version >= 3 { put_compact_string(buf, &self.v3_and_below_transactional_id) } else { put_string(buf, &self.v3_and_below_transactional_id) };
        }
        if version <= 3 {
            put_i64(buf, self.v3_and_below_producer_id);
        }
        if version <= 3 {
            put_i16(buf, self.v3_and_below_producer_epoch);
        }
        if version <= 3 {
            { let arr = &self.v3_and_below_topics;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = AddPartitionsToTxnRequest::default();
        if version >= 4 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AddPartitionsToTxnTransaction::decode(version, buf)?); }
            msg.transactions = items; }
        }
        if version <= 3 {
            msg.v3_and_below_transactional_id = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 3 {
            msg.v3_and_below_producer_id = get_i64(buf)?;
        }
        if version <= 3 {
            msg.v3_and_below_producer_epoch = get_i16(buf)?;
        }
        if version <= 3 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(AddPartitionsToTxnTopic::decode(version, buf)?); }
            msg.v3_and_below_topics = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for AddPartitionsToTxnRequest {
    const API_KEY: i16 = 24;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for AddPartitionsToTxnRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
