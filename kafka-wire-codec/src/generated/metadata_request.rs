#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MetadataRequestTopic {
    /// The topic id.
    pub topic_id: Uuid,
    /// The topic name.
    pub name: Option<TopicName>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        Self {
            topic_id: Uuid::nil(),
            name: Some(TopicName::default()),
            tagged_fields: Vec::new(),
        }
    }
}

impl MetadataRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 10 {
            size += 16;
        }
        {
            size += if version >= 10 { if version >= 9 { compact_nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 9 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        {
            if version >= 10 { if version >= 9 { put_compact_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.name.as_ref().map(|v| v.as_str())) } } else { let v = self.name.as_ref().expect("field name is None but not nullable at this version"); if version >= 9 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MetadataRequestTopic::default();
        if version >= 10 {
            msg.topic_id = get_uuid(buf)?;
        }
        {
            msg.name = { let v = if version >= 9 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 10 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } }.map(TopicName);
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-13.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    pub topics: Option<Vec<MetadataRequestTopic>>,
    /// If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
    pub allow_auto_topic_creation: bool,
    /// Whether to include cluster authorized operations.
    pub include_cluster_authorized_operations: bool,
    /// Whether to include topic authorized operations.
    pub include_topic_authorized_operations: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for MetadataRequest {
    fn default() -> Self {
        Self {
            topics: Some(Vec::new()),
            allow_auto_topic_creation: true,
            include_cluster_authorized_operations: false,
            include_topic_authorized_operations: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl MetadataRequest {
    pub const API_KEY: i16 = 3;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 13;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 9;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            match &self.topics {
                Some(arr) => {
                if version >= 9 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    assert!(version >= 1, "field topics is None but not nullable at version {}", version);
                    if version >= 9 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 4 {
            size += 1;
        }
        if version >= 8 && version <= 10 {
            size += 1;
        }
        if version >= 8 {
            size += 1;
        }
        if version >= 9 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            match &self.topics {
                Some(arr) => {
                if version >= 9 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    assert!(version >= 1, "field topics is None but not nullable at version {}", version);
                    if version >= 9 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 4 {
            put_bool(buf, self.allow_auto_topic_creation);
        }
        if version >= 8 && version <= 10 {
            put_bool(buf, self.include_cluster_authorized_operations);
        }
        if version >= 8 {
            put_bool(buf, self.include_topic_authorized_operations);
        }
        if version >= 9 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = MetadataRequest::default();
        {
            let len_opt = if version >= 9 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.topics = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(MetadataRequestTopic::decode(version, buf)?); }
                Some(items)
                }
                None => { if version >= 1 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
        }
        if version >= 4 {
            msg.allow_auto_topic_creation = get_bool(buf)?;
        }
        if version >= 8 && version <= 10 {
            msg.include_cluster_authorized_operations = get_bool(buf)?;
        }
        if version >= 8 {
            msg.include_topic_authorized_operations = get_bool(buf)?;
        }
        if version >= 9 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for MetadataRequest {
    const API_KEY: i16 = 3;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 13;
    const FLEXIBLE_MIN_VERSION: i16 = 9;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for MetadataRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
