#![allow(unused_variables, clippy::manual_range_contains)]

use bytes::Bytes;
use crate::codec::*;
use crate::error::DecodeError;

/// Retired API (validVersions: "none"): kept for the api-key namespace; no version can be encoded or decoded.
#[derive(Debug, Clone, Default)]
pub struct StopReplicaResponse {
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl StopReplicaResponse {
    pub const API_KEY: i16 = 5;
    pub const VALID_MIN_VERSION: i16 = 32767;
    pub const VALID_MAX_VERSION: i16 = -32768;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;

    pub fn encoded_size(&self, version: i16) -> usize {
        panic!("api key {} supports no protocol versions (requested {})", Self::API_KEY, version);
    }

    pub fn encode<B: WireBuf>(&self, version: i16, _buf: &mut B) {
        panic!("api key {} supports no protocol versions (requested {})", Self::API_KEY, version);
    }

    pub fn decode(version: i16, _buf: &mut Bytes) -> Result<Self, DecodeError> {
        Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version })
    }
}

impl crate::Encodable for StopReplicaResponse {
    const API_KEY: i16 = 5;
    const VALID_MIN_VERSION: i16 = 32767;
    const VALID_MAX_VERSION: i16 = -32768;
    const FLEXIBLE_MIN_VERSION: i16 = i16::MAX;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for StopReplicaResponse {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
