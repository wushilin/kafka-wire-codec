#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransactionState {
    /// The error code.
    pub error_code: i16,
    /// The transactional id.
    pub transactional_id: TransactionalId,
    /// The current transaction state of the producer.
    pub transaction_state: StrBytes,
    /// The timeout in milliseconds for the transaction.
    pub transaction_timeout_ms: i32,
    /// The start time of the transaction in milliseconds.
    pub transaction_start_time_ms: i64,
    /// The current producer id associated with the transaction.
    pub producer_id: ProducerId,
    /// The current epoch associated with the producer id.
    pub producer_epoch: i16,
    /// The set of partitions included in the current transaction (if active). When a transaction is preparing to commit or abort, this will include only partitions which do not have markers.
    pub topics: Vec<TopicData>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TransactionState {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += compact_string_size(self.transactional_id.as_str());
        }
        {
            size += compact_string_size(self.transaction_state.as_str());
        }
        {
            size += 4;
        }
        {
            size += 8;
        }
        {
            size += 8;
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
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            put_compact_string(buf, self.transactional_id.as_str());
        }
        {
            put_compact_string(buf, self.transaction_state.as_str());
        }
        {
            put_i32(buf, self.transaction_timeout_ms);
        }
        {
            put_i64(buf, self.transaction_start_time_ms);
        }
        {
            put_i64(buf, self.producer_id.0);
        }
        {
            put_i16(buf, self.producer_epoch);
        }
        {
            { let arr = &self.topics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = TransactionState::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.transactional_id = TransactionalId((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.transaction_state = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.transaction_timeout_ms = get_i32(buf)?;
        }
        {
            msg.transaction_start_time_ms = get_i64(buf)?;
        }
        {
            msg.producer_id = ProducerId(get_i64(buf)?);
        }
        {
            msg.producer_epoch = get_i16(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TopicData::decode(version, buf)?); }
            msg.topics = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TopicData {
    /// The topic name.
    pub topic: TopicName,
    /// The partition ids included in the current transaction.
    pub partitions: Vec<i32>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl TopicData {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.topic.as_str());
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
            put_compact_string(buf, self.topic.as_str());
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
        let mut msg = TopicData::default();
        {
            msg.topic = TopicName((get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?);
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

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeTransactionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The current state of the transaction.
    pub transaction_states: Vec<TransactionState>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeTransactionsResponse {
    pub const API_KEY: i16 = 65;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            { let arr = &self.transaction_states;
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
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.transaction_states;
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
        let mut msg = DescribeTransactionsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(TransactionState::decode(version, buf)?); }
            msg.transaction_states = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeTransactionsResponse {
    const API_KEY: i16 = 65;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeTransactionsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
