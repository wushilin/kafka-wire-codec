#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeClusterBroker {
    /// The broker ID.
    pub broker_id: BrokerId,
    /// The broker hostname.
    pub host: StrBytes,
    /// The broker port.
    pub port: i32,
    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<StrBytes>,
    /// Whether the broker is fenced
    pub is_fenced: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeClusterBroker {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += compact_string_size(self.host.as_str());
        }
        {
            size += 4;
        }
        {
            size += compact_nullable_string_size(self.rack.as_ref().map(|v| v.as_str()));
        }
        if version >= 2 {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.broker_id.0);
        }
        {
            put_compact_string(buf, self.host.as_str());
        }
        {
            put_i32(buf, self.port);
        }
        {
            put_compact_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str()));
        }
        if version >= 2 {
            put_bool(buf, self.is_fenced);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeClusterBroker::default();
        {
            msg.broker_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.port = get_i32(buf)?;
        }
        {
            msg.rack = get_compact_string(buf)?;
        }
        if version >= 2 {
            msg.is_fenced = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-2.
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeClusterResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// The endpoint type that was described. 1=brokers, 2=controllers.
    pub endpoint_type: i8,
    /// The cluster ID that responding broker belongs to.
    pub cluster_id: StrBytes,
    /// The ID of the controller. When handled by a controller, returns the current voter leader ID. When handled by a broker, returns a random alive broker ID as a fallback.
    pub controller_id: BrokerId,
    /// Each broker in the response.
    pub brokers: Vec<DescribeClusterBroker>,
    /// 32-bit bitfield to represent authorized operations for this cluster.
    pub cluster_authorized_operations: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeClusterResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: None,
            endpoint_type: 1,
            cluster_id: StrBytes::new(),
            controller_id: BrokerId(-1),
            brokers: Vec::new(),
            cluster_authorized_operations: -2147483648,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClusterResponse {
    pub const API_KEY: i16 = 60;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 2;
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
        if version >= 1 {
            size += 1;
        }
        {
            size += compact_string_size(self.cluster_id.as_str());
        }
        {
            size += 4;
        }
        {
            { let arr = &self.brokers;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 4;
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
        if version >= 1 {
            put_i8(buf, self.endpoint_type);
        }
        {
            put_compact_string(buf, self.cluster_id.as_str());
        }
        {
            put_i32(buf, self.controller_id.0);
        }
        {
            { let arr = &self.brokers;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_i32(buf, self.cluster_authorized_operations);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeClusterResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.error_message = get_compact_string(buf)?;
        }
        if version >= 1 {
            msg.endpoint_type = get_i8(buf)?;
        }
        {
            msg.cluster_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.controller_id = BrokerId(get_i32(buf)?);
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeClusterBroker::decode(version, buf)?); }
            msg.brokers = items; }
        }
        {
            msg.cluster_authorized_operations = get_i32(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeClusterResponse {
    const API_KEY: i16 = 60;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeClusterResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
