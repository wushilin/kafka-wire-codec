#![allow(unused_variables, unused_imports, clippy::manual_range_contains, clippy::unnecessary_unwrap)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::{DecodeError, EncodeError};
use crate::types::*;

/// Valid versions: 0-4.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeartbeatRequest {
    /// The group id.
    pub group_id: GroupId,
    /// The generation of the group.
    pub generation_id: i32,
    /// The member ID.
    pub member_id: StrBytes,
    /// The unique identifier of the consumer instance provided by end user.
    pub group_instance_id: Option<StrBytes>,
    /// Unknown/raw tagged fields (flexible versions), ascending tag order.
    /// Schema-declared tagged fields decode into their typed fields above,
    /// not into this bucket.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl HeartbeatRequest {
    pub const API_KEY: i16 = 12;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 4;
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
        {
            size += 4;
        }
        {
            size += if version >= 4 { compact_string_size(self.member_id.as_str()) } else { string_size(self.member_id.as_str()) };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 4 { compact_nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
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
        {
            put_i32(buf, self.generation_id);
        }
        {
            if version >= 4 { put_compact_string(buf, self.member_id.as_str()) } else { put_string(buf, self.member_id.as_str()) };
        }
        if version >= 3 {
            if version >= 3 { if version >= 4 { put_compact_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.group_instance_id.as_ref().map(|v| v.as_str())) } } else { let v = self.group_instance_id.as_ref().expect("field group_instance_id is None but not nullable at this version"); if version >= 4 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 4 { put_tagged_fields(buf, &self.tagged_fields); }
        Ok(())
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = HeartbeatRequest::default();
        {
            msg.group_id = GroupId((if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            msg.generation_id = get_i32(buf)?;
        }
        {
            msg.member_id = (if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 3 {
            msg.group_instance_id = { let v = if version >= 4 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 4 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for HeartbeatRequest {
    const API_KEY: i16 = 12;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 4;
    const FLEXIBLE_MIN_VERSION: i16 = 4;
    fn wire_size(&self, version: i16) -> Result<usize, EncodeError> { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) -> Result<(), EncodeError> { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for HeartbeatRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) -> Result<(), EncodeError> { self.encode(version, buf) }
}
