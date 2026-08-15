#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

/// Valid versions: 0-2.
#[derive(Debug, Clone, PartialEq)]
pub struct ListTransactionsRequest {
    /// The transaction states to filter by: if empty, all transactions are returned; if non-empty, then only transactions matching one of the filtered states will be returned.
    pub state_filters: Vec<StrBytes>,
    /// The producerIds to filter by: if empty, all transactions will be returned; if non-empty, only transactions which match one of the filtered producerIds will be returned.
    pub producer_id_filters: Vec<ProducerId>,
    /// Duration (in millis) to filter by: if < 0, all transactions will be returned; otherwise, only transactions running longer than this duration will be returned.
    pub duration_filter: i64,
    /// The transactional ID regular expression pattern to filter by: if it is empty or null, all transactions are returned; Otherwise then only the transactions matching the given regular expression will be returned.
    pub transactional_id_pattern: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ListTransactionsRequest {
    fn default() -> Self {
        Self {
            state_filters: Vec::new(),
            producer_id_filters: Vec::new(),
            duration_filter: -1,
            transactional_id_pattern: None,
            tagged_fields: Vec::new(),
        }
    }
}

impl ListTransactionsRequest {
    pub const API_KEY: i16 = 66;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 2;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            { let arr = &self.state_filters;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
                }
            }
        }
        {
            { let arr = &self.producer_id_filters;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 8;
            }
        }
        if version >= 1 {
            size += 8;
        }
        if version >= 2 {
            size += if version >= 2 { compact_nullable_string_size(self.transactional_id_pattern.as_ref().map(|v| v.as_str())) } else { let v = self.transactional_id_pattern.as_ref().expect("field transactional_id_pattern is None but not nullable at this version"); compact_string_size(v.as_str()) };
        }
        size += tagged_fields_size(&self.tagged_fields);
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            { let arr = &self.state_filters;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        {
            { let arr = &self.producer_id_filters;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i64(buf, item.0); }
            }
        }
        if version >= 1 {
            put_i64(buf, self.duration_filter);
        }
        if version >= 2 {
            if version >= 2 { put_compact_nullable_string(buf, self.transactional_id_pattern.as_ref().map(|v| v.as_str())) } else { let v = self.transactional_id_pattern.as_ref().expect("field transactional_id_pattern is None but not nullable at this version"); put_compact_string(buf, v.as_str()) };
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ListTransactionsRequest::default();
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.state_filters = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_i64(buf)).map(ProducerId)?); }
            msg.producer_id_filters = items; }
        }
        if version >= 1 {
            msg.duration_filter = get_i64(buf)?;
        }
        if version >= 2 {
            msg.transactional_id_pattern = { let v = get_compact_string(buf)?; if version >= 2 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for ListTransactionsRequest {
    const API_KEY: i16 = 66;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ListTransactionsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
