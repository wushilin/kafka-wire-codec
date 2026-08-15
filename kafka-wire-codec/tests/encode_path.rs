//! Exercises the size-first encode path, the Encodable trait, framing helpers,
//! and header-first / body-later decoding (the proxy pattern).

use kafka_wire_codec::frame::{frame_request, read_frame};
use kafka_wire_codec::generated::api_constants::ApiKey;
use kafka_wire_codec::generated::api_versions_request::ApiVersionsRequest;
use kafka_wire_codec::generated::dispatch;
use kafka_wire_codec::header::RequestHeader;
use kafka_wire_codec::{Encodable, StrBytes};

fn sample() -> ApiVersionsRequest {
    ApiVersionsRequest {
        client_software_name: "my-client".into(),
        client_software_version: "1.0".into(),
        ..Default::default()
    }
}

#[test]
fn encodable_to_bytes_is_size_exact() {
    let req = sample();
    let version = 3;
    let buf = req.to_bytes(version).unwrap();
    // Size-first: the buffer length equals the pre-computed wire size exactly.
    assert_eq!(buf.len(), req.wire_size(version).unwrap());

    // And it round-trips.
    let mut b = buf.freeze();
    let decoded = ApiVersionsRequest::decode(version, &mut b).unwrap();
    assert!(b.is_empty(), "decode consumed the whole body");
    assert_eq!(decoded.client_software_name, req.client_software_name);
    assert_eq!(decoded.client_software_version, req.client_software_version);
}

#[test]
fn generated_api_and_version_constants_are_exposed() {
    assert_eq!(ApiKey::API_VERSIONS.as_i16(), 18);
    assert_eq!(ApiKey::API_VERSIONS_KEY, ApiVersionsRequest::API_KEY);
    assert_eq!(
        ApiKey::from_i16(ApiVersionsRequest::API_KEY),
        Some(ApiKey::API_VERSIONS)
    );

    assert_eq!(
        <ApiVersionsRequest as Encodable>::VALID_MIN_VERSION,
        ApiVersionsRequest::VALID_MIN_VERSION
    );
    assert_eq!(
        <ApiVersionsRequest as Encodable>::VALID_MAX_VERSION,
        ApiVersionsRequest::VALID_MAX_VERSION
    );
}

#[test]
fn frame_request_then_header_first_decode() {
    let version = 3;
    let header_version = 2; // flexible request header
    let header = RequestHeader {
        api_key: ApiVersionsRequest::API_KEY,
        api_version: version,
        correlation_id: 42,
        client_id: Some(StrBytes::from_static("my-client")),
        tagged_fields: vec![],
    };
    let req = sample();

    // Size-first whole-frame build, then write [len][header][body] to a Vec.
    let frame = frame_request(&header, header_version, &req, version).unwrap();
    let mut wire: Vec<u8> = Vec::new();
    frame.write_to(&mut wire).unwrap();

    // Read the frame back (one allocation), then decode ONLY the header.
    let mut body = read_frame(&mut wire.as_slice()).unwrap();
    let decoded_header = RequestHeader::decode(&mut body, header_version).unwrap();
    assert_eq!(decoded_header.api_key, ApiVersionsRequest::API_KEY);
    assert_eq!(decoded_header.correlation_id, 42);

    // `body` is now a zero-copy slice of the remaining bytes — decode it on demand.
    let reencoded = dispatch::roundtrip(
        decoded_header.api_key,
        true,
        decoded_header.api_version,
        &body,
    )
    .unwrap();
    assert_eq!(reencoded, body, "body round-trips byte-for-byte");
}
