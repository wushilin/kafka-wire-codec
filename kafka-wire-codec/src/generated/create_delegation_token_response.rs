#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 1-3.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreateDelegationTokenResponse {
    /// The top-level error, or zero if there was no error.
    pub error_code: i16,
    /// The principal type of the token owner.
    pub principal_type: StrBytes,
    /// The name of the token owner.
    pub principal_name: StrBytes,
    /// The principal type of the requester of the token.
    pub token_requester_principal_type: StrBytes,
    /// The principal type of the requester of the token.
    pub token_requester_principal_name: StrBytes,
    /// When this token was generated.
    pub issue_timestamp_ms: i64,
    /// When this token expires.
    pub expiry_timestamp_ms: i64,
    /// The maximum lifetime of this token.
    pub max_timestamp_ms: i64,
    /// The token UUID.
    pub token_id: StrBytes,
    /// HMAC of the delegation token.
    pub hmac: Bytes,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CreateDelegationTokenResponse {
    pub const API_KEY: i16 = 38;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 3;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 2;
        }
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
            size += 4;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i16(buf, self.error_code);
        }
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
            put_i64(buf, self.issue_timestamp_ms);
        }
        {
            put_i64(buf, self.expiry_timestamp_ms);
        }
        {
            put_i64(buf, self.max_timestamp_ms);
        }
        {
            if version >= 2 { put_compact_string(buf, self.token_id.as_str()) } else { put_string(buf, self.token_id.as_str()) };
        }
        {
            if version >= 2 { put_compact_bytes_zc(buf, &self.hmac) } else { put_bytes_zc(buf, &self.hmac) };
        }
        {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = CreateDelegationTokenResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
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
            msg.issue_timestamp_ms = get_i64(buf)?;
        }
        {
            msg.expiry_timestamp_ms = get_i64(buf)?;
        }
        {
            msg.max_timestamp_ms = get_i64(buf)?;
        }
        {
            msg.token_id = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.hmac = (if version >= 2 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for CreateDelegationTokenResponse {
    const API_KEY: i16 = 38;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for CreateDelegationTokenResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
