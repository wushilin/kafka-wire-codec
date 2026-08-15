//! Proves the zero-copy guarantees a proxy relies on:
//!  - decode: `Bytes` fields are refcounted slices of the inbound frame (no copy),
//!  - encode: large `Bytes` payloads become shared segments of the outbound
//!    frame (no copy), byte-identical to the contiguous encode path.

use bytes::Bytes;
use kafka_wire_codec::codec::{SegmentedBuf, DEFAULT_ZERO_COPY_THRESHOLD};
use kafka_wire_codec::frame::{frame_request, frame_request_zero_copy};
use kafka_wire_codec::generated::produce_request::{
    PartitionProduceData, ProduceRequest, TopicProduceData,
};
use kafka_wire_codec::header::RequestHeader;
use kafka_wire_codec::{Encodable, EncodableZeroCopy, StrBytes};

/// True if `inner` points into `outer`'s memory (i.e. shares the allocation).
fn is_subslice(outer: &[u8], inner: &[u8]) -> bool {
    let (o_start, o_end) = (outer.as_ptr() as usize, outer.as_ptr() as usize + outer.len());
    let (i_start, i_end) = (inner.as_ptr() as usize, inner.as_ptr() as usize + inner.len());
    i_start >= o_start && i_end <= o_end
}

fn sample_produce(records: Bytes) -> ProduceRequest {
    ProduceRequest {
        acks: -1,
        timeout_ms: 30_000,
        topic_data: vec![TopicProduceData {
            name: "my-topic".into(),
            partition_data: vec![PartitionProduceData {
                index: 3,
                records: Some(records),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn decode_is_zero_copy() {
    let version: i16 = 9;
    let records = Bytes::from(vec![0x62u8; 4096]);
    let req = sample_produce(records);
    let wire = req.to_bytes(version).freeze();

    let mut buf = wire.clone();
    let decoded = ProduceRequest::decode(version, &mut buf).unwrap();

    let got = decoded.topic_data[0].partition_data[0]
        .records
        .as_ref()
        .unwrap();
    assert_eq!(got.len(), 4096);
    // The decoded records field must point INTO the wire buffer — not a copy.
    assert!(
        is_subslice(&wire, got),
        "decoded records were copied instead of sliced from the frame"
    );
    // Topic name too (TopicName -> StrBytes -> zero-copy slice of the frame).
    assert!(is_subslice(&wire, decoded.topic_data[0].name.as_bytes()));
}

#[test]
fn encode_is_zero_copy_for_large_payloads() {
    let version: i16 = 9;
    let records = Bytes::from(vec![0x62u8; 1 << 20]); // 1 MiB, above threshold
    let req = sample_produce(records.clone());

    let segments = req.to_segments(version).into_segments();

    // Exactly one segment must BE the records payload (same allocation).
    let shared: Vec<_> = segments
        .iter()
        .filter(|s| s.as_ptr() == records.as_ptr() && s.len() == records.len())
        .collect();
    assert_eq!(
        shared.len(),
        1,
        "large records payload was copied instead of shared"
    );

    // And the concatenation is byte-identical to the contiguous encode.
    let contiguous = req.to_bytes(version);
    let mut joined = Vec::with_capacity(contiguous.len());
    for s in &segments {
        joined.extend_from_slice(s);
    }
    assert_eq!(joined, contiguous.to_vec());
    assert_eq!(req.wire_size(version), joined.len());
}

#[test]
fn small_payloads_are_coalesced_not_fragmented() {
    let version: i16 = 9;
    // Below the threshold: should be copied into the scratch, yielding ONE segment.
    let records = Bytes::from(vec![0x62u8; DEFAULT_ZERO_COPY_THRESHOLD - 1]);
    let req = sample_produce(records);
    let segments = req.to_segments(version).into_segments();
    assert_eq!(
        segments.len(),
        1,
        "small payloads should coalesce into the scratch segment"
    );
}

#[test]
fn segmented_buf_threshold_zero_shares_everything() {
    let mut buf = SegmentedBuf::with_threshold(0);
    let payload = Bytes::from_static(b"xyz");
    use kafka_wire_codec::codec::WireBuf;
    buf.put_slice(b"ab");
    buf.put_shared(&payload);
    let segs = buf.into_segments();
    assert_eq!(segs.len(), 2);
    assert_eq!(segs[1].as_ptr(), payload.as_ptr());
}

#[test]
fn zero_copy_frame_matches_contiguous_frame() {
    let version: i16 = 9;
    let header_version: i16 = 2;
    let header = RequestHeader {
        api_key: ProduceRequest::API_KEY,
        api_version: version,
        correlation_id: 7,
        client_id: Some(StrBytes::from_static("proxy")),
        tagged_fields: vec![],
    };
    let records = Bytes::from(vec![0x41u8; 128 * 1024]);
    let req = sample_produce(records.clone());

    let plain = frame_request(&header, header_version, &req, version);
    let zc = frame_request_zero_copy(&header, header_version, &req, version);

    assert_eq!(plain.len(), zc.len());
    assert_eq!(plain.to_contiguous(), zc.to_contiguous());

    // The zero-copy frame carries the records payload by reference.
    assert!(zc
        .segments()
        .iter()
        .any(|s| s.as_ptr() == records.as_ptr() && s.len() == records.len()));

    // Both write identical wire bytes.
    let (mut w1, mut w2) = (Vec::new(), Vec::new());
    plain.write_to(&mut w1).unwrap();
    zc.write_to(&mut w2).unwrap();
    assert_eq!(w1, w2);
}

#[test]
fn header_version_derivation() {
    use kafka_wire_codec::generated::dispatch;
    // Produce (0): flexible from v9.
    assert_eq!(dispatch::request_header_version(0, 8), Some(1));
    assert_eq!(dispatch::request_header_version(0, 9), Some(2));
    assert_eq!(dispatch::response_header_version(0, 8), Some(0));
    assert_eq!(dispatch::response_header_version(0, 9), Some(1));
    // ApiVersions (18): request header follows flexibility, but the RESPONSE
    // header is ALWAYS v0 (KIP-511) so old clients can parse errors.
    assert_eq!(dispatch::request_header_version(18, 3), Some(2));
    assert_eq!(dispatch::response_header_version(18, 3), Some(0));
    // Unknown api key.
    assert_eq!(dispatch::request_header_version(9999, 0), None);
}

#[test]
fn version_introspection_constants() {
    use kafka_wire_codec::generated::dispatch;
    use kafka_wire_codec::generated::fetch_request::FetchRequest;
    use kafka_wire_codec::generated::metadata_request::MetadataRequest;

    // Per-message flexible threshold is exposed as a constant (Produce v9+,
    // Fetch v12+, Metadata v9+ per the Kafka schemas).
    assert_eq!(ProduceRequest::FLEXIBLE_MIN_VERSION, 9);
    assert_eq!(FetchRequest::FLEXIBLE_MIN_VERSION, 12);
    assert_eq!(MetadataRequest::FLEXIBLE_MIN_VERSION, 9);
    assert_eq!(
        <ProduceRequest as Encodable>::FLEXIBLE_MIN_VERSION,
        ProduceRequest::FLEXIBLE_MIN_VERSION
    );

    // Supported version ranges are queryable by api_key, matching the consts.
    assert_eq!(
        dispatch::supported_request_versions(ProduceRequest::API_KEY),
        Some((
            ProduceRequest::VALID_MIN_VERSION,
            ProduceRequest::VALID_MAX_VERSION
        ))
    );
    assert!(dispatch::supported_response_versions(0).is_some());
    // Unknown api key -> None; retired (version-less) APIs also -> None.
    assert_eq!(dispatch::supported_request_versions(9999), None);
    // Every non-retired matrix row agrees with the lookup.
    for &(api_key, is_request, vmin, vmax) in dispatch::MESSAGES {
        if vmin > vmax {
            continue;
        }
        let got = if is_request {
            dispatch::supported_request_versions(api_key)
        } else {
            dispatch::supported_response_versions(api_key)
        };
        assert_eq!(got, Some((vmin, vmax)), "api_key {}", api_key);
    }
}

#[test]
fn schema_defaults_are_applied() {
    use kafka_wire_codec::generated::fetch_request::FetchRequest;
    use kafka_wire_codec::generated::metadata_request::MetadataRequest;
    let fetch = FetchRequest::default();
    assert_eq!(fetch.replica_id, -1, "FetchRequest.ReplicaId default");
    assert_eq!(fetch.session_epoch, -1, "FetchRequest.SessionEpoch default");
    assert_eq!(fetch.max_bytes, 0x7fffffff, "FetchRequest.MaxBytes default");
    let md = MetadataRequest::default();
    assert!(
        md.allow_auto_topic_creation,
        "MetadataRequest.AllowAutoTopicCreation default"
    );
}

#[test]
fn nullable_struct_uses_kafka_presence_byte() {
    use kafka_wire_codec::generated::describe_topic_partitions_request::DescribeTopicPartitionsRequest;
    let req = DescribeTopicPartitionsRequest::default(); // cursor: None
    let wire = req.to_bytes(0).freeze();
    // Kafka encodes a null nullable struct as -1 (and reads any byte >= 0 as
    // present). The cursor is the last field before tagged fields.
    let mut b = wire.clone();
    let decoded = DescribeTopicPartitionsRequest::decode(0, &mut b).unwrap();
    assert!(decoded.cursor.is_none());
    assert!(
        wire.iter().rev().nth(1).map(|&x| x as i8) == Some(-1),
        "null struct must be encoded as -1, wire: {:x?}",
        &wire[..]
    );
}
