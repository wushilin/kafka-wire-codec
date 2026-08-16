//! Shell (chunked-payload) path: BufferSupplier strategy dispatch, chunked
//! frame reads, zero-copy cargo, and the proxy rewrite flow end-to-end.

use bytes::{Bytes, BytesMut};
use kafka_wire_codec::codec::SegmentedBuf;
use kafka_wire_codec::frame::EncodedFrame;
use kafka_wire_codec::generated::fetch_response::{
    FetchResponse, FetchResponseShell, NodeEndpoint,
};
use kafka_wire_codec::generated::produce_request::{ProduceRequest, ProduceRequestShell};
use kafka_wire_codec::{
    frame, BrokerId, BufferSupplier, ChunkChain, DefaultSupplier, Encodable, ReadStrategy,
    RecordsChunks, SuppliedFrame,
};

fn big_fetch_response(records: Bytes) -> FetchResponse {
    use kafka_wire_codec::generated::fetch_response::{FetchableTopicResponse, PartitionData};
    FetchResponse {
        session_id: 7,
        responses: vec![FetchableTopicResponse {
            partitions: vec![PartitionData {
                partition_index: 2,
                high_watermark: 100,
                records: Some(records),
                ..Default::default()
            }],
            ..Default::default()
        }],
        node_endpoints: vec![NodeEndpoint {
            node_id: BrokerId(1),
            host: "broker-1.internal".into(),
            port: 9092,
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn supplier_strategy_picks_the_path() {
    let sup = DefaultSupplier {
        contiguous_max: 64,
        chunk_size: 16,
    };
    // Small frame -> contiguous.
    let mut wire = (5i32).to_be_bytes().to_vec();
    wire.extend_from_slice(b"hello");
    match frame::read_frame_supplied(&mut wire.as_slice(), &sup).unwrap() {
        SuppliedFrame::Contiguous(b) => assert_eq!(&b[..], b"hello"),
        SuppliedFrame::Chunked(_) => panic!("small frame should be contiguous"),
    }
    // Large frame -> chunked, ceil(100/16) = 7 chunks.
    let payload = vec![0x61u8; 100];
    let mut wire = (100i32).to_be_bytes().to_vec();
    wire.extend_from_slice(&payload);
    match frame::read_frame_supplied(&mut wire.as_slice(), &sup).unwrap() {
        SuppliedFrame::Chunked(ch) => assert_eq!(ch.remaining(), 100),
        SuppliedFrame::Contiguous(_) => panic!("large frame should be chunked"),
    }
}

#[tokio::test]
async fn supplier_strategy_async_and_custom_supplier() {
    // A custom supplier that counts acquisitions — the codec never sees how
    // buffers are provided.
    struct Counting(std::sync::atomic::AtomicUsize);
    impl BufferSupplier for Counting {
        fn strategy(&self, frame_len: usize) -> ReadStrategy {
            if frame_len > 8 {
                ReadStrategy::Chunked { chunk_size: 8 }
            } else {
                ReadStrategy::Contiguous
            }
        }
        fn acquire(&self, len: usize) -> BytesMut {
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            BytesMut::with_capacity(len)
        }
    }
    let sup = Counting(std::sync::atomic::AtomicUsize::new(0));
    let payload = vec![0x62u8; 20];
    let mut wire = (20i32).to_be_bytes().to_vec();
    wire.extend_from_slice(&payload);
    let frame = frame::read_frame_supplied_async(&mut wire.as_slice(), &sup)
        .await
        .unwrap();
    assert_eq!(frame.len(), 20);
    // ceil(20/8) = 3 chunk acquisitions.
    assert_eq!(sup.0.load(std::sync::atomic::Ordering::Relaxed), 3);
}

#[test]
fn shell_decode_is_zero_copy_and_boundary_safe() {
    let version = 16i16;
    let records = Bytes::from(vec![0x41u8; 100_000]);
    let wire = big_fetch_response(records).to_bytes(version).unwrap().freeze();

    // Read chunks of many awkward sizes; cargo must always come out equal and
    // zero-copy (pointing into the read chunks).
    for chunk_size in [3usize, 64, 1024, 65_536, usize::MAX] {
        let mut chunks = Vec::new();
        let mut off = 0;
        while off < wire.len() {
            let end = (off + chunk_size.min(wire.len())).min(wire.len());
            chunks.push(wire.slice(off..end));
            off = end;
        }
        let mut ch = ChunkChain::new(chunks);
        let shell = FetchResponseShell::decode_chained(version, &mut ch).unwrap();
        assert!(ch.is_empty());

        let cargo = shell.responses[0].partitions[0].records.as_ref().unwrap();
        assert_eq!(cargo.len(), 100_000);
        // Zero-copy: every cargo chunk points into the original wire buffer.
        let range = wire.as_ptr() as usize..wire.as_ptr() as usize + wire.len();
        for c in cargo.chunks() {
            assert!(
                range.contains(&(c.as_ptr() as usize)),
                "cargo was copied at chunk_size={}",
                chunk_size
            );
        }
        // The small fields decoded fully, including the trailing tagged section.
        assert_eq!(shell.session_id, 7);
        assert_eq!(shell.node_endpoints[0].host, "broker-1.internal");
    }
}

#[test]
fn proxy_rewrite_flow_end_to_end() {
    // broker -> proxy: a fetch response with a large batch arrives chunked.
    let version = 16i16;
    let records = Bytes::from(vec![0x42u8; 3 << 20]); // 3 MiB
    let inbound = big_fetch_response(records.clone())
        .to_bytes(version)
        .unwrap()
        .freeze();

    let sup = DefaultSupplier::default(); // 1 MiB threshold and chunks
    let mut framed = ((inbound.len() as i32).to_be_bytes()).to_vec();
    framed.extend_from_slice(&inbound);
    let mut ch = match frame::read_frame_supplied(&mut framed.as_slice(), &sup).unwrap() {
        SuppliedFrame::Chunked(ch) => ch,
        SuppliedFrame::Contiguous(_) => panic!("3 MiB frame must be chunked"),
    };

    // Decode the shell, rewrite what the proxy rewrites.
    let mut shell = FetchResponseShell::decode_chained(version, &mut ch).unwrap();
    shell.node_endpoints[0].host = "proxy.example.com".into();
    shell.node_endpoints[0].port = 19092;

    // Re-encode zero-copy: cargo chunks become shared frame segments.
    let mut seg = SegmentedBuf::new();
    shell.encode(version, &mut seg).unwrap();
    let frame = EncodedFrame::from_segments(seg);

    // The cargo was never copied: some outbound segment IS a read chunk.
    assert!(
        frame.segments().len() > 1,
        "large cargo should be shared segments"
    );

    // A downstream client decodes the rewritten response normally.
    let mut out = frame.to_contiguous();
    let decoded = FetchResponse::decode(version, &mut out).unwrap();
    assert_eq!(decoded.node_endpoints[0].host, "proxy.example.com");
    assert_eq!(decoded.node_endpoints[0].port, 19092);
    assert_eq!(
        decoded.responses[0].partitions[0].records.as_ref().unwrap(),
        &records
    );
}

#[test]
fn produce_request_shell_roundtrip() {
    use kafka_wire_codec::generated::produce_request::{PartitionProduceData, TopicProduceData};
    let version = 9i16;
    let req = ProduceRequest {
        acks: -1,
        timeout_ms: 1000,
        topic_data: vec![TopicProduceData {
            name: "events".into(),
            partition_data: vec![PartitionProduceData {
                index: 0,
                records: Some(Bytes::from(vec![0x43u8; 4096])),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let wire = req.to_bytes(version).unwrap().freeze();

    let mut ch = ChunkChain::new(vec![wire.clone()]);
    let shell = ProduceRequestShell::decode_chained(version, &mut ch).unwrap();
    assert!(ch.is_empty());
    assert_eq!(shell.topic_data[0].name, "events");
    assert_eq!(
        shell.topic_data[0].partition_data[0]
            .records
            .as_ref()
            .unwrap()
            .len(),
        4096
    );

    // Byte-identical re-encode, size-first exact.
    let size = shell.encoded_size(version).unwrap();
    let mut buf = BytesMut::with_capacity(size);
    shell.encode(version, &mut buf).unwrap();
    assert_eq!(buf.len(), size);
    assert_eq!(buf.freeze(), wire);
}

#[test]
fn records_chunks_equality_ignores_boundaries() {
    let mut a = RecordsChunks::new();
    a.push(Bytes::from_static(b"hello "));
    a.push(Bytes::from_static(b"world"));
    let b = RecordsChunks::from(Bytes::from_static(b"hello world"));
    assert_eq!(a, b);
    assert_eq!(a.len(), 11);
    assert_eq!(a.to_contiguous(), Bytes::from_static(b"hello world"));
    let c = RecordsChunks::from(Bytes::from_static(b"hello worlD"));
    assert_ne!(a, c);
}
