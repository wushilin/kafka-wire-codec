#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MemberIdentity {
    /// The member ID to remove from the group.
    pub member_id: StrBytes,
    /// The group instance ID to remove from the group.
    pub group_instance_id: Option<StrBytes>,
    /// The reason why the member left the group.
    pub reason: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl MemberIdentity {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 4 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 4 { compact_nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 5 {
            size += if version >= 5 { if version >= 4 { compact_nullable_string_size(self.reason.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.reason.as_ref().map(|v| v.as_str())) } } else { let v = self.reason.as_ref().expect("field reason is None but not nullable at this version"); if version >= 4 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 4 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 3 {
            if version >= 4 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 3 {
            if version >= 3 { if version >= 4 { put_compact_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 5 {
            if version >= 5 { if version >= 4 { put_compact_nullable_string(buf, self.reason.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.reason.as_ref().map(|v| v.as_str())) } } else { let v = self.reason.as_ref().expect("field reason is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    pub group_id: GroupId,
    /// The member ID to remove from the group.
    pub member_id: StrBytes,
    /// List of leaving member identities.
    pub members: Vec<MemberIdentity>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl LeaveGroupRequest {
    pub const API_KEY: i16 = 13;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 4;

    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut size = 0usize;
        {
            size += if version >= 4 { compact_string_size(self.group_id.as_str()) } else { string_size(self.group_id.as_str()) };
        }
        if version <= 2 {
            size += if version >= 4 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
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
        Ok(size)
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(EncodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        {
            if version >= 4 { put_compact_string(buf, self.group_id.as_str()) } else { put_string(buf, self.group_id.as_str()) };
        }
        if version <= 2 {
            if version >= 4 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 3 {
            { let arr = &self.members;
                if version >= 4 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = LeaveGroupRequest::default();
        {
            msg.group_id = GroupId((if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
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
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for LeaveGroupRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
