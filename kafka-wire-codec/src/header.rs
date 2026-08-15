use crate::codec::*;
use crate::error::DecodeError;
use bytes::Bytes;

/// Decoded request header (all fields, all header versions).
#[derive(Debug, Clone)]
pub struct RequestHeader {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    /// Present in header versions >= 1.
    pub client_id: Option<Bytes>,
    /// Present in header versions >= 2 (flexible).
    pub tagged_fields: Vec<(u32, Bytes)>,
}

/// Decoded response header.
#[derive(Debug, Clone)]
pub struct ResponseHeader {
    pub correlation_id: i32,
    /// Present in header versions >= 1 (flexible).
    pub tagged_fields: Vec<(u32, Bytes)>,
}

impl RequestHeader {
    /// Decode request header from buf, advancing past the header bytes.
    /// Returns the header and leaves the body in buf.
    /// header_version: 0 = no client_id, 1 = with client_id, 2 = flexible
    pub fn decode(buf: &mut Bytes, header_version: i16) -> Result<Self, DecodeError> {
        let api_key = get_i16(buf)?;
        let api_version = get_i16(buf)?;
        let correlation_id = get_i32(buf)?;
        let client_id = if header_version >= 1 {
            get_string(buf)?
        } else {
            None
        };
        let tagged_fields = if header_version >= 2 {
            get_tagged_fields(buf)?
        } else {
            vec![]
        };
        Ok(RequestHeader {
            api_key,
            api_version,
            correlation_id,
            client_id,
            tagged_fields,
        })
    }

    pub fn encoded_size(&self, header_version: i16) -> usize {
        let mut size = 2 + 2 + 4; // api_key + api_version + correlation_id
        if header_version >= 1 {
            size += nullable_string_size(self.client_id.as_deref());
        }
        if header_version >= 2 {
            size += tagged_fields_size(&self.tagged_fields);
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, buf: &mut B, header_version: i16) {
        put_i16(buf, self.api_key);
        put_i16(buf, self.api_version);
        put_i32(buf, self.correlation_id);
        if header_version >= 1 {
            put_nullable_string(buf, self.client_id.as_deref());
        }
        if header_version >= 2 {
            put_tagged_fields(buf, &self.tagged_fields);
        }
    }
}

impl ResponseHeader {
    pub fn decode(buf: &mut Bytes, header_version: i16) -> Result<Self, DecodeError> {
        let correlation_id = get_i32(buf)?;
        let tagged_fields = if header_version >= 1 {
            get_tagged_fields(buf)?
        } else {
            vec![]
        };
        Ok(ResponseHeader {
            correlation_id,
            tagged_fields,
        })
    }

    pub fn encoded_size(&self, header_version: i16) -> usize {
        let mut size = 4;
        if header_version >= 1 {
            size += tagged_fields_size(&self.tagged_fields);
        }
        size
    }

    pub fn encode<B: WireBuf>(&self, buf: &mut B, header_version: i16) {
        put_i32(buf, self.correlation_id);
        if header_version >= 1 {
            put_tagged_fields(buf, &self.tagged_fields);
        }
    }
}
