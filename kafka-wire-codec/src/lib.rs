pub mod codec;
pub mod error;
pub mod frame;
pub mod generated;
pub mod header;
pub mod message;
pub mod types;

pub use bytes::{Bytes, BytesMut};
pub use codec::{SegmentedBuf, WireBuf, DEFAULT_ZERO_COPY_THRESHOLD};
pub use error::{DecodeError, EncodeError};
pub use generated::api_constants::ApiKey;
pub use generated::kinds::{RequestKind, ResponseKind};
pub use message::{Encodable, EncodableZeroCopy};
pub use types::{BrokerId, GroupId, ProducerId, StrBytes, TopicName, TransactionalId};
pub use uuid::Uuid;

/// The Apache Kafka release tag the bundled protocol definitions were generated
/// from (e.g. "4.0.0"). Matches `generated::KAFKA_VERSION`.
pub use generated::KAFKA_VERSION;
