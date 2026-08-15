#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: i16,
    /// The describe error message, or null if there was no error.
    pub error_message: Option<StrBytes>,
    /// The group ID string.
    pub group_id: GroupId,
    /// The group state string, or the empty string.
    pub group_state: StrBytes,
    /// The group protocol type, or the empty string.
    pub protocol_type: StrBytes,
    /// The group protocol data, or the empty string.
    pub protocol_data: StrBytes,
    /// The group members.
    pub members: Vec<DescribedGroupMember>,
    /// 32-bit bitfield to represent authorized operations for this group.
    pub authorized_operations: i32,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribedGroup {
    fn default() -> Self {
        Self {
            error_code: 0,
            error_message: None,
            group_id: GroupId::default(),
            group_state: StrBytes::new(),
            protocol_type: StrBytes::new(),
            protocol_data: StrBytes::new(),
            members: Vec::new(),
            authorized_operations: -2147483648,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribedGroup {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        if version >= 6 {
            size += if version >= 6 { if version >= 5 { compact_nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 5 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            size += if version >= 5 { compact_string_size(self.group_id.as_str()) } else { string_size(self.group_id.as_str()) };
        }
        {
            size += if version >= 5 { compact_string_size(self.group_state.as_str()) } else { string_size(self.group_state.as_str()) };
        }
        {
            size += if version >= 5 { compact_string_size(self.protocol_type.as_str()) } else { string_size(self.protocol_type.as_str()) };
        }
        {
            size += if version >= 5 { compact_string_size(self.protocol_data.as_str()) } else { string_size(self.protocol_data.as_str()) };
        }
        {
            { let arr = &self.members;
                if version >= 5 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 3 {
            size += 4;
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        if version >= 6 {
            if version >= 6 { if version >= 5 { put_compact_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.error_message.as_ref().map(|v| v.as_str())) } } else { let v = self.error_message.as_ref().expect("field error_message is None but not nullable at this version"); if version >= 5 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            if version >= 5 { put_compact_string(buf, self.group_id.as_str()) } else { put_string(buf, self.group_id.as_str()) };
        }
        {
            if version >= 5 { put_compact_string(buf, self.group_state.as_str()) } else { put_string(buf, self.group_state.as_str()) };
        }
        {
            if version >= 5 { put_compact_string(buf, self.protocol_type.as_str()) } else { put_string(buf, self.protocol_type.as_str()) };
        }
        {
            if version >= 5 { put_compact_string(buf, self.protocol_data.as_str()) } else { put_string(buf, self.protocol_data.as_str()) };
        }
        {
            { let arr = &self.members;
                if version >= 5 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 3 {
            put_i32(buf, self.authorized_operations);
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribedGroup::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 6 {
            msg.error_message = { let v = if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 6 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.group_id = GroupId((if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.group_state = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.protocol_type = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.protocol_data = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 5 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribedGroupMember::decode(version, buf)?); }
            msg.members = items; }
        }
        if version >= 3 {
            msg.authorized_operations = get_i32(buf)?;
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribedGroupMember {
    /// The member id.
    pub member_id: StrBytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<StrBytes>,
    /// The client ID used in the member's latest join group request.
    pub client_id: StrBytes,
    /// The client host.
    pub client_host: StrBytes,
    /// The metadata corresponding to the current group protocol in use.
    pub member_metadata: Bytes,
    /// The current assignment provided by the group leader.
    pub member_assignment: Bytes,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribedGroupMember {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 5 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
        }
        if version >= 4 {
            size += if version >= 4 { if version >= 5 { compact_nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 5 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            size += if version >= 5 { compact_string_size(self.client_id.as_str()) } else { string_size(self.client_id.as_str()) };
        }
        {
            size += if version >= 5 { compact_string_size(self.client_host.as_str()) } else { string_size(self.client_host.as_str()) };
        }
        {
            size += if version >= 5 { compact_bytes_size(&self.member_metadata) } else { bytes_size(&self.member_metadata) };
        }
        {
            size += if version >= 5 { compact_bytes_size(&self.member_assignment) } else { bytes_size(&self.member_assignment) };
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 5 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 4 {
            if version >= 4 { if version >= 5 { put_compact_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 5 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            if version >= 5 { put_compact_string(buf, self.client_id.as_str()) } else { put_string(buf, self.client_id.as_str()) };
        }
        {
            if version >= 5 { put_compact_string(buf, self.client_host.as_str()) } else { put_string(buf, self.client_host.as_str()) };
        }
        {
            if version >= 5 { put_compact_bytes_zc(buf, &self.member_metadata) } else { put_bytes_zc(buf, &self.member_metadata) };
        }
        {
            if version >= 5 { put_compact_bytes_zc(buf, &self.member_assignment) } else { put_bytes_zc(buf, &self.member_assignment) };
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribedGroupMember::default();
        {
            msg.member_id = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 4 {
            msg.group_instance_id = { let v = if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 4 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            msg.client_id = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.client_host = (if version >= 5 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.member_metadata = (if version >= 5 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.member_assignment = (if version >= 5 { get_compact_bytes(buf)? } else { get_bytes(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 0-6.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    pub groups: Vec<DescribedGroup>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeGroupsResponse {
    pub const API_KEY: i16 = 15;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 6;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 5;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 1 {
            size += 4;
        }
        {
            { let arr = &self.groups;
                if version >= 5 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 5 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 1 {
            put_i32(buf, self.throttle_time_ms);
        }
        {
            { let arr = &self.groups;
                if version >= 5 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 5 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeGroupsResponse::default();
        if version >= 1 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        {
            let len_opt = if version >= 5 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribedGroup::decode(version, buf)?); }
            msg.groups = items; }
        }
        if version >= 5 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeGroupsResponse {
    const API_KEY: i16 = 15;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 6;
    const FLEXIBLE_MIN_VERSION: i16 = 5;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeGroupsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
