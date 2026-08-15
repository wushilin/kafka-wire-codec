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

/// Valid versions: 0-1.
#[derive(Debug, Clone, PartialEq)]
pub struct AddRaftVoterRequest {
    /// The cluster id.
    pub cluster_id: Option<StrBytes>,
    /// The maximum time to wait for the request to complete before returning.
    pub timeout_ms: i32,
    /// The replica id of the voter getting added to the topic partition.
    pub voter_id: i32,
    /// The directory id of the voter getting added to the topic partition.
    pub voter_directory_id: Uuid,
    /// The endpoints that can be used to communicate with the voter.
    pub listeners: Vec<Listener>,
    /// When true, return a response after the new voter set is committed. Otherwise, return after the leader writes the changes locally.
    pub ack_when_committed: bool,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for AddRaftVoterRequest {
    fn default() -> Self {
        Self {
            cluster_id: Some(StrBytes::new()),
            timeout_ms: 0,
            voter_id: 0,
            voter_directory_id: Uuid::nil(),
            listeners: Vec::new(),
            ack_when_committed: true,
            tagged_fields: Vec::new(),
        }
    }
}

impl AddRaftVoterRequest {
    pub const API_KEY: i16 = 80;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 1;
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
        if version >= 1 {
            size += 1;
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
            put_i32(buf, self.timeout_ms);
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
        if version >= 1 {
            put_bool(buf, self.ack_when_committed);
        }
        put_tagged_fields(buf, &self.tagged_fields);
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = AddRaftVoterRequest::default();
        {
            msg.cluster_id = get_compact_string(buf)?;
        }
        {
            msg.timeout_ms = get_i32(buf)?;
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
        if version >= 1 {
            msg.ack_when_committed = get_bool(buf)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for AddRaftVoterRequest {
    const API_KEY: i16 = 80;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 1;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for AddRaftVoterRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
