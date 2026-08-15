#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

/// Valid versions: 0-2.
#[derive(Debug, Clone, PartialEq)]
pub struct SaslAuthenticateResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    pub auth_bytes: Bytes,
    /// Number of milliseconds after which only re-authentication over the existing connection to create a new session can occur.
    pub session_lifetime_ms: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for SaslAuthenticateResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: Some(StrBytes::new()),
            auth_bytes: Bytes::new(),
            session_lifetime_ms: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl SaslAuthenticateResponse {
    pub const API_KEY: i16 = 36;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 2;
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
            size += if version >= 2 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            size += if version >= 2 { compact_bytes_size(&self.auth_bytes) } else { bytes_size(&self.auth_bytes) };
        }
        if version >= 1 {
            size += 8;
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
            if version >= 2 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) };
        }
        {
            if version >= 2 { put_compact_bytes_zc(buf, &self.auth_bytes) } else { put_bytes_zc(buf, &self.auth_bytes) };
        }
        if version >= 1 {
            put_i64(buf, self.session_lifetime_ms);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = SaslAuthenticateResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? };
        }
        {
            msg.auth_bytes = (if version >= 2 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 1 {
            msg.session_lifetime_ms = get_i64(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for SaslAuthenticateResponse {
    const API_KEY: i16 = 36;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for SaslAuthenticateResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
