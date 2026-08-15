#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

#[derive(Debug, Clone, Default)]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    pub name: Bytes,
    /// The responses per partition.
    pub partitions: Vec<OffsetFetchResponsePartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchResponseTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 7 {
            size += if version >= 6 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version <= 7 {
            { let arr = &self.partitions;
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
        if version <= 7 {
            if version >= 6 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version <= 7 {
            { let arr = &self.partitions;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchResponseTopic::default();
        if version <= 7 {
            msg.name = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version <= 7 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchResponsePartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The committed message offset.
    pub committed_offset: i64,
    /// The leader epoch.
    pub committed_leader_epoch: i32,
    /// The partition metadata.
    pub metadata: Option<Bytes>,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetFetchResponsePartition {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            metadata: Some(Bytes::new()),
            error_code: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponsePartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version <= 7 {
            size += 4;
        }
        if version <= 7 {
            size += 8;
        }
        if version >= 5 && version <= 7 {
            size += 4;
        }
        if version <= 7 {
            size += if version <= 7 { if version >= 6 { compact_nullable_string_size(self.metadata.as_deref()) } else { nullable_string_size(self.metadata.as_deref()) } } else { let v = self.metadata.as_deref().expect("field metadata is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        if version <= 7 {
            size += 2;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version <= 7 {
            put_i32(buf, self.partition_index);
        }
        if version <= 7 {
            put_i64(buf, self.committed_offset);
        }
        if version >= 5 && version <= 7 {
            put_i32(buf, self.committed_leader_epoch);
        }
        if version <= 7 {
            if version <= 7 { if version >= 6 { put_compact_nullable_string(buf, self.metadata.as_deref()) } else { put_nullable_string(buf, self.metadata.as_deref()) } } else { let v = self.metadata.as_deref().expect("field metadata is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version <= 7 {
            put_i16(buf, self.error_code);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchResponsePartition::default();
        if version <= 7 {
            msg.partition_index = get_i32(buf)?;
        }
        if version <= 7 {
            msg.committed_offset = get_i64(buf)?;
        }
        if version >= 5 && version <= 7 {
            msg.committed_leader_epoch = get_i32(buf)?;
        }
        if version <= 7 {
            msg.metadata = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version <= 7 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version <= 7 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OffsetFetchResponseGroup {
    /// The group ID.
    pub group_id: Bytes,
    /// The responses per topic.
    pub topics: Vec<OffsetFetchResponseTopics>,
    /// The group-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchResponseGroup {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 {
            size += if version >= 6 { compact_string_size(&self.group_id) } else { string_size(&self.group_id) };
        }
        if version >= 8 {
            { let arr = &self.topics;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 8 {
            size += 2;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 {
            if version >= 6 { put_compact_string(buf, &self.group_id) } else { put_string(buf, &self.group_id) };
        }
        if version >= 8 {
            { let arr = &self.topics;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 8 {
            put_i16(buf, self.error_code);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchResponseGroup::default();
        if version >= 8 {
            msg.group_id = (if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        if version >= 8 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchResponseTopics::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 8 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct OffsetFetchResponseTopics {
    /// The topic name.
    pub name: Bytes,
    /// The topic ID.
    pub topic_id: [u8; 16],
    /// The responses per partition.
    pub partitions: Vec<OffsetFetchResponsePartitions>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchResponseTopics {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 && version <= 9 {
            size += if version >= 6 { compact_string_size(&self.name) } else { string_size(&self.name) };
        }
        if version >= 10 {
            size += 16;
        }
        if version >= 8 {
            { let arr = &self.partitions;
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
        if version >= 8 && version <= 9 {
            if version >= 6 { put_compact_string(buf, &self.name) } else { put_string(buf, &self.name) };
        }
        if version >= 10 {
            put_uuid(buf, &self.topic_id);
        }
        if version >= 8 {
            { let arr = &self.partitions;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchResponseTopics::default();
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
                for _ in 0..count { items.push(OffsetFetchResponsePartitions::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone)]
pub struct OffsetFetchResponsePartitions {
    /// The partition index.
    pub partition_index: i32,
    /// The committed message offset.
    pub committed_offset: i64,
    /// The leader epoch.
    pub committed_leader_epoch: i32,
    /// The partition metadata.
    pub metadata: Option<Bytes>,
    /// The partition-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for OffsetFetchResponsePartitions {
    fn default() -> Self {
        Self {
            partition_index: 0,
            committed_offset: 0,
            committed_leader_epoch: -1,
            metadata: Some(Bytes::new()),
            error_code: 0,
            tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponsePartitions {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        if version >= 8 {
            size += 4;
        }
        if version >= 8 {
            size += 8;
        }
        if version >= 8 {
            size += 4;
        }
        if version >= 8 {
            size += if version >= 8 { if version >= 6 { compact_nullable_string_size(self.metadata.as_deref()) } else { nullable_string_size(self.metadata.as_deref()) } } else { let v = self.metadata.as_deref().expect("field metadata is None but not nullable at this version"); if version >= 6 { compact_string_size(v) } else { string_size(v) } };
        }
        if version >= 8 {
            size += 2;
        }
        if version >= 6 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        if version >= 8 {
            put_i32(buf, self.partition_index);
        }
        if version >= 8 {
            put_i64(buf, self.committed_offset);
        }
        if version >= 8 {
            put_i32(buf, self.committed_leader_epoch);
        }
        if version >= 8 {
            if version >= 8 { if version >= 6 { put_compact_nullable_string(buf, self.metadata.as_deref()) } else { put_nullable_string(buf, self.metadata.as_deref()) } } else { let v = self.metadata.as_deref().expect("field metadata is None but not nullable at this version"); if version >= 6 { put_compact_string(buf, v) } else { put_string(buf, v) } };
        }
        if version >= 8 {
            put_i16(buf, self.error_code);
        }
        if version >= 6 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = OffsetFetchResponsePartitions::default();
        if version >= 8 {
            msg.partition_index = get_i32(buf)?;
        }
        if version >= 8 {
            msg.committed_offset = get_i64(buf)?;
        }
        if version >= 8 {
            msg.committed_leader_epoch = get_i32(buf)?;
        }
        if version >= 8 {
            msg.metadata = { let v = if version >= 6 { get_compact_string(buf)? } else { get_string(buf)? }; if version >= 8 { v } else { Some(v.ok_or(DecodeError::NullForNonNullable)?) } };
        }
        if version >= 8 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-10.
#[derive(Debug, Clone, Default)]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses per topic.
    pub topics: Vec<OffsetFetchResponseTopic>,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The responses per group id.
    pub groups: Vec<OffsetFetchResponseGroup>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl OffsetFetchResponse {
    pub const API_KEY: i16 = 9;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 10;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 6;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        if version >= 3 {
            size += 4;
        }
        if version <= 7 {
            { let arr = &self.topics;
                if version >= 6 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 && version <= 7 {
            size += 2;
        }
        if version >= 8 {
            { let arr = &self.groups;
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
        if version >= 3 {
            put_i32(buf, self.throttle_time_ms);
        }
        if version <= 7 {
            { let arr = &self.topics;
                if version >= 6 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 && version <= 7 {
            put_i16(buf, self.error_code);
        }
        if version >= 8 {
            { let arr = &self.groups;
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
        let mut msg = OffsetFetchResponse::default();
        if version >= 3 {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version <= 7 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchResponseTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 2 && version <= 7 {
            msg.error_code = get_i16(buf)?;
        }
        if version >= 8 {
            let len_opt = if version >= 6 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(OffsetFetchResponseGroup::decode(version, buf)?); }
            msg.groups = items; }
        }
        if version >= 6 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for OffsetFetchResponse {
    const API_KEY: i16 = 9;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 10;
    const FLEXIBLE_MIN_VERSION: i16 = 6;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for OffsetFetchResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
