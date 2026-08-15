#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
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
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KRaftVersionFeature {
    /// The minimum supported KRaft protocol version.
    pub min_supported_version: i16,
    /// The maximum supported KRaft protocol version.
    pub max_supported_version: i16,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl KRaftVersionFeature {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
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
            put_i16(buf, self.min_supported_version);
        }
        {
            put_i16(buf, self.max_supported_version);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = KRaftVersionFeature::default();
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

/// Valid versions: 0-0.
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRaftVoterRequest {
    /// The cluster id.
    pub cluster_id: Option<StrBytes>,
    /// The current leader epoch of the partition, -1 for unknown leader epoch.
    pub current_leader_epoch: i32,
    /// The replica id of the voter getting updated in the topic partition.
    pub voter_id: i32,
    /// The directory id of the voter getting updated in the topic partition.
    pub voter_directory_id: Uuid,
    /// The endpoint that can be used to communicate with the leader.
    pub listeners: Vec<Listener>,
    /// The range of versions of the protocol that the replica supports.
    pub k_raft_version_feature: KRaftVersionFeature,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for UpdateRaftVoterRequest {
    fn default() -> Self {
        Self {
            cluster_id: Some(StrBytes::new()),
            current_leader_epoch: 0,
            voter_id: 0,
            voter_directory_id: Uuid::nil(),
            listeners: Vec::new(),
            k_raft_version_feature: KRaftVersionFeature::default(),
            tagged_fields: Vec::new(),
        }
    }
}

impl UpdateRaftVoterRequest {
    pub const API_KEY: i16 = 82;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += compact_nullable_string_size(self.cluster_id.as_ref().map(|v| v.as_str()));
        }
        {
            size += 4;
        }
        {
            size += 4;
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
            size += self.k_raft_version_feature.encoded_size(version);
        }
        size += tagged_fields_size(&self.tagged_fields);
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            put_compact_nullable_string(buf, self.cluster_id.as_ref().map(|v| v.as_str()));
        }
        {
            put_i32(buf, self.current_leader_epoch);
        }
        {
            put_i32(buf, self.voter_id);
        }
        {
            put_uuid(buf, &self.voter_directory_id);
        }
        {
            { let arr = &self.listeners;
                put_uvarint(buf, arr.len() as u64 + 1);
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            self.k_raft_version_feature.encode(version, buf);
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = UpdateRaftVoterRequest::default();
        {
            msg.cluster_id = get_compact_string(buf)?;
        }
        {
            msg.current_leader_epoch = get_i32(buf)?;
        }
        {
            msg.voter_id = get_i32(buf)?;
        }
        {
            msg.voter_directory_id = get_uuid(buf)?;
        }
        {
            let len_opt = { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(Listener::decode(version, buf)?); }
            msg.listeners = items; }
        }
        {
            msg.k_raft_version_feature = KRaftVersionFeature::decode(version, buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for UpdateRaftVoterRequest {
    const API_KEY: i16 = 82;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for UpdateRaftVoterRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
