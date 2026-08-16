#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::chain::*;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Listener {
    /// The name of the endpoint.
    pub name: StrBytes,
    /// The hostname.
    pub host: StrBytes,
    /// The port.
    pub port: u16,
    /// The security protocol.
    pub security_protocol: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Listener {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
        }
        {
            size += compact_string_size(self.host.as_str());
        }
        {
            size += 2;
        }
        {
            size += 2;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.name.as_str());
        }
        {
            put_compact_string(buf, self.host.as_str());
        }
        {
            put_u16(buf, self.port);
        }
        {
            put_i16(buf, self.security_protocol);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Listener::default();
        {
            msg.name = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.host = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.port = get_u16(buf)?;
        }
        {
            msg.security_protocol = get_i16(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Feature {
    /// The feature name.
    pub name: StrBytes,
    /// The minimum supported feature level.
    pub min_supported_version: i16,
    /// The maximum supported feature level.
    pub max_supported_version: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Feature {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(self.name.as_str());
        }
        {
            size += 2;
        }
        {
            size += 2;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_compact_string(buf, self.name.as_str());
        }
        {
            put_i16(buf, self.min_supported_version);
        }
        {
            put_i16(buf, self.max_supported_version);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = Feature::default();
        {
            msg.name = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.min_supported_version = get_i16(buf)?;
        }
        {
            msg.max_supported_version = get_i16(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

/// Valid versions: 0-4.
#[derive(Debug, Clone, PartialEq)]
pub struct BrokerRegistrationRequest {
    /// The broker ID.
    pub broker_id: BrokerId,
    /// The cluster id of the broker process.
    pub cluster_id: StrBytes,
    /// The incarnation id of the broker process.
    pub incarnation_id: Uuid,
    /// The listeners of this broker.
    pub listeners: Vec<Listener>,
    /// The features on this broker. Note: in v0-v3, features with MinSupportedVersion = 0 are omitted.
    pub features: Vec<Feature>,
    /// The rack which this broker is in.
    pub rack: Option<StrBytes>,
    /// If the required configurations for ZK migration are present, this value is set to true.
    pub is_migrating_zk_broker: bool,
    /// Log directories configured in this broker which are available.
    pub log_dirs: Vec<Uuid>,
    /// The epoch before a clean shutdown.
    pub previous_broker_epoch: i64,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for BrokerRegistrationRequest {
    fn default() -> Self {
        Self {
            broker_id: BrokerId::default(),
            cluster_id: StrBytes::new(),
            incarnation_id: Uuid::nil(),
            listeners: Vec::new(),
            features: Vec::new(),
            rack: Some(StrBytes::new()),
            is_migrating_zk_broker: false,
            log_dirs: Vec::new(),
            previous_broker_epoch: -1,
            tagged_fields: Vec::new(),
        }
    }
}

impl BrokerRegistrationRequest {
    pub const API_KEY: i16 = 62;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 4;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += compact_string_size(self.cluster_id.as_str());
        }
        {
            size += 16;
        }
        {
            { let arr = &self.listeners;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            { let arr = &self.features;
                size += uvarint_size(arr.len() as u64 + 1);
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += compact_nullable_string_size(self.rack.as_ref().map(|v| v.as_str()));
        }
        if version >= 1 {
            size += 1;
        }
        if version >= 2 {
            { let arr = &self.log_dirs;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 16;
            }
        }
        if version >= 3 {
            size += 8;
        }
        size += tagged_fields_size(&self.tagged_fields);
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_i32(buf, self.broker_id.0);
        }
        {
            put_compact_string(buf, self.cluster_id.as_str());
        }
        {
            put_uuid(buf, &self.incarnation_id);
        }
        {
            { let arr = &self.listeners;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            { let arr = &self.features;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_compact_nullable_string(buf, self.rack.as_ref().map(|v| v.as_str()));
        }
        if version >= 1 {
            put_bool(buf, self.is_migrating_zk_broker);
        }
        if version >= 2 {
            { let arr = &self.log_dirs;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_uuid(buf, item); }
            }
        }
        if version >= 3 {
            put_i64(buf, self.previous_broker_epoch);
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = BrokerRegistrationRequest::default();
        {
            msg.broker_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.cluster_id = (get_compact_string(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.incarnation_id = get_uuid(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Listener::decode(version, buf)?); }
            msg.listeners = items; }
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Feature::decode(version, buf)?); }
            msg.features = items; }
        }
        {
            msg.rack = get_compact_string(buf)?;
        }
        if version >= 1 {
            msg.is_migrating_zk_broker = get_bool(buf)?;
        }
        if version >= 2 {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_uuid(buf)?); }
            msg.log_dirs = items; }
        }
        if version >= 3 {
            msg.previous_broker_epoch = get_i64(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for BrokerRegistrationRequest {
    const API_KEY: i16 = 62;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for BrokerRegistrationRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
