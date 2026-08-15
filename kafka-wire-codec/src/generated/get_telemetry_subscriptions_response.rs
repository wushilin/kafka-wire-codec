#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

/// Valid versions: 0-0.
#[derive(Debug, Clone, Default)]
pub struct GetTelemetrySubscriptionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Assigned client instance id if ClientInstanceId was 0 in the request, else 0.
    pub client_instance_id: [u8; 16],
    /// Unique identifier for the current subscription set for this client instance.
    pub subscription_id: i32,
    /// Compression types that broker accepts for the PushTelemetryRequest.
    pub accepted_compression_types: Vec<i8>,
    /// Configured push interval, which is the lowest configured interval in the current subscription set.
    pub push_interval_ms: i32,
    /// The maximum bytes of binary data the broker accepts in PushTelemetryRequest.
    pub telemetry_max_bytes: i32,
    /// Flag to indicate monotonic/counter metrics are to be emitted as deltas or cumulative values.
    pub delta_temporality: bool,
    /// Requested metrics prefix string match. Empty array: No metrics subscribed, Array[0] empty string: All metrics subscribed.
    pub requested_metrics: Vec<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl GetTelemetrySubscriptionsResponse {
    pub const API_KEY: i16 = 71;
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
            size += 16;
        }
        {
            size += 4;
        }
        {
            { let arr = &self.accepted_compression_types;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len();
            }
        }
        {
            size += 4;
        }
        {
            size += 4;
        }
        {
            size += 1;
        }
        {
            { let arr = &self.requested_metrics;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += compact_string_size(item);
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
            put_uuid(buf, &self.client_instance_id);
        }
        {
            put_i32(buf, self.subscription_id);
        }
        {
            { let arr = &self.accepted_compression_types;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_i8(buf, *item); }
            }
        }
        {
            put_i32(buf, self.push_interval_ms);
        }
        {
            put_i32(buf, self.telemetry_max_bytes);
        }
        {
            put_bool(buf, self.delta_temporality);
        }
        {
            { let arr = &self.requested_metrics;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_compact_string(buf, item); }
            }
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = GetTelemetrySubscriptionsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.client_instance_id = get_uuid(buf)?;
        }
        {
            msg.subscription_id = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i8(buf)?); }
            msg.accepted_compression_types = items; }
        }
        {
            msg.push_interval_ms = get_i32(buf)?;
        }
        {
            msg.telemetry_max_bytes = get_i32(buf)?;
        }
        {
            msg.delta_temporality = get_bool(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push((get_compact_string(buf)).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))?); }
            msg.requested_metrics = items; }
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for GetTelemetrySubscriptionsResponse {
    const API_KEY: i16 = 71;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for GetTelemetrySubscriptionsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
