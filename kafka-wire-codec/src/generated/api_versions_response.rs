#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ApiVersion {
    /// The API index.
    pub api_key: i16,
    /// The minimum supported version, inclusive.
    pub min_version: i16,
    /// The maximum supported version, inclusive.
    pub max_version: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl ApiVersion {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += 2;
        }
        {
            size += 2;
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.api_key);
        }
        {
            put_i16(buf, self.min_version);
        }
        {
            put_i16(buf, self.max_version);
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = ApiVersion::default();
        {
            msg.api_key = get_i16(buf)?;
        }
        {
            msg.min_version = get_i16(buf)?;
        }
        {
            msg.max_version = get_i16(buf)?;
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SupportedFeatureKey {
    /// The name of the feature.
    pub name: StrBytes,
    /// The minimum supported version for the feature.
    pub min_version: i16,
    /// The maximum supported version for the feature.
    pub max_version: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl SupportedFeatureKey {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 3 {
            size += 2;
        }
        if version >= 3 {
            size += 2;
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        if version >= 3 {
            put_i16(buf, self.min_version);
        }
        if version >= 3 {
            put_i16(buf, self.max_version);
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = SupportedFeatureKey::default();
        if version >= 3 {
            msg.name = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.min_version = get_i16(buf)?;
        }
        if version >= 3 {
            msg.max_version = get_i16(buf)?;
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FinalizedFeatureKey {
    /// The name of the feature.
    pub name: StrBytes,
    /// The cluster-wide finalized max version level for the feature.
    pub max_version_level: i16,
    /// The cluster-wide finalized min version level for the feature.
    pub min_version_level: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl FinalizedFeatureKey {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        if version >= 3 {
            size += 2;
        }
        if version >= 3 {
            size += 2;
        }
        if version >= 3 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 3 {
            if version >= 3 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        if version >= 3 {
            put_i16(buf, self.max_version_level);
        }
        if version >= 3 {
            put_i16(buf, self.min_version_level);
        }
        if version >= 3 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = FinalizedFeatureKey::default();
        if version >= 3 {
            msg.name = (if version >= 3 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.max_version_level = get_i16(buf)?;
        }
        if version >= 3 {
            msg.min_version_level = get_i16(buf)?;
        }
        if version >= 3 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-4.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    pub error_code: i16,
    /// The APIs supported by the broker.
    pub api_keys: Vec<ApiVersion>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Features supported by the broker. Note: in v0-v3, features with MinSupportedVersion = 0 are omitted.
    /// Tagged field (tag 0, versions 3+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub supported_features: Vec<SupportedFeatureKey>,
    /// The monotonically increasing epoch for the finalized features information. Valid values are >= 0. A value of -1 is special and represents unknown epoch.
    /// Tagged field (tag 1, versions 3+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub finalized_features_epoch: i64,
    /// List of cluster-wide finalized features. The information is valid only if FinalizedFeaturesEpoch >= 0.
    /// Tagged field (tag 2, versions 3+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub finalized_features: Vec<FinalizedFeatureKey>,
    /// Set by a KRaft controller if the required configurations for ZK migration are present.
    /// Tagged field (tag 3, versions 3+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub zk_migration_ready: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for ApiVersionsResponse {
    fn default() -> Self {
        Self {
            error_code: 0,
            api_keys: Vec::new(),
            throttle_time_ms: 0,
            supported_features: Vec::new(),
            finalized_features_epoch: -1,
            finalized_features: Vec::new(),
            zk_migration_ready: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl ApiVersionsResponse {
    pub const API_KEY: i16 = 18;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 3;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            { let arr = &self.api_keys;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 {
            size += 4;
        }
        if version >= 3 { {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 3 && (!self.supported_features.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.supported_features;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 3 && (self.finalized_features_epoch != -1) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += 8;
                size };
                known_tagged_size += uvarint_size(1u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 3 && (!self.finalized_features.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.finalized_features;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                known_tagged_size += uvarint_size(2u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 3 && (self.zk_migration_ready) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            size += 1;
                size };
                known_tagged_size += uvarint_size(3u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        } }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.api_keys;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 3 { {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 3 && (!self.supported_features.is_empty()) { num_tagged += 1; }
            if version >= 3 && (self.finalized_features_epoch != -1) { num_tagged += 1; }
            if version >= 3 && (!self.finalized_features.is_empty()) { num_tagged += 1; }
            if version >= 3 && (self.zk_migration_ready) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            let mut raw_it = self.tagged_fields.iter().peekable();
            if version >= 3 && (!self.supported_features.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.supported_features;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.supported_features;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
            }
            if version >= 3 && (self.finalized_features_epoch != -1) {
                while let Some((t, d)) = raw_it.peek() { if *t < 1 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 1u64);
                let data_len = { let mut size = 0usize;
            size += 8;
                size };
                put_uvarint(buf, data_len as u64);
            put_i64(buf, self.finalized_features_epoch);
            }
            if version >= 3 && (!self.finalized_features.is_empty()) {
                while let Some((t, d)) = raw_it.peek() { if *t < 2 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 2u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.finalized_features;
                if version >= 3 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.finalized_features;
                if version >= 3 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
            }
            if version >= 3 && (self.zk_migration_ready) {
                while let Some((t, d)) = raw_it.peek() { if *t < 3 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 3u64);
                let data_len = { let mut size = 0usize;
            size += 1;
                size };
                put_uvarint(buf, data_len as u64);
            put_bool(buf, self.zk_migration_ready);
            }
            for (t, d) in raw_it { put_raw_tagged_field(buf, *t, d); }
        } }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = ApiVersionsResponse::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(ApiVersion::decode(version, buf)?); }
            msg.api_keys = items; }
        }
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 3 { {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 3 => {
                        let buf = &mut data;
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(SupportedFeatureKey::decode(version, buf)?); }
            msg.supported_features = items; }
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    1 if version >= 3 => {
                        let buf = &mut data;
            msg.finalized_features_epoch = get_i64(buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    2 if version >= 3 => {
                        let buf = &mut data;
            let len_opt = if version >= 3 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FinalizedFeatureKey::decode(version, buf)?); }
            msg.finalized_features = items; }
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    3 if version >= 3 => {
                        let buf = &mut data;
            msg.zk_migration_ready = get_bool(buf)?;
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        } }
        Ok(msg)
    }
}

impl crate::Encodable for ApiVersionsResponse {
    const API_KEY: i16 = 18;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 3;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for ApiVersionsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
