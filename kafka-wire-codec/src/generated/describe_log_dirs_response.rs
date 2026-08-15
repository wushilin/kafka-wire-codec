#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct DescribeLogDirsResult {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The absolute log directory path.
    pub log_dir: StrBytes,
    /// The topics.
    pub topics: Vec<DescribeLogDirsTopic>,
    /// The total size in bytes of the volume the log directory is in. This value does not include the size of data stored in remote storage.
    pub total_bytes: i64,
    /// The usable size in bytes of the volume the log directory is in. This value does not include the size of data stored in remote storage.
    pub usable_bytes: i64,
    /// True if this log directory is cordoned.
    pub is_cordoned: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for DescribeLogDirsResult {
    fn default() -> Self {
        Self {
            error_code: 0,
            log_dir: StrBytes::new(),
            topics: Vec::new(),
            total_bytes: -1,
            usable_bytes: -1,
            is_cordoned: false,
            tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsResult {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 2;
        }
        {
            size += if version >= 2 { compact_string_size(self.log_dir.as_str()) } else { string_size(self.log_dir.as_str()) };
        }
        {
            { let arr = &self.topics;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 4 {
            size += 8;
        }
        if version >= 4 {
            size += 8;
        }
        if version >= 5 {
            size += 1;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i16(buf, self.error_code);
        }
        {
            if version >= 2 { put_compact_string(buf, self.log_dir.as_str()) } else { put_string(buf, self.log_dir.as_str()) };
        }
        {
            { let arr = &self.topics;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 4 {
            put_i64(buf, self.total_bytes);
        }
        if version >= 4 {
            put_i64(buf, self.usable_bytes);
        }
        if version >= 5 {
            put_bool(buf, self.is_cordoned);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeLogDirsResult::default();
        {
            msg.error_code = get_i16(buf)?;
        }
        {
            msg.log_dir = (if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeLogDirsTopic::decode(version, buf)?); }
            msg.topics = items; }
        }
        if version >= 4 {
            msg.total_bytes = get_i64(buf)?;
        }
        if version >= 4 {
            msg.usable_bytes = get_i64(buf)?;
        }
        if version >= 5 {
            msg.is_cordoned = get_bool(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DescribeLogDirsTopic {
    /// The topic name.
    pub name: TopicName,
    /// The partitions.
    pub partitions: Vec<DescribeLogDirsPartition>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeLogDirsTopic {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += if version >= 2 { compact_string_size(self.name.as_str()) } else { string_size(self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            if version >= 2 { put_compact_string(buf, self.name.as_str()) } else { put_string(buf, self.name.as_str()) };
        }
        {
            { let arr = &self.partitions;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeLogDirsTopic::default();
        {
            msg.name = TopicName((if version >= 2 { get_compact_string(buf)? } else { get_string(buf)? }).ok_or(DecodeError::NullForNonNullable)?);
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeLogDirsPartition::decode(version, buf)?); }
            msg.partitions = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

#[derive(Debug, Clone, Default)]
pub struct DescribeLogDirsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The size of the log segments in this partition in bytes.
    pub partition_size: i64,
    /// The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition).
    pub offset_lag: i64,
    /// True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
    pub is_future_key: bool,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeLogDirsPartition {
    pub fn encoded_size(&self, version: i16) -> usize {
        let mut size = 0usize;
        {
            size += 4;
        }
        {
            size += 8;
        }
        {
            size += 8;
        }
        {
            size += 1;
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        {
            put_i32(buf, self.partition_index);
        }
        {
            put_i64(buf, self.partition_size);
        }
        {
            put_i64(buf, self.offset_lag);
        }
        {
            put_bool(buf, self.is_future_key);
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        let mut msg = DescribeLogDirsPartition::default();
        {
            msg.partition_index = get_i32(buf)?;
        }
        {
            msg.partition_size = get_i64(buf)?;
        }
        {
            msg.offset_lag = get_i64(buf)?;
        }
        {
            msg.is_future_key = get_bool(buf)?;
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

/// Valid versions: 1-5.
#[derive(Debug, Clone, Default)]
pub struct DescribeLogDirsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The log directories.
    pub results: Vec<DescribeLogDirsResult>,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl DescribeLogDirsResponse {
    pub const API_KEY: i16 = 35;
    pub const VALID_MIN_VERSION: i16 = 1;
    pub const VALID_MAX_VERSION: i16 = 5;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 2;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += 4;
        }
        if version >= 3 {
            size += 2;
        }
        {
            { let arr = &self.results;
                if version >= 2 { size += uvarint_size(arr.len() as u64 + 1); } else { size += 4; }
                for item in arr {
                    size += item.encoded_size(version);
                }
            }
        }
        if version >= 2 { size += tagged_fields_size(&self.tagged_fields); }
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_i32(buf, self.throttle_time_ms);
        }
        if version >= 3 {
            put_i16(buf, self.error_code);
        }
        {
            { let arr = &self.results;
                if version >= 2 { put_uvarint(buf, arr.len() as u64 + 1); } else { put_i32(buf, arr.len() as i32); }
                for item in arr { item.encode(version, buf); }
            }
        }
        if version >= 2 { put_tagged_fields(buf, &self.tagged_fields); }
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = DescribeLogDirsResponse::default();
        {
            msg.throttle_time_ms = get_i32(buf)?;
        }
        if version >= 3 {
            msg.error_code = get_i16(buf)?;
        }
        {
            let len_opt = if version >= 2 { { let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } } } else { { let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } } };
            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;
            { let mut items = Vec::with_capacity(count.min(buf.len()));
                for _ in 0..count { items.push(DescribeLogDirsResult::decode(version, buf)?); }
            msg.results = items; }
        }
        if version >= 2 { msg.tagged_fields = get_tagged_fields(buf)?; }
        Ok(msg)
    }
}

impl crate::Encodable for DescribeLogDirsResponse {
    const API_KEY: i16 = 35;
    const VALID_MIN_VERSION: i16 = 1;
    const VALID_MAX_VERSION: i16 = 5;
    const FLEXIBLE_MIN_VERSION: i16 = 2;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for DescribeLogDirsResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
