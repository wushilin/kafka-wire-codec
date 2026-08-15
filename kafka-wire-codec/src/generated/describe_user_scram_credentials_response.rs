#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct DescribeUserScramCredentialsResult {
    /// The user name.
    pub user: StrBytes,
    /// The user-level error code.
    pub error_code: i16,
    /// The user-level error message, if any.
    pub error_message: Option<StrBytes>,
    /// The mechanism and related information associated with the user's SCRAM credentials.
    pub credential_infos: Vec<CredentialInfo>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeUserScramCredentialsResult {
    fn default() -> Self {
        Self {
            user: StrBytes::new(),
            error_code: 0,
            error_message: Some(StrBytes::new()),
            credential_infos: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeUserScramCredentialsResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.user.as_str());
        }
        {
            size += 2;
        }
        {
            size += compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            { let arr = &self.credential_infos;
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
            put_compact_string(buf, self.user.as_str());
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            { let arr = &self.credential_infos;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeUserScramCredentialsResult::default();
        {
            msg.user = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = get_compact_string(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(CredentialInfo::decode(version, buf)?); }
            msg.credential_infos = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CredentialInfo {
    /// The SCRAM mechanism.
    pub mechanism: i8,
    /// The number of iterations used in the SCRAM credential.
    pub iterations: i32,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CredentialInfo {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 1;
        }
        {
            size += 4;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i8(buf, self.mechanism);
        }
        {
            put_i32(buf, self.iterations);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = CredentialInfo::default();
        {
            msg.mechanism = get_i8(buf)?;
        }
        {
            msg.iterations = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-0.
#[derive(Debug, Clone)]
pub struct DescribeUserScramCredentialsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The message-level error code, 0 except for user authorization or infrastructure issues.
    pub error_code: i16,
    /// The message-level error message, if any.
    pub error_message: Option<StrBytes>,
    /// The results for descriptions, one per user.
    pub results: Vec<DescribeUserScramCredentialsResult>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeUserScramCredentialsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(StrBytes::new()),
            results: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeUserScramCredentialsResponse {
    pub const API_KEY: i16 = 50;
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
            size += 2;
        }
        {
            size += compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            { let arr = &self.results;
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
            put_i16(buf, self.error_code);
        }
        {
            put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str()));
        }
        {
            { let arr = &self.results;
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
        let mut msg = DescribeUserScramCredentialsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = get_compact_string(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeUserScramCredentialsResult::decode(version, buf)?); }
            msg.results = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeUserScramCredentialsResponse {
    const API_KEY: i16 = 50;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeUserScramCredentialsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
