#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct MemberIdentity {
    /// The member ID to remove from the group.
    pub member_id: Bytes,
    /// The group instance ID to remove from the group.
    pub group_instance_id: Option<Bytes>,
    /// The reason why the member left the group.
    pub reason: Option<Bytes>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl MemberIdentity {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 4 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 4 { compact_nullable_string_size(self.group_instance_id.as_deref()) } else { nullable_string_size(self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 4 { compact_nullable_string_size(self.reason.as_deref()) } else { nullable_string_size(self.reason.as_deref()) } } else { let v = self.reason.as_deref().expect("field reason is None but not nullable at this version"); if version >= 4 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 3 {
            if version >= 4 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        if version >= 3 {
            if version >= 3 { if version >= 4 { put_compact_nullable_string(buf, self.group_instance_id.as_deref()) } else { put_nullable_string(buf, self.group_instance_id.as_deref()) } } else { let v = self.group_instance_id.as_deref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 5 {
            if version >= 5 { if version >= 4 { put_compact_nullable_string(buf, self.reason.as_deref()) } else { put_nullable_string(buf, self.reason.as_deref()) } } else { let v = self.reason.as_deref().expect("field reason is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = MemberIdentity::default();
        if version >= 3 {
            msg.member_id = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.group_instance_id = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 5 {
            msg.reason = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 5 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-5.
#[derive(Debug, Clone, Default)]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    pub group_id: Bytes,
    /// The member ID to remove from the group.
    pub member_id: Bytes,
    /// List of leaving member identities.
    pub members: Vec<MemberIdentity>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl LeaveGroupRequest {
    pub const API_KEY: i16 = 13;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += if version >= 4 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
        }
        if version <= 2 {
            size += if version >= 4 { compact_string_size(&self.member_id) } else { string_size(&self.member_id) };
        }
        if version >= 3 {
            { let arr = &self.members;
                if version >= 4 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            if version >= 4 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        if version <= 2 {
            if version >= 4 { put_compact_string(buf, &self.member_id) } else { put_string(buf, &self.member_id) };
        }
        if version >= 3 {
            { let arr = &self.members;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = LeaveGroupRequest::default();
        {
            msg.group_id = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 2 {
            msg.member_id = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            let len_opt = if version >= 4 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(MemberIdentity::decode(version, buf)?); }
            msg.members = items; }
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for LeaveGroupRequest {
    const API_KEY: i16 = 13;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for LeaveGroupRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
