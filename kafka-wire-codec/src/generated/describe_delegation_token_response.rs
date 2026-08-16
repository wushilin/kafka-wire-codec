#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::chain::*;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribedDelegationToken {
    /// The token principal type.
    pub principal_type: StrBytes,
    /// The token principal name.
    pub principal_name: StrBytes,
    /// The principal type of the requester of the token.
    pub token_requester_principal_type: StrBytes,
    /// The principal type of the requester of the token.
    pub token_requester_principal_name: StrBytes,
    /// The token issue timestamp in milliseconds.
    pub issue_timestamp: i64,
    /// The token expiry timestamp in milliseconds.
    pub expiry_timestamp: i64,
    /// The token maximum timestamp length in milliseconds.
    pub max_timestamp: i64,
    /// The token ID.
    pub token_id: StrBytes,
    /// The token HMAC.
    pub hmac: Bytes,
    /// Those who are able to renew this token before it expires.
    pub renewers: Vec<DescribedDelegationTokenRenewer>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribedDelegationToken {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.principal_type.as_str()) } else { string_size(self.principal_type.as_str()) };
        }
        {
            size += if version >= 2 { compact_string_size(self.principal_name.as_str()) } else { string_size(self.principal_name.as_str()) };
        }
        if version >= 3 {
            size += if version >= 2 { compact_string_size(self.token_requester_principal_type.as_str()) } else { string_size(self.token_requester_principal_type.as_str()) };
        }
        if version >= 3 {
            size += if version >= 2 { compact_string_size(self.token_requester_principal_name.as_str()) } else { string_size(self.token_requester_principal_name.as_str()) };
        }
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += if version >= 2 { compact_string_size(self.token_id.as_str()) } else { string_size(self.token_id.as_str()) };
        }
        {
            size += if version >= 2 { compact_bytes_size(&self.hmac) } else { bytes_size(&self.hmac) };
        }
        {
            { let arr = &self.renewers;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.principal_type.as_str()) } else { put_string(buf, self.principal_type.as_str()) };
        }
        {
            if version >= 2 { put_compact_string(buf, self.principal_name.as_str()) } else { put_string(buf, self.principal_name.as_str()) };
        }
        if version >= 3 {
            if version >= 2 { put_compact_string(buf, self.token_requester_principal_type.as_str()) } else { put_string(buf, self.token_requester_principal_type.as_str()) };
        }
        if version >= 3 {
            if version >= 2 { put_compact_string(buf, self.token_requester_principal_name.as_str()) } else { put_string(buf, self.token_requester_principal_name.as_str()) };
        }
        {
            put_i64(buf, self.issue_timestamp);
        }
        {
            put_i64(buf, self.expiry_timestamp);
        }
        {
            put_i64(buf, self.max_timestamp);
        }
        {
            if version >= 2 { put_compact_string(buf, self.token_id.as_str()) } else { put_string(buf, self.token_id.as_str()) };
        }
        {
            if version >= 2 { put_compact_bytes_zc(buf, &self.hmac) } else { put_bytes_zc(buf, &self.hmac) };
        }
        {
            { let arr = &self.renewers;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribedDelegationToken::default();
        {
            msg.principal_type = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.principal_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.token_requester_principal_type = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.token_requester_principal_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.issue_timestamp = get_i64(buf)?;
        }
        {
            msg.expiry_timestamp = get_i64(buf)?;
        }
        {
            msg.max_timestamp = get_i64(buf)?;
        }
        {
            msg.token_id = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.hmac = (if version >= 2 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribedDelegationTokenRenewer::decode(version, buf)?); }
            msg.renewers = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribedDelegationTokenRenewer {
    /// The renewer principal type.
    pub principal_type: StrBytes,
    /// The renewer principal name.
    pub principal_name: StrBytes,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribedDelegationTokenRenewer {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.principal_type.as_str()) } else { string_size(self.principal_type.as_str()) };
        }
        {
            size += if version >= 2 { compact_string_size(self.principal_name.as_str()) } else { string_size(self.principal_name.as_str()) };
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.principal_type.as_str()) } else { put_string(buf, self.principal_type.as_str()) };
        }
        {
            if version >= 2 { put_compact_string(buf, self.principal_name.as_str()) } else { put_string(buf, self.principal_name.as_str()) };
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribedDelegationTokenRenewer::default();
        {
            msg.principal_type = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.principal_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-3.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The tokens.
    pub tokens: Vec<DescribedDelegationToken>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeDelegationTokenResponse {
    pub const API_KEY: i16 = 41;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 3;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            { let arr = &self.tokens;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 4;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.tokens;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeDelegationTokenResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribedDelegationToken::decode(version, buf)?); }
            msg.tokens = items; }
        }
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeDelegationTokenResponse {
    const API_KEY: i16 = 41;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeDelegationTokenResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
