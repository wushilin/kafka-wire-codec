use thiserror::Error;

/// Encoding failure. Non-exhaustive so new failure modes can be added in a
/// patch release without breaking downstream `match`es.
///
/// Encoding is caller-driven, so today the only failure is asking for a
/// version outside the message's supported range — the same condition
/// [`DecodeError::UnsupportedVersion`] reports on the decode side.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EncodeError {
    #[error("unsupported version {version} for api key {api_key}")]
    UnsupportedVersion { api_key: i16, version: i16 },
}

/// Non-exhaustive so new failure modes can be added in a patch release without
/// breaking downstream `match`es. Match with a `_ =>` arm.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DecodeError {
    #[error("unexpected end of buffer: needed {needed}, available {available}")]
    UnexpectedEof { needed: usize, available: usize },

    #[error("invalid varint encoding")]
    InvalidVarint,

    #[error("unknown api key {0}")]
    UnknownApiKey(i16),

    #[error("{remaining} trailing bytes left after decode")]
    TrailingBytes { remaining: usize },

    #[error("unsupported version {version} for api key {api_key}")]
    UnsupportedVersion { api_key: i16, version: i16 },

    #[error("frame length {size} exceeds maximum {max}")]
    FrameTooLarge { size: usize, max: usize },

    #[error("null value for a field that is not nullable at this version")]
    NullForNonNullable,

    #[error("invalid UTF-8 in string field")]
    InvalidUtf8,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
