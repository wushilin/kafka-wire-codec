#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-2.
#[derive(Debug, Clone)]
pub struct DescribeClusterRequest {
    /// Whether to include cluster authorized operations.
    pub include_cluster_authorized_operations: bool,
    /// The endpoint type to describe. 1=brokers, 2=controllers.
    pub endpoint_type: i8,
    /// Whether to include fenced brokers when listing brokers.
    pub include_fenced_brokers: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeClusterRequest {
    fn default() -> Self {
        Self {
            include_cluster_authorized_operations: false,
            endpoint_type: 1,
            include_fenced_brokers: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClusterRequest {
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
            size += 1;
        }
        if version >= 1 {
            size += 1;
        }
        if version >= 2 {
            size += 1;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_bool(buf, self.include_cluster_authorized_operations);
        }
        if version >= 1 {
            put_i8(buf, self.endpoint_type);
        }
        if version >= 2 {
            put_bool(buf, self.include_fenced_brokers);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeClusterRequest::default();
        {
            msg.include_cluster_authorized_operations = get_bool(buf)?;
        }
        if version >= 1 {
            msg.endpoint_type = get_i8(buf)?;
        }
        if version >= 2 {
            msg.include_fenced_brokers = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for DescribeClusterRequest {
    const API_KEY: i16 = 60;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 2;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeClusterRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
