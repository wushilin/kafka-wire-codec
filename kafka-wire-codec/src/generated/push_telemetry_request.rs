#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-0.
#[derive(Debug, Clone, Default)]
pub struct PushTelemetryRequest {
    /// Unique id for this client instance.
    pub client_instance_id: Uuid,
    /// Unique identifier for the current subscription.
    pub subscription_id: i32,
    /// Client is terminating the connection.
    pub terminating: bool,
    /// Compression codec used to compress the metrics.
    pub compression_type: i8,
    /// Metrics encoded in OpenTelemetry MetricsData v1 protobuf format.
    pub metrics: Bytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl PushTelemetryRequest {
    pub const API_KEY: i16 = 72;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 16;
        }
        {
            size += 4;
        }
        {
            size += 1;
        }
        {
            size += 1;
        }
        {
            size += compact_bytes_size(&self.metrics);
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_uuid(buf, &self.client_instance_id);
        }
        {
            put_i32(buf, self.subscription_id);
        }
        {
            put_bool(buf, self.terminating);
        }
        {
            put_i8(buf, self.compression_type);
        }
        {
            put_compact_bytes_zc(buf, &self.metrics);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = PushTelemetryRequest::default();
        {
            msg.client_instance_id = get_uuid(buf)?;
        }
        {
            msg.subscription_id = get_i32(buf)?;
        }
        {
            msg.terminating = get_bool(buf)?;
        }
        {
            msg.compression_type = get_i8(buf)?;
        }
        {
            msg.metrics = (get_compact_bytes(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for PushTelemetryRequest {
    const API_KEY: i16 = 72;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for PushTelemetryRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
