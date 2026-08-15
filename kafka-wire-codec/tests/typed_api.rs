//! Covers the typed API surface: RequestKind/ResponseKind dispatch enums,
//! StrBytes UTF-8 guarantees, entity newtypes, Uuid fields, and the
//! buffer-reusing frame readers.

use bytes::{Bytes, BytesMut};
use kafka_wire_codec::generated::metadata_response::{MetadataResponse, MetadataResponseTopic};
use kafka_wire_codec::generated::produce_request::{
    PartitionProduceData, ProduceRequest, TopicProduceData,
};
use kafka_wire_codec::{
    frame, BrokerId, DecodeError, Encodable, RequestKind, ResponseKind, StrBytes, TopicName, Uuid,
};

fn sample_produce() -> ProduceRequest {
    ProduceRequest {
        acks: -1,
        timeout_ms: 30_000,
        topic_data: vec![TopicProduceData {
            name: TopicName::from_static("my-topic"),
            partition_data: vec![PartitionProduceData {
                index: 3,
                records: Some(Bytes::from_static(b"fake-batch")),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn request_kind_roundtrip_without_hand_written_match() {
    let version: i16 = 9;
    let req = sample_produce();
    let wire = req.to_bytes(version).freeze();

    // One decode call, no `match api_key` at the call site.
    let mut buf = wire.clone();
    let kind = RequestKind::decode(ProduceRequest::API_KEY, version, &mut buf).unwrap();
    assert!(buf.is_empty());
    assert_eq!(kind.api_key(), ProduceRequest::API_KEY);
    assert_eq!(kind.name(), "Produce");

    // Typed access via a single variant match.
    let RequestKind::Produce(decoded) = &kind else {
        panic!("wrong variant");
    };
    assert_eq!(decoded.topic_data[0].name, "my-topic");

    // Generic re-encode straight off the enum, byte-identical.
    assert_eq!(kind.encoded_size(version), wire.len());
    assert_eq!(kind.to_bytes(version).freeze(), wire);

    // From<message> for ergonomic construction.
    let kind2: RequestKind = sample_produce().into();
    assert_eq!(kind2.to_bytes(version).freeze(), wire);

    assert!(matches!(
        RequestKind::decode(9999, 0, &mut wire.clone()),
        Err(DecodeError::UnknownApiKey(9999))
    ));
}

#[test]
fn response_kind_roundtrip() {
    let version: i16 = 12;
    let resp = MetadataResponse {
        topics: vec![MetadataResponseTopic {
            name: Some(TopicName::from_static("t1")),
            topic_id: Uuid::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0),
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = resp.to_bytes(version).freeze();

    let kind = ResponseKind::decode(MetadataResponse::API_KEY, version, &mut wire.clone()).unwrap();
    assert_eq!(kind.name(), "Metadata");
    let ResponseKind::Metadata(decoded) = &kind else {
        panic!("wrong variant");
    };
    // Typed topic id: a real uuid::Uuid, not [u8; 16].
    assert_eq!(
        decoded.topics[0].topic_id,
        Uuid::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0)
    );
    assert_eq!(kind.to_bytes(version).freeze(), wire);
}

#[test]
fn strings_are_utf8_validated_and_zero_copy() {
    let version: i16 = 9;
    let wire = sample_produce().to_bytes(version).freeze();
    let decoded = ProduceRequest::decode(version, &mut wire.clone()).unwrap();

    // StrBytes gives &str access with no further checks or copies.
    let name: &TopicName = &decoded.topic_data[0].name;
    assert_eq!(name.as_str(), "my-topic");
    let s: &str = name; // Deref chain TopicName -> StrBytes -> str

    // ...and it still points into the wire buffer (zero-copy).
    let range = wire.as_ptr() as usize..wire.as_ptr() as usize + wire.len();
    assert!(range.contains(&(s.as_ptr() as usize)));

    // Invalid UTF-8 on the wire is rejected at decode time.
    let mut req = sample_produce();
    req.topic_data[0].name =
        TopicName(unsafe { StrBytes::from_utf8_unchecked(Bytes::from_static(b"\xff\xfe")) });
    let bad = req.to_bytes(version).freeze();
    assert!(matches!(
        ProduceRequest::decode(version, &mut bad.clone()),
        Err(DecodeError::InvalidUtf8)
    ));
}

#[test]
fn entity_newtypes_are_distinct_types() {
    // Compile-time check by construction: brokers are BrokerId, not raw i32.
    let resp = MetadataResponse::default();
    let _controller: BrokerId = resp.controller_id;
    // The schema default (-1) is applied inside the newtype.
    assert_eq!(resp.controller_id, BrokerId(-1));
    // From/Into both directions.
    let b: BrokerId = 7.into();
    let raw: i32 = b.into();
    assert_eq!(raw, 7);
}

fn frame_bytes(payload: &[u8]) -> Vec<u8> {
    let mut wire = (payload.len() as i32).to_be_bytes().to_vec();
    wire.extend_from_slice(payload);
    wire
}

#[test]
fn read_frame_into_reuses_the_buffer() {
    // Two frames back to back on the "socket".
    let mut wire = frame_bytes(b"frame-one");
    wire.extend_from_slice(&frame_bytes(b"frame-two"));
    let mut reader = wire.as_slice();

    let mut buf = BytesMut::with_capacity(64);
    let cap = buf.capacity();

    let f1 = frame::read_frame_into(&mut reader, &mut buf).unwrap();
    assert_eq!(&f1[..], b"frame-one");
    drop(f1); // release the view so the allocation can be reclaimed
    let f2 = frame::read_frame_into(&mut reader, &mut buf).unwrap();
    assert_eq!(&f2[..], b"frame-two");
    drop(f2);

    // Steady state: no regrow happened — the caller's allocation was reused.
    assert!(buf.capacity() <= cap, "buffer should not have grown");
}

#[tokio::test]
async fn read_frame_into_async_reuses_the_buffer() {
    let mut wire = frame_bytes(b"alpha");
    wire.extend_from_slice(&frame_bytes(b"beta"));
    let mut reader = wire.as_slice();

    let mut buf = BytesMut::with_capacity(64);
    let cap = buf.capacity();

    let f1 = frame::read_frame_into_async(&mut reader, &mut buf).await.unwrap();
    assert_eq!(&f1[..], b"alpha");
    drop(f1);
    let f2 = frame::read_frame_into_async(&mut reader, &mut buf).await.unwrap();
    assert_eq!(&f2[..], b"beta");
    drop(f2);

    assert!(buf.capacity() <= cap, "buffer should not have grown");

    // An undersized buffer is not an error — BytesMut grows itself.
    let mut small = BytesMut::with_capacity(1);
    let mut reader = frame_bytes(&[0x61u8; 4096]).to_vec();
    let f = frame::read_frame_into_async(&mut reader.as_slice(), &mut small)
        .await
        .unwrap();
    assert_eq!(f.len(), 4096);
    let _ = &mut reader;
}

#[test]
fn multibyte_utf8_strings_roundtrip_at_legacy_and_compact_versions() {
    use kafka_wire_codec::generated::delete_topics_request::DeleteTopicsRequest;
    // Multibyte content: wire lengths are BYTE counts, not char counts — the
    // size-first encoder must agree with the actual bytes written.
    let req = DeleteTopicsRequest {
        topic_names: vec![
            TopicName::from_static("主题-日志"),
            TopicName::from_static("café-☕"),
            TopicName::from_static(""),
        ],
        timeout_ms: 1000,
        ..Default::default()
    };
    // v1 = legacy (int16-length strings), v5 = flexible (compact varint strings).
    for version in [1i16, 5] {
        let wire = req.to_bytes(version).freeze();
        assert_eq!(wire.len(), req.encoded_size(version), "v{version} size-first mismatch");
        let mut buf = wire.clone();
        let decoded = DeleteTopicsRequest::decode(version, &mut buf).unwrap();
        assert!(buf.is_empty());
        assert_eq!(decoded.topic_names, req.topic_names, "v{version}");
        assert_eq!(decoded.topic_names[0].as_str(), "主题-日志");
    }
}

#[test]
fn invalid_utf8_is_rejected_on_both_string_encodings() {
    use kafka_wire_codec::generated::delete_topics_request::DeleteTopicsRequest;
    let bad = unsafe { StrBytes::from_utf8_unchecked(Bytes::from_static(&[0xe4, 0xbd])) };
    let req = DeleteTopicsRequest {
        topic_names: vec![TopicName(bad)],
        ..Default::default()
    };
    // Legacy (v1) and compact (v5) string decoders both validate.
    for version in [1i16, 5] {
        let wire = req.to_bytes(version).freeze();
        assert!(
            matches!(
                DeleteTopicsRequest::decode(version, &mut wire.clone()),
                Err(DecodeError::InvalidUtf8)
            ),
            "v{version} should reject invalid UTF-8"
        );
    }
}

#[test]
fn nullable_transactional_id_and_producer_id_default() {
    use kafka_wire_codec::generated::init_producer_id_request::InitProducerIdRequest;
    use kafka_wire_codec::{ProducerId, TransactionalId};

    // Schema defaults land inside the newtypes.
    let d = InitProducerIdRequest::default();
    assert_eq!(d.producer_id, ProducerId(-1));
    assert_eq!(d.transactional_id, Some(TransactionalId::default()));

    let version = 4i16;
    // Some and None both round-trip through the nullable-string encoding.
    for txn_id in [Some(TransactionalId::from_static("txn-1")), None] {
        let req = InitProducerIdRequest {
            transactional_id: txn_id.clone(),
            transaction_timeout_ms: 60_000,
            ..Default::default()
        };
        let wire = req.to_bytes(version).freeze();
        assert_eq!(wire.len(), req.encoded_size(version));
        let decoded = InitProducerIdRequest::decode(version, &mut wire.clone()).unwrap();
        assert_eq!(decoded.transactional_id, txn_id);
    }
}

#[test]
fn broker_id_arrays_roundtrip() {
    use kafka_wire_codec::generated::metadata_response::MetadataResponsePartition;
    let version = 12i16;
    let resp = MetadataResponse {
        topics: vec![MetadataResponseTopic {
            name: Some(TopicName::from_static("t")),
            partitions: vec![MetadataResponsePartition {
                partition_index: 0,
                leader_id: BrokerId(1),
                replica_nodes: vec![BrokerId(1), BrokerId(2), BrokerId(3)],
                isr_nodes: vec![BrokerId(1), BrokerId(3)],
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = resp.to_bytes(version).freeze();
    assert_eq!(wire.len(), resp.encoded_size(version));
    let decoded = MetadataResponse::decode(version, &mut wire.clone()).unwrap();
    let p = &decoded.topics[0].partitions[0];
    assert_eq!(p.leader_id, BrokerId(1));
    assert_eq!(p.replica_nodes, vec![BrokerId(1), BrokerId(2), BrokerId(3)]);
    assert_eq!(p.isr_nodes, vec![BrokerId(1), BrokerId(3)]);
}

#[test]
fn uuid_wire_format_is_rfc_big_endian() {
    let version = 12i16;
    let id = Uuid::parse_str("0102 0304-0506-0708-090a-0b0c0d0e0f10".replace(' ', "").as_str())
        .unwrap();
    let resp = MetadataResponse {
        topics: vec![MetadataResponseTopic {
            name: Some(TopicName::from_static("t")),
            topic_id: id,
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = resp.to_bytes(version).freeze();
    // The 16 uuid bytes must appear on the wire exactly as the RFC big-endian
    // byte sequence — the same layout Java's Uuid(mostSigBits, leastSigBits) writes.
    let expected: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert!(
        wire.windows(16).any(|w| w == expected),
        "uuid bytes not found in wire encoding"
    );
    // Default is the nil uuid, and it round-trips.
    assert_eq!(MetadataResponseTopic::default().topic_id, Uuid::nil());
    let decoded = MetadataResponse::decode(version, &mut wire.clone()).unwrap();
    assert_eq!(decoded.topics[0].topic_id, id);
}

#[test]
fn tagged_node_endpoints_are_typed_fields() {
    use kafka_wire_codec::generated::fetch_response::{FetchResponse, NodeEndpoint};

    let resp = FetchResponse {
        node_endpoints: vec![NodeEndpoint {
            node_id: BrokerId(5),
            host: "broker-5.internal".into(),
            port: 9092,
            rack: Some("rack-a".into()),
            ..Default::default()
        }],
        ..Default::default()
    };

    // v16+: encoded as tagged field 0, size-first exact, and decodes back into
    // the TYPED field — nothing lands in the raw bucket.
    let version = 16i16;
    let wire = resp.to_bytes(version).freeze();
    assert_eq!(wire.len(), resp.encoded_size(version));
    let decoded = FetchResponse::decode(version, &mut wire.clone()).unwrap();
    assert_eq!(decoded.node_endpoints, resp.node_endpoints);
    assert_eq!(decoded.node_endpoints[0].node_id, BrokerId(5));
    assert!(decoded.tagged_fields.is_empty());

    // Below the taggedVersions floor the field is not encoded at all.
    let wire15 = resp.to_bytes(15).freeze();
    let decoded15 = FetchResponse::decode(15, &mut wire15.clone()).unwrap();
    assert!(decoded15.node_endpoints.is_empty());

    // Default value ⇒ tag omitted: a default message encodes zero tagged
    // fields and round-trips to default.
    let empty = FetchResponse::default().to_bytes(version).freeze();
    let d = FetchResponse::decode(version, &mut empty.clone()).unwrap();
    assert!(d.node_endpoints.is_empty() && d.tagged_fields.is_empty());
    assert_eq!(FetchResponse::default().to_bytes(version), empty);
}

#[test]
fn tagged_produce_response_node_endpoints_roundtrip() {
    use kafka_wire_codec::generated::produce_response::{NodeEndpoint, ProduceResponse};
    let version = 10i16;
    let resp = ProduceResponse {
        node_endpoints: vec![NodeEndpoint {
            node_id: BrokerId(1),
            host: "h".into(),
            port: 1,
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = resp.to_bytes(version).freeze();
    assert_eq!(wire.len(), resp.encoded_size(version));
    let decoded = ProduceResponse::decode(version, &mut wire.clone()).unwrap();
    assert_eq!(decoded.node_endpoints, resp.node_endpoints);
    assert_eq!(decoded.to_bytes(version).freeze(), wire);
}

#[test]
fn unknown_tags_are_preserved_and_interleaved_in_ascending_order() {
    use kafka_wire_codec::generated::fetch_request::{FetchRequest, ReplicaState};

    // FetchRequest v15 knows tag 0 (ClusterId, 12+) and tag 1 (ReplicaState,
    // 15+). Encode with cluster_id UNSET but a raw tag 0 carrying a valid
    // cluster-id encoding, plus a trailing unknown tag 200: the encoder must
    // drain raw tag 0 BEFORE typed tag 1 (ascending order) and tag 200 after.
    let version = 15i16;
    let req = FetchRequest {
        max_wait_ms: 500,
        replica_state: ReplicaState {
            replica_id: BrokerId(3),
            replica_epoch: 9,
            ..Default::default()
        },
        // b"\x02s" = compact string "s" — what ClusterId would encode as.
        tagged_fields: vec![(0, Bytes::from_static(b"\x02s")), (200, Bytes::from_static(b"xy"))],
        ..Default::default()
    };

    let wire = req.to_bytes(version).freeze();
    assert_eq!(wire.len(), req.encoded_size(version));

    // Decode routes tag 0 into the TYPED cluster_id (it is schema-known),
    // keeps unknown tag 200 raw, and fills replica_state from typed tag 1.
    let decoded = FetchRequest::decode(version, &mut wire.clone()).unwrap();
    assert_eq!(decoded.cluster_id.as_ref().unwrap().as_str(), "s");
    assert_eq!(decoded.replica_state, req.replica_state);
    assert_eq!(decoded.tagged_fields, vec![(200, Bytes::from_static(b"xy"))]);

    // Byte-exact round-trip: the raw-tag-0 encode and the typed-cluster_id
    // encode must produce identical wire order (0, 1, 200). A wrong interleave
    // would decode field-equal but produce different bytes.
    assert_eq!(decoded.to_bytes(version).freeze(), wire);

    // A tagged uuid at partition level (FetchPartition.ReplicaDirectoryId,
    // tag 0 from v17) decodes into the typed field, not the raw bucket.
    use kafka_wire_codec::generated::fetch_request::{FetchPartition, FetchTopic};
    let req17 = FetchRequest {
        topics: vec![FetchTopic {
            partitions: vec![FetchPartition {
                partition: 4,
                replica_directory_id: Uuid::from_u128(7),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire17 = req17.to_bytes(17).freeze();
    assert_eq!(wire17.len(), req17.encoded_size(17));
    let decoded17 = FetchRequest::decode(17, &mut wire17.clone()).unwrap();
    let p = &decoded17.topics[0].partitions[0];
    assert_eq!(p.replica_directory_id, Uuid::from_u128(7));
    assert!(p.tagged_fields.is_empty());
    // ...and below v17 the same field is silently omitted (default rule).
    let wire16 = req17.to_bytes(16).freeze();
    let decoded16 = FetchRequest::decode(16, &mut wire16.clone()).unwrap();
    assert_eq!(
        decoded16.topics[0].partitions[0].replica_directory_id,
        Uuid::nil()
    );
}

#[test]
fn tagged_partition_level_structs_roundtrip() {
    use kafka_wire_codec::generated::fetch_response::{
        EpochEndOffset, FetchResponse, FetchableTopicResponse, LeaderIdAndEpoch, PartitionData,
    };
    let version = 16i16;
    let resp = FetchResponse {
        responses: vec![FetchableTopicResponse {
            partitions: vec![PartitionData {
                partition_index: 1,
                diverging_epoch: EpochEndOffset {
                    epoch: 5,
                    end_offset: 42,
                    ..Default::default()
                },
                current_leader: LeaderIdAndEpoch {
                    leader_id: BrokerId(2),
                    leader_epoch: 7,
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = resp.to_bytes(version).freeze();
    assert_eq!(wire.len(), resp.encoded_size(version));
    let decoded = FetchResponse::decode(version, &mut wire.clone()).unwrap();
    let p = &decoded.responses[0].partitions[0];
    assert_eq!(p.diverging_epoch.epoch, 5);
    assert_eq!(p.diverging_epoch.end_offset, 42);
    assert_eq!(p.current_leader.leader_id, BrokerId(2));
    assert_eq!(decoded.to_bytes(version).freeze(), wire);
}

#[test]
fn kind_frame_helpers_match_message_framing() {
    use kafka_wire_codec::frame::{frame_request, frame_request_kind, frame_request_kind_zero_copy};
    use kafka_wire_codec::header::RequestHeader;

    let version = 9i16;
    let req = sample_produce();
    let header = RequestHeader {
        api_key: ProduceRequest::API_KEY,
        api_version: version,
        correlation_id: 1,
        client_id: Some("proxy".into()),
        tagged_fields: vec![],
    };
    let plain = frame_request(&header, 2, &req, version);
    let kind: RequestKind = req.into();
    let via_kind = frame_request_kind(&header, 2, &kind, version);
    let via_kind_zc = frame_request_kind_zero_copy(&header, 2, &kind, version);
    assert_eq!(plain.to_contiguous(), via_kind.to_contiguous());
    assert_eq!(plain.to_contiguous(), via_kind_zc.to_contiguous());
}

#[test]
fn supports_version_gates_encoding() {
    assert!(ProduceRequest::supports_version(ProduceRequest::VALID_MIN_VERSION));
    assert!(ProduceRequest::supports_version(ProduceRequest::VALID_MAX_VERSION));
    assert!(!ProduceRequest::supports_version(99));
    assert!(!ProduceRequest::supports_version(-1));
    // Decode of an unsupported version is an Err, not a panic.
    assert!(matches!(
        ProduceRequest::decode(99, &mut Bytes::new()),
        Err(DecodeError::UnsupportedVersion { api_key: 0, version: 99 })
    ));
}

#[test]
fn header_client_id_is_str_bytes() {
    use kafka_wire_codec::header::RequestHeader;
    let h = RequestHeader {
        api_key: 0,
        api_version: 9,
        correlation_id: 1,
        client_id: Some("console-producer".into()),
        tagged_fields: vec![],
    };
    let mut buf = BytesMut::new();
    h.encode(&mut buf, 2);
    let decoded = RequestHeader::decode(&mut buf.freeze(), 2).unwrap();
    assert_eq!(decoded.client_id.as_ref().unwrap().as_str(), "console-producer");
}
