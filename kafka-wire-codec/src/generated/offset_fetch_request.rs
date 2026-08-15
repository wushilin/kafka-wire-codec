#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct OffsetFetchRequestTopic {
    /// The topic name.
    pub name: Bytes,
    /// The partition indexes we would like to fetch offsets for.
    pub partition_indexes: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchRequestTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 7 {
            size += if version >= 6 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version <= 7 {
            { let arr = &self.partition_indexes;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 7 {
            if version >= 6 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version <= 7 {
            { let arr = &self.partition_indexes;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchRequestTopic::default();
        if version <= 7 {
            msg.name = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 7 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partition_indexes = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct OffsetFetchRequestGroup {
    /// The group ID.
    pub group_id: Bytes,
    /// The member id.
    pub member_id: Option<Bytes>,
    /// The member epoch if using the new consumer protocol (KIP-848).
    pub member_epoch: i32,
    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    pub topics: Option<Vec<OffsetFetchRequestTopics>>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetFetchRequestGroup {
    fn default() -> Self {
        Self {
            group_id: Bytes::new(),
            member_id: None,
            member_epoch: -1,
            topics: Some(Vec::new()),
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchRequestGroup {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 {
            size += if version >= 6 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
        }
        if version >= 9 {
            size += if version >= 9 { if version >= 6 { compact_nullable_string_size(self.member_id.as_deref()) } else { nullable_string_size(self.member_id.as_deref()) } } else { let v = self.member_id.as_deref().expect("field member_id is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 9 {
            size += 4;
        }
        if version >= 8 {
            match &self.topics {
                Some(arr) => {
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    assert!(version >= 8, "field topics is None but not nullable at version {}", version);
                    if version >= 6 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 {
            if version >= 6 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        if version >= 9 {
            if version >= 9 { if version >= 6 { put_compact_nullable_string(buf, self.member_id.as_deref()) } else { put_nullable_string(buf, self.member_id.as_deref()) } } else { let v = self.member_id.as_deref().expect("field member_id is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 9 {
            put_i32(buf, self.member_epoch);
        }
        if version >= 8 {
            match &self.topics {
                Some(arr) => {
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    assert!(version >= 8, "field topics is None but not nullable at version {}", version);
                    if version >= 6 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchRequestGroup::default();
        if version >= 8 {
            msg.group_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 9 {
            msg.member_id = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 9 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 9 {
            msg.member_epoch = get_i32(buf)?;
        }
        if version >= 8 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.topics = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchRequestTopics::decode(version, buf)?); }
                Some(items)
                }
                None => { if version >= 8 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OffsetFetchRequestTopics {
    /// The topic name.
    pub name: Bytes,
    /// The topic ID.
    pub topic_id: [u8; 16],
    /// The partition indexes we would like to fetch offsets for.
    pub partition_indexes: Vec<i32>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchRequestTopics {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 && version <= 9 {
            size += if version >= 6 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version >= 10 {
            size += 16;
        }
        if version >= 8 {
            { let arr = &self.partition_indexes;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                size += arr.len() * 4;
            }
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 && version <= 9 {
            if version >= 6 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        if version >= 8 {
            { let arr = &self.partition_indexes;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { put_i32(buf, *item); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchRequestTopics::default();
        if version >= 8 && version <= 9 {
            msg.name = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 10 {
            msg.topic_id = get_uuid(buf)?;
        }
        if version >= 8 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(get_i32(buf)?); }
            msg.partition_indexes = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-10.
#[derive(Debug, Clone)]
pub struct OffsetFetchRequest {
    /// The group to fetch offsets for.
    pub group_id: Bytes,
    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    pub topics: Option<Vec<OffsetFetchRequestTopic>>,
    /// Each group we would like to fetch offsets for.
    pub groups: Vec<OffsetFetchRequestGroup>,
    /// Whether broker should hold on returning unstable offsets but set a retriable error code for the partitions.
    pub require_stable: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetFetchRequest {
    fn default() -> Self {
        Self {
            group_id: Bytes::new(),
            topics: Some(Vec::new()),
            groups: Vec::new(),
            require_stable: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchRequest {
    pub const API_KEY: i16 = 9;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 10;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 6;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version <= 7 {
            size += if version >= 6 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
        }
        if version <= 7 {
            match &self.topics {
                Some(arr) => {
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
                }
                None => {
                    assert!(version >= 2 && version <= 7, "field topics is None but not nullable at version {}", version);
                    if version >= 6 { size += 1; } else { size += 4; }
                }
            }
        }
        if version >= 8 {
            { let arr = &self.groups;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 7 {
            size += 1;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        if version <= 7 {
            if version >= 6 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        if version <= 7 {
            match &self.topics {
                Some(arr) => {
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
                }
                None => {
                    assert!(version >= 2 && version <= 7, "field topics is None but not nullable at version {}", version);
                    if version >= 6 { put_uvarint(buf, 0); } else { put_i32(buf, -1); }
                }
            }
        }
        if version >= 8 {
            { let arr = &self.groups;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 7 {
            put_bool(buf, self.require_stable);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = OffsetFetchRequest::default();
        if version <= 7 {
            msg.group_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 7 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            msg.topics = match len_opt {
                Some(count) => {
                let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchRequestTopic::decode(version, buf)?); }
                Some(items)
                }
                None => { if version >= 2 && version <= 7 { None } else { return Err(DecodeError::NullForNonNullable); } }
            };
        }
        if version >= 8 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchRequestGroup::decode(version, buf)?); }
            msg.groups = items; }
        }
        if version >= 7 {
            msg.require_stable = get_bool(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetFetchRequest {
    const API_KEY: i16 = 9;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 10;
    const FLEXIBLE_MIN_VERSION: i16 = 6;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetFetchRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
