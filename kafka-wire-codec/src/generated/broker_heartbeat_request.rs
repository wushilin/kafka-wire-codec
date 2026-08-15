#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-2.
#[derive(Debug, Clone, PartialEq)]
pub struct BrokerHeartbeatRequest {
    /// The broker ID.
    pub broker_id: BrokerId,
    /// The broker epoch.
    pub broker_epoch: i64,
    /// The highest metadata offset which the broker has reached.
    pub current_metadata_offset: i64,
    /// True if the broker wants to be fenced, false otherwise.
    pub want_fence: bool,
    /// True if the broker wants to be shut down, false otherwise.
    pub want_shut_down: bool,
    /// Log directories that failed and went offline.
    /// Tagged field (tag 0, versions 1+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub offline_log_dirs: Vec<Uuid>,
    /// List of log directories that are cordoned. This is null before the broker reaches the RECOVERY state.
    /// Tagged field (tag 1, versions 2+): encoded only when it differs from
    /// the schema default; an omitted tag decodes to that default.
    pub cordoned_log_dirs: Option<Vec<Uuid>>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for BrokerHeartbeatRequest {
    fn default() -> Self {
        Self {
            broker_id: BrokerId::default(),
            broker_epoch: -1,
            current_metadata_offset: 0,
            want_fence: false,
            want_shut_down: false,
            offline_log_dirs: Vec::new(),
            cordoned_log_dirs: None,
            tagged_fields: Vec::new(),
        }
    }
}

impl BrokerHeartbeatRequest {
    pub const API_KEY: i16 = 63;
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
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += 1;
        }
        {
            size += 1;
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            let mut known_tagged_size = 0usize;
            if version >= 1 && (!self.offline_log_dirs.is_empty()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            { let arr = &self.offline_log_dirs;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 16;
            }
                size };
                known_tagged_size += uvarint_size(0u64) + uvarint_size(data_len as u64) + data_len;
            }
            if version >= 2 && (self.cordoned_log_dirs.is_some()) {
                num_tagged += 1;
                let data_len = { let mut size = 0usize;
            match &self.cordoned_log_dirs {
                Some(arr) => {
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 16;
                }
                None => {
                    assert!(version >= 2, "field cordoned_log_dirs is None but not nullable at version {}", version);
                    size += 1;
                }
            }
                size };
                known_tagged_size += uvarint_size(1u64) + uvarint_size(data_len as u64) + data_len;
            }
            size += uvarint_size(num_tagged as u64) + known_tagged_size + raw_tagged_fields_size(&self.tagged_fields);
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.broker_id.0);
        }
        {
            put_i64(buf, self.broker_epoch);
        }
        {
            put_i64(buf, self.current_metadata_offset);
        }
        {
            put_bool(buf, self.want_fence);
        }
        {
            put_bool(buf, self.want_shut_down);
        }
        {
            let mut num_tagged = self.tagged_fields.len();
            if version >= 1 && (!self.offline_log_dirs.is_empty()) { num_tagged += 1; }
            if version >= 2 && (self.cordoned_log_dirs.is_some()) { num_tagged += 1; }
            put_uvarint(buf, num_tagged as u64);
            let mut raw_it = self.tagged_fields.iter().peekable();
            if version >= 1 && (!self.offline_log_dirs.is_empty()) {
                put_uvarint(buf, 0u64);
                let data_len = { let mut size = 0usize;
            { let arr = &self.offline_log_dirs;
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 16;
            }
                size };
                put_uvarint(buf, data_len as u64);
            { let arr = &self.offline_log_dirs;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_uuid(buf, item); }
            }
            }
            if version >= 2 && (self.cordoned_log_dirs.is_some()) {
                while let Some((t, d)) = raw_it.peek() { if *t < 1 { put_raw_tagged_field(buf, *t, d); raw_it.next(); } else { break; } }
                put_uvarint(buf, 1u64);
                let data_len = { let mut size = 0usize;
            match &self.cordoned_log_dirs {
                Some(arr) => {
                size += uvarint_size(arr.len() as u64 + 1);
                size += arr.len() * 16;
                }
                None => {
                    assert!(version >= 2, "field cordoned_log_dirs is None but not nullable at version {}", version);
                    size += 1;
                }
            }
                size };
                put_uvarint(buf, data_len as u64);
            match &self.cordoned_log_dirs {
                Some(arr) => {
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { put_uuid(buf, item); }
                }
                None => {
                    assert!(version >= 2, "field cordoned_log_dirs is None but not nullable at version {}", version);
                    put_uvarint(buf, 0);
                }
            }
            }
            for (t, d) in raw_it { put_raw_tagged_field(buf, *t, d); }
        }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = BrokerHeartbeatRequest::default();
        {
            msg.broker_id = BrokerId(get_i32(buf)?);
        }
        {
            msg.broker_epoch = get_i64(buf)?;
        }
        {
            msg.current_metadata_offset = get_i64(buf)?;
        }
        {
            msg.want_fence = get_bool(buf)?;
        }
        {
            msg.want_shut_down = get_bool(buf)?;
        }
        {
            let count = get_uvarint32(buf)? as usize;
            let mut raw: Vec<(u32, Bytes)> = Vec::with_capacity(count.min(buf.len() / 2));
            for _ in 0..count {
                let (tag, mut data) = get_tagged_field(buf)?;
                match tag {
                    0 if version >= 1 => {
                        let buf = &mut data;
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_uuid(buf)?); }
            msg.offline_log_dirs = items; }
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    1 if version >= 2 => {
                        let buf = &mut data;
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            msg.cordoned_log_dirs = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_uuid(buf)?); }
                Some(items)
                }
                None => { if version >= 2 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
                        if !buf.is_empty() { return Err(DecodeError::TrailingBytes { remaining: buf.len() }); }
                    }
                    _ => raw.push((tag, data)),
                }
            }
            msg.tagged_fields = raw;
        }
        Ok(msg)
    }
}

impl crate::Encodable for BrokerHeartbeatRequest {
    const API_KEY: i16 = 63;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for BrokerHeartbeatRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
