#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    pub member_id: Bytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<Bytes>,
    /// The group member metadata.
    pub metadata: Bytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl JoinGroupResponseMember {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 6 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 6 { compact_nullable_string_size(self.group_instance_id.as_deref()) } else { nullable_string_size(self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        {
            size += if version >= 6 { compact_bytes_size(&self.metadata) } else { bytes_size(&self.metadata) };
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 6 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        if version >= 5 {
            if version >= 5 { if version >= 6 { put_compact_nullable_string(buf, self.group_instance_id.as_deref()) } else { put_nullable_string(buf, self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            if version >= 6 { put_compact_bytes_zc(buf, &self.metadata) } else { put_bytes_zc(buf, &self.metadata) };
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = JoinGroupResponseMember::default();
        {
            msg.member_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 5 {
            msg.group_instance_id = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
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
pub struct JoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The generation ID of the group.
    pub generation_id: i32,
    /// The group protocol name.
    pub protocol_type: Option<Bytes>,
    /// The group protocol selected by the coordinator.
    pub protocol_name: Option<Bytes>,
    /// The leader of the group.
    pub leader: Bytes,
    /// True if the leader must skip running the assignment.
    pub skip_assignment: bool,
    /// The member ID assigned by the group coordinator.
    pub member_id: Bytes,
    /// The group members.
    pub members: Vec<JoinGroupResponseMember>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for JoinGroupResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            generation_id: -1,
            protocol_type: None,
            protocol_name: Some(Bytes::new()),
            leader: Bytes::new(),
            skip_assignment: false,
            member_id: Bytes::new(),
            members: Vec::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupResponse {
    pub const API_KEY: i16 = 11;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 9;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 6;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 2 {
            size += 4;
        }
        {
            size += 2;
        }
        {
            size += 4;
        }
        if version >= 7 {
            size += if version >= 7 { if version >= 6 { compact_nullable_string_size(self.protocol_type.as_deref()) } else { nullable_string_size(self.protocol_type.as_deref()) } } else { let v = self.protocol_type.as_deref().expect("field protocol_type is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        {
            size += if version >= 7 { if version >= 6 { compact_nullable_string_size(self.protocol_name.as_deref()) } else { nullable_string_size(self.protocol_name.as_deref()) } } else { let v = self.protocol_name.as_deref().expect("field protocol_name is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        {
            size += if version >= 6 { compact_string_size(&self.leader) } else { string_size(&self.leader) };
        }
        if version >= 9 {
            size += 1;
        }
        {
            size += if version >= 6 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        {
            { let arr = &self.members;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 2 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            put_i16(buf, self.error_code);
        }
        {
            put_i32(buf, self.generation_id);
        }
        if version >= 7 {
            if version >= 7 { if version >= 6 { put_compact_nullable_string(buf, self.protocol_type.as_deref()) } else { put_nullable_string(buf, self.protocol_type.as_deref()) } } else { let v = self.protocol_type.as_deref().expect("field protocol_type is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            if version >= 7 { if version >= 6 { put_compact_nullable_string(buf, self.protocol_name.as_deref()) } else { put_nullable_string(buf, self.protocol_name.as_deref()) } } else { let v = self.protocol_name.as_deref().expect("field protocol_name is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        {
            if version >= 6 { put_compact_string(buf, &self.leader) } else { put_string(buf, &self.leader) };
        }
        if version >= 9 {
            put_bool(buf, self.skip_assignment);
        }
        {
            if version >= 6 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        {
            { let arr = &self.members;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = JoinGroupResponse::default();
        if version >= 2 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.generation_id = get_i32(buf)?;
        }
        if version >= 7 {
            msg.protocol_type = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 7 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.protocol_name = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 7 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.leader = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 9 {
            msg.skip_assignment = get_bool(buf)?;
        }
        {
            msg.member_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(JoinGroupResponseMember::decode(version, buf)?); }
            msg.members = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for JoinGroupResponse {
    const API_KEY: i16 = 11;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 9;
    const FLEXIBLE_MIN_VERSION: i16 = 6;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for JoinGroupResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
