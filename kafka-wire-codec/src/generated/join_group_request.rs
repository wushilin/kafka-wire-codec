#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct JoinGroupRequestProtocol {
    /// The protocol name.
    pub name: Bytes,
    /// The protocol metadata.
    pub metadata: Bytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl JoinGroupRequestProtocol {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 6 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        {
            size += if version >= 6 { compact_bytes_size(&self.metadata) } else { bytes_size(&self.metadata) };
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 6 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        {
            if version >= 6 { put_compact_bytes_zc(buf, &self.metadata) } else { put_bytes_zc(buf, &self.metadata) };
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = JoinGroupRequestProtocol::default();
        {
            msg.name = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.metadata = (if version >= 6 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-9.
#[derive(Debug, Clone)]
pub struct JoinGroupRequest {
    /// The group identifier.
    pub group_id: Bytes,
    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
    pub session_timeout_ms: i32,
    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
    pub rebalance_timeout_ms: i32,
    /// The member id assigned by the group coordinator.
    pub member_id: Bytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<Bytes>,
    /// The unique name the for class of protocols implemented by the group we want to join.
    pub protocol_type: Bytes,
    /// The list of protocols that the member supports.
    pub protocols: Vec<JoinGroupRequestProtocol>,
    /// The reason why the member (re-)joins the group.
    pub reason: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for JoinGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Bytes::new(),
            session_timeout_ms: 0,
            rebalance_timeout_ms: -1,
            member_id: Bytes::new(),
            group_instance_id: None,
            protocol_type: Bytes::new(),
            protocols: Vec::new(),
            reason: None,
            tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupRequest {
    pub const API_KEY: i16 = 11;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 9;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 6;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 6 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
        }
        {
            size += 4;
        }
        if version >= 1 {
            size += 4;
        }
        {
            size += if version >= 6 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 6 { compact_nullable_string_size(self.group_instance_id.as_deref()) } else { nullable_string_size(self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        {
            size += if version >= 6 { compact_string_size(&self.protocol_type) } else { string_size(&self.protocol_type) };
        }
        {
            { let arr = &self.protocols;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 {
            size += if version >= 8 { if version >= 6 { compact_nullable_string_size(self.reason.as_deref()) } else { nullable_string_size(self.reason.as_deref()) } } else { let v = self.reason.as_deref().expect("field reason is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 6 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        {
            put_i32(buf, self.session_timeout_ms);
        }
        if version >= 1 {
            put_i32(buf, self.rebalance_timeout_ms);
        }
        {
            if version >= 6 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        if version >= 5 {
            if version >= 5 { if version >= 6 { put_compact_nullable_string(buf, self.group_instance_id.as_deref()) } else { put_nullable_string(buf, self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            if version >= 6 { put_compact_string(buf, &self.protocol_type) } else { put_string(buf, &self.protocol_type) };
        }
        {
            { let arr = &self.protocols;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 {
            if version >= 8 { if version >= 6 { put_compact_nullable_string(buf, self.reason.as_deref()) } else { put_nullable_string(buf, self.reason.as_deref()) } } else { let v = self.reason.as_deref().expect("field reason is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = JoinGroupRequest::default();
        {
            msg.group_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.session_timeout_ms = get_i32(buf)?;
        }
        if version >= 1 {
            msg.rebalance_timeout_ms = get_i32(buf)?;
        }
        {
            msg.member_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 5 {
            msg.group_instance_id = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.protocol_type = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(JoinGroupRequestProtocol::decode(version, buf)?); }
            msg.protocols = items; }
        }
        if version >= 8 {
            msg.reason = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 8 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for JoinGroupRequest {
    const API_KEY: i16 = 11;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 9;
    const FLEXIBLE_MIN_VERSION: i16 = 6;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for JoinGroupRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
