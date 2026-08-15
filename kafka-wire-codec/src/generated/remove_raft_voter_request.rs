#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveRaftVoterRequest {
    /// The cluster id of the request.
    pub cluster_id: Option<StrBytes>,
    /// The replica id of the voter getting removed from the topic partition.
    pub voter_id: i32,
    /// The directory id of the voter getting removed from the topic partition.
    pub voter_directory_id: Uuid,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for RemoveRaftVoterRequest {
    fn default() -> Self {
        Self {
            cluster_id: Some(StrBytes::new()),
            voter_id: 0,
            voter_directory_id: Uuid::nil(),
            tagged_fields: Vec::new(),
        }
    }
}

impl RemoveRaftVoterRequest {
    pub const API_KEY: i16 = 81;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += 4;
        }
        {
            size += 16;
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_compact_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.voter_id);
        }
        {
            put_uuid(buf, &self.voter_directory_id);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = RemoveRaftVoterRequest::default();
        {
            msg.cluster_id = get_compact_string(buf)?;
        }
        {
            msg.voter_id = get_i32(buf)?;
        }
        {
            msg.voter_directory_id = get_uuid(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for RemoveRaftVoterRequest {
    const API_KEY: i16 = 81;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for RemoveRaftVoterRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
