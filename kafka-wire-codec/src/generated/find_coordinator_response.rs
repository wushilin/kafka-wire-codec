#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinator {
    /// The coordinator key.
    pub key: StrBytes,
    /// The node id.
    pub node_id: BrokerId,
    /// The host name.
    pub host: StrBytes,
    /// The port.
    pub port: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for Coordinator {
    fn default() -> Self {
        Self {
            key: StrBytes::new(),
            node_id: BrokerId::default(),
            host: StrBytes::new(),
            port: 0,
            error_code: 0,
            error_message: Some(StrBytes::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl Coordinator {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 4 {
            size += if version >= 3 { compact_string_size(self.key.as_str()) } else { string_size(self.key.as_str()) };
        }
        if version >= 4 {
            size += 4;
        }
        if version >= 4 {
            size += if version >= 3 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        if version >= 4 {
            size += 4;
        }
        if version >= 4 {
            size += 2;
        }
        if version >= 4 {
            size += if version >= 4 { if version >= 3 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 3 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 4 {
            if version >= 3 { put_compact_string(buf, self.key.as_str()) } else { put_string(buf, self.key.as_str()) };
        }
        if version >= 4 {
            put_i32(buf, self.node_id.0);
        }
        if version >= 4 {
            if version >= 3 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        if version >= 4 {
            put_i32(buf, self.port);
        }
        if version >= 4 {
            put_i16(buf, self.error_code);
        }
        if version >= 4 {
            if version >= 4 { if version >= 3 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 3 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Coordinator::default();
        if version >= 4 {
            msg.key = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 4 {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        if version >= 4 {
            msg.host = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 4 {
            msg.port = get_i32(buf)?;
        }
        if version >= 4 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 4 {
            msg.error_message = { let v = if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 4 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-6.
#[derive(Debug, Clone, PartialEq)]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// The node id.
    pub node_id: BrokerId,
    /// The host name.
    pub host: StrBytes,
    /// The port.
    pub port: i32,
    /// Each coordinator result in the response.
    pub coordinators: Vec<Coordinator>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for FindCoordinatorResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(StrBytes::new()),
            node_id: BrokerId::default(),
            host: StrBytes::new(),
            port: 0,
            coordinators: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl FindCoordinatorResponse {
    pub const API_KEY: i16 = 10;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 6;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        if version <= 3 {
            size += 2;
        }
        if version >= 1 && version <= 3 {
            size += if version >= 1 && version <= 3 { if version >= 3 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 3 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version <= 3 {
            size += 4;
        }
        if version <= 3 {
            size += if version >= 3 { compact_string_size(self.host.as_str()) } else { string_size(self.host.as_str()) };
        }
        if version <= 3 {
            size += 4;
        }
        if version >= 4 {
            { let arr = &self.coordinators;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        if version <= 3 {
            put_i16(buf, self.error_code);
        }
        if version >= 1 && version <= 3 {
            if version >= 1 && version <= 3 { if version >= 3 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 3 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version <= 3 {
            put_i32(buf, self.node_id.0);
        }
        if version <= 3 {
            if version >= 3 { put_compact_string(buf, self.host.as_str()) } else { put_string(buf, self.host.as_str()) };
        }
        if version <= 3 {
            put_i32(buf, self.port);
        }
        if version >= 4 {
            { let arr = &self.coordinators;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = FindCoordinatorResponse::default();
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version <= 3 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 1 && version <= 3 {
            msg.error_message = { let v = if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 1 && version <= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version <= 3 {
            msg.node_id = BrokerId(get_i32(buf)?);
        }
        if version <= 3 {
            msg.host = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 3 {
            msg.port = get_i32(buf)?;
        }
        if version >= 4 {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Coordinator::decode(version, buf)?); }
            msg.coordinators = items; }
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for FindCoordinatorResponse {
    const API_KEY: i16 = 10;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 6;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for FindCoordinatorResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
