#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeTransactionsRequest {
    /// Array of transactionalIds to include in describe results. If empty, then no results will be returned.
    pub transactional_ids: Vec<TransactionalId>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeTransactionsRequest {
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
            { let arr = &self.transactional_ids;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item.as_str());
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
            { let arr = &self.transactional_ids;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item.as_str()); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeTransactionsRequest::default();
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))).map(TransactionalId)?); }
            msg.transactional_ids = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeTransactionsRequest {
    const API_KEY: i16 = 65;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeTransactionsRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
