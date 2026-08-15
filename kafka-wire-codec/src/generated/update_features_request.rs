#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureUpdateKey {
    /// The name of the finalized feature to be updated.
    pub feature: StrBytes,
    /// The new maximum version level for the finalized feature. A value >= 1 is valid. A value < 1, is special, and can be used to request the deletion of the finalized feature.
    pub max_version_level: i16,
    /// DEPRECATED in version 1 (see DowngradeType). When set to true, the finalized feature version level is allowed to be downgraded/deleted. The downgrade request will fail if the new maximum version level is a value that's not lower than the existing maximum finalized version level.
    pub allow_downgrade: bool,
    /// Determine which type of upgrade will be performed: 1 will perform an upgrade only (default), 2 is safe downgrades only (lossless), 3 is unsafe downgrades (lossy).
    pub upgrade_type: i8,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for FeatureUpdateKey {
    fn default() -> Self {
        Self {
            feature: StrBytes::new(),
            max_version_level: 0,
            allow_downgrade: false,
            upgrade_type: 1,
            tagged_fields: Vec::new(),
        }
    }
}

impl FeatureUpdateKey {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.feature.as_str());
        }
        {
            size += 2;
        }
        if version <= 0 {
            size += 1;
        }
        if version >= 1 {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.feature.as_str());
        }
        {
            put_i16(buf, self.max_version_level);
        }
        if version <= 0 {
            put_bool(buf, self.allow_downgrade);
        }
        if version >= 1 {
            put_i8(buf, self.upgrade_type);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = FeatureUpdateKey::default();
        {
            msg.feature = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.max_version_level = get_i16(buf)?;
        }
        if version <= 0 {
            msg.allow_downgrade = get_bool(buf)?;
        }
        if version >= 1 {
            msg.upgrade_type = get_i8(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-2.
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateFeaturesRequest {
    /// How long to wait in milliseconds before timing out the request.
    pub timeout_ms: i32,
    /// The list of updates to finalized features.
    pub feature_updates: Vec<FeatureUpdateKey>,
    /// True if we should validate the request, but not perform the upgrade or downgrade.
    pub validate_only: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for UpdateFeaturesRequest {
    fn default() -> Self {
        Self {
            timeout_ms: 60000,
            feature_updates: Vec::new(),
            validate_only: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl UpdateFeaturesRequest {
    pub const API_KEY: i16 = 57;
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
            { let arr = &self.feature_updates;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 1 {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.timeout_ms);
        }
        {
            { let arr = &self.feature_updates;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 1 {
            put_bool(buf, self.validate_only);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = UpdateFeaturesRequest::default();
        {
            msg.timeout_ms = get_i32(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(FeatureUpdateKey::decode(version, buf)?); }
            msg.feature_updates = items; }
        }
        if version >= 1 {
            msg.validate_only = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for UpdateFeaturesRequest {
    const API_KEY: i16 = 57;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for UpdateFeaturesRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
