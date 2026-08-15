#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone, Default)]
pub struct CreatableRenewers {
    /// The type of the Kafka principal.
    pub principal_type: StrBytes,
    /// The name of the Kafka principal.
    pub principal_name: StrBytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl CreatableRenewers {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.principal_type.as_str()) } else { string_size(self.principal_type.as_str()) };
        }
        {
            size += if version >= 2 { compact_string_size(self.principal_name.as_str()) } else { string_size(self.principal_name.as_str()) };
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.principal_type.as_str()) } else { put_string(buf, self.principal_type.as_str()) };
        }
        {
            if version >= 2 { put_compact_string(buf, self.principal_name.as_str()) } else { put_string(buf, self.principal_name.as_str()) };
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = CreatableRenewers::default();
        {
            msg.principal_type = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.principal_name = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-3.
#[derive(Debug, Clone)]
pub struct CreateDelegationTokenRequest {
    /// The principal type of the owner of the token. If it's null it defaults to the token request principal.
    pub owner_principal_type: Option<StrBytes>,
    /// The principal name of the owner of the token. If it's null it defaults to the token request principal.
    pub owner_principal_name: Option<StrBytes>,
    /// A list of those who are allowed to renew this token before it expires.
    pub renewers: Vec<CreatableRenewers>,
    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    pub max_lifetime_ms: i64,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for CreateDelegationTokenRequest {
    fn default() -> Self {
        Self {
            owner_principal_type: Some(StrBytes::new()),
            owner_principal_name: Some(StrBytes::new()),
            renewers: Vec::new(),
            max_lifetime_ms: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl CreateDelegationTokenRequest {
    pub const API_KEY: i16 = 38;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 3;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 3 {
            size += if version >= 3 { if version >= 2 { compact_nullable_string_size(self.owner_principal_type.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.owner_principal_type.as_ref().map(|v| v.as_str())) } } else { let v = self.owner_principal_type.as_ref().expect("field owner_principal_type is None but not nullable at this version"); if version >= 2 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        if version >= 3 {
            size += if version >= 3 { if version >= 2 { compact_nullable_string_size(self.owner_principal_name.as_ref().map(|v| v.as_str())) } else { nullable_string_size(self.owner_principal_name.as_ref().map(|v| v.as_str())) } } else { let v = self.owner_principal_name.as_ref().expect("field owner_principal_name is None but not nullable at this version"); if version >= 2 { compact_string_size(v.as_str()) } else { string_size(v.as_str()) } };
        }
        {
            { let arr = &self.renewers;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        {
            size += 8;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version >= 3 {
            if version >= 3 { if version >= 2 { put_compact_nullable_string(buf, self.owner_principal_type.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.owner_principal_type.as_ref().map(|v| v.as_str())) } } else { let v = self.owner_principal_type.as_ref().expect("field owner_principal_type is None but not nullable at this version"); if version >= 2 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        if version >= 3 {
            if version >= 3 { if version >= 2 { put_compact_nullable_string(buf, self.owner_principal_name.as_ref().map(|v| v.as_str())) } else { put_nullable_string(buf, self.owner_principal_name.as_ref().map(|v| v.as_str())) } } else { let v = self.owner_principal_name.as_ref().expect("field owner_principal_name is None but not nullable at this version"); if version >= 2 { put_compact_string(buf, v.as_str()) } else { put_string(buf, v.as_str()) } };
        }
        {
            { let arr = &self.renewers;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        {
            put_i64(buf, self.max_lifetime_ms);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = CreateDelegationTokenRequest::default();
        if version >= 3 {
            msg.owner_principal_type = { let v = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 3 {
            msg.owner_principal_name = { let v = if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 3 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(CreatableRenewers::decode(version, buf)?); }
            msg.renewers = items; }
        }
        {
            msg.max_lifetime_ms = get_i64(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for CreateDelegationTokenRequest {
    const API_KEY: i16 = 38;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 3;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for CreateDelegationTokenRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
