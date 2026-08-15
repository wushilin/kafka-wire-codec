#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-6.
#[derive(Debug, Clone)]
pub struct InitProducerIdRequest {
    /// The transactional id, or null if the producer is not transactional.
    pub transactional_id: Option<TransactionalId>,
    /// The time in ms to wait before aborting idle transactions sent by this producer. This is only relevant if a TransactionalId has been defined.
    pub transaction_timeout_ms: i32,
    /// The producer id. This is used to disambiguate requests if a transactional id is reused following its expiration.
    pub producer_id: ProducerId,
    /// The producer's current epoch. This will be checked against the producer epoch on the broker, and the request will return an error if they do not match.
    pub producer_epoch: i16,
    /// True if the client wants to enable two-phase commit (2PC) protocol for transactions.
    pub enable2_pc: bool,
    /// True if the client wants to keep the currently ongoing transaction instead of aborting it.
    pub keep_prepared_txn: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for InitProducerIdRequest {
    fn default() -> Self {
        Self {
            transactional_id: Some(TransactionalId::default()),
            transaction_timeout_ms: 0,
            producer_id: ProducerId(-1),
            producer_epoch: -1,
            enable2_pc: false,
            keep_prepared_txn: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl InitProducerIdRequest {
    pub const API_KEY: i16 = 22;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 6;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.transactional_id.as_ref().map(|v| v.as_str())) };
        }
        {
            size += 4;
        }
        if version >= 3 {
            size += 8;
        }
        if version >= 3 {
            size += 2;
        }
        if version >= 6 {
            size += 1;
        }
        if version >= 6 {
            size += 1;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 2 { put_compact_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.transactional_id.as_ref().map(|v| v.as_str())) };
        }
        {
            put_i32(buf, self.transaction_timeout_ms);
        }
        if version >= 3 {
            put_i64(buf, self.producer_id.0);
        }
        if version >= 3 {
            put_i16(buf, self.producer_epoch);
        }
        if version >= 6 {
            put_bool(buf, self.enable2_pc);
        }
        if version >= 6 {
            put_bool(buf, self.keep_prepared_txn);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = InitProducerIdRequest::default();
        {
            msg.transactional_id = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).map(TransactionalId);
        }
        {
            msg.transaction_timeout_ms = get_i32(buf)?;
        }
        if version >= 3 {
            msg.producer_id = ProducerId(get_i64(buf)?);
        }
        if version >= 3 {
            msg.producer_epoch = get_i16(buf)?;
        }
        if version >= 6 {
            msg.enable2_pc = get_bool(buf)?;
        }
        if version >= 6 {
            msg.keep_prepared_txn = get_bool(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for InitProducerIdRequest {
    const API_KEY: i16 = 22;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 6;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for InitProducerIdRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
