#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct Listener {
    /// The name of the endpoint.
    pub name: Bytes,
    /// The hostname.
    pub host: Bytes,
    /// The port.
    pub port: u16,
    /// The security protocol.
    pub security_protocol: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Listener {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(&self.name);
        }
        {
            size += compact_string_size(&self.host);
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
            put_compact_string(buf, &self.name);
        }
        {
            put_compact_string(buf, &self.host);
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

#[derive(Debug, Clone, Default)]
pub struct Feature {
    /// The feature name.
    pub name: Bytes,
    /// The minimum supported feature level.
    pub min_supported_version: i16,
    /// The maximum supported feature level.
    pub max_supported_version: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Feature {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += compact_string_size(&self.name);
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
            put_compact_string(buf, &self.name);
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
#[derive(Debug, Clone)]
pub struct BrokerRegistrationRequest {
    /// The broker ID.
    pub broker_id: i32,
    /// The cluster id of the broker process.
    pub cluster_id: Bytes,
    /// The incarnation id of the broker process.
    pub incarnation_id: [u8; 16],
    /// The listeners of this broker.
    pub listeners: Vec<Listener>,
    /// The features on this broker. Note: in v0-v3, features with MinSupportedVersion = 0 are omitted.
    pub features: Vec<Feature>,
    /// The rack which this broker is in.
    pub rack: Option<Bytes>,
    /// If the required configurations for ZK migration are present, this value is set to true.
    pub is_migrating_zk_broker: bool,
    /// Log directories configured in this broker which are available.
    pub log_dirs: Vec<[u8; 16]>,
    /// The epoch before a clean shutdown.
    pub previous_broker_epoch: i64,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for BrokerRegistrationRequest {
    fn default() -> Self {
        Self {
            broker_id: 0,
            cluster_id: Bytes::new(),
            incarnation_id: [0u8; 16],
            listeners: Vec::new(),
            features: Vec::new(),
            rack: Some(Bytes::new()),
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

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += compact_string_size(&self.cluster_id);
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
            size += compact_nullable_string_size(self.rack.as_deref());
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
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.broker_id);
        }
        {
            put_compact_string(buf, &self.cluster_id);
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
            put_compact_nullable_string(buf, self.rack.as_deref());
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
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = BrokerRegistrationRequest::default();
        {
            msg.broker_id = get_i32(buf)?;
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
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for BrokerRegistrationRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
