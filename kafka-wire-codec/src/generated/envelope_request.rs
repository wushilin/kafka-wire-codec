#![allow(unused_variables, unused_imports, clippy::manual_range_contains)]

use bytes::Bytes;
use uuid::Uuid;
use crate::codec::*;
use crate::error::DecodeError;
use crate::types::*;

/// Valid versions: 0-0.
#[derive(Debug, Clone)]
pub struct EnvelopeRequest {
    /// The embedded request header and data.
    pub request_data: Bytes,
    /// Value of the initial client principal when the request is redirected by a broker.
    pub request_principal: Option<Bytes>,
    /// The original client's address in bytes.
    pub client_host_address: Bytes,
    /// Raw tagged fields (flexible versions), in ascending tag order.
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl Default for EnvelopeRequest {
    fn default() -> Self {
        Self {
            request_data: Bytes::new(),
            request_principal: Some(Bytes::new()),
            client_host_address: Bytes::new(),
            tagged_fields: Vec::new(),
        }
    }
}

impl EnvelopeRequest {
    pub const API_KEY: i16 = 58;
    pub const VALID_MIN_VERSION: i16 = 0;
    pub const VALID_MAX_VERSION: i16 = 0;
    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.
    pub const FLEXIBLE_MIN_VERSION: i16 = 0;

    pub fn encoded_size(&self, version: i16) -> usize {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        let mut size = 0usize;
        {
            size += compact_bytes_size(&self.request_data);
        }
        {
            size += compact_nullable_bytes_size(self.request_principal.as_deref());
        }
        {
            size += compact_bytes_size(&self.client_host_address);
        }
        size += tagged_fields_size(&self.tagged_fields);
        size
    }

    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {
        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),
            "unsupported version {} for api key {}", version, Self::API_KEY);
        {
            put_compact_bytes_zc(buf, &self.request_data);
        }
        {
            put_compact_nullable_bytes_zc(buf, self.request_principal.as_ref());
        }
        {
            put_compact_bytes_zc(buf, &self.client_host_address);
        }
        put_tagged_fields(buf, &self.tagged_fields);
    }

    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {
        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {
            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });
        }
        let mut msg = EnvelopeRequest::default();
        {
            msg.request_data = (get_compact_bytes(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        {
            msg.request_principal = get_compact_bytes(buf)?;
        }
        {
            msg.client_host_address = (get_compact_bytes(buf)?).ok_or(DecodeError::NullForNonNullable)?;
        }
        msg.tagged_fields = get_tagged_fields(buf)?;
        Ok(msg)
    }
}

impl crate::Encodable for EnvelopeRequest {
    const API_KEY: i16 = 58;
    const VALID_MIN_VERSION: i16 = 0;
    const VALID_MAX_VERSION: i16 = 0;
    const FLEXIBLE_MIN_VERSION: i16 = 0;
    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }
    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }
    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }
}

impl crate::EncodableZeroCopy for EnvelopeRequest {
    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }
}
