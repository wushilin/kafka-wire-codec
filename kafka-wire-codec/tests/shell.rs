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
fn pooled_supplier_reuses_chunks_across_frames() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(64, 8);

    let mut wire = (100i32).to_be_bytes().to_vec();
    wire.extend_from_slice(&[0x61u8; 100]);

    // Frame 1: chunked (100 > 64), ceil(100/64) = 2 chunks, all fresh.
    let f1 = frame::read_frame_supplied(&mut wire.as_slice(), &pool).unwrap();
    let ptrs1: Vec<usize> = match &f1 {
        SuppliedFrame::Chunked(ch) => {
            assert_eq!(ch.remaining(), 100);
            // Capture allocation identities via the pool stats instead of
            // reaching into the chain; drop returns them below.
            vec![]
        }
        _ => panic!("expected chunked"),
    };
    let _ = ptrs1;
    let s = pool.stats();
    assert_eq!((s.in_flight, s.created, s.reused), (2, 2, 0));

    // Dropping the frame returns both chunks to standby.
    drop(f1);
    let s = pool.stats();
    assert_eq!((s.in_flight, s.standby), (0, 2));

    // Frame 2: served entirely from standby — zero new allocations.
    let mut wire2 = (100i32).to_be_bytes().to_vec();
    wire2.extend_from_slice(&[0x62u8; 100]);
    let f2 = frame::read_frame_supplied(&mut wire2.as_slice(), &pool).unwrap();
    let s = pool.stats();
    assert_eq!((s.in_flight, s.created, s.reused), (2, 2, 2));
    assert_eq!(s.high_watermark, 2);
    drop(f2);

    // Small frame (≤ chunk_size): contiguous fast path, still pooled.
    let mut wire3 = (5i32).to_be_bytes().to_vec();
    wire3.extend_from_slice(b"small");
    let f3 = frame::read_frame_supplied(&mut wire3.as_slice(), &pool).unwrap();
    assert!(matches!(f3, SuppliedFrame::Contiguous(_)));
    assert_eq!(pool.stats().reused, 3);
    drop(f3);

    // trim() frees standby.
    pool.trim();
    assert_eq!(pool.stats().standby, 0);
}

#[test]
fn pooled_supplier_returns_only_after_last_slice_drops() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(1 << 20, 8);

    let version = 16i16;
    let records = Bytes::from(vec![0x41u8; 2 << 20]); // 2 MiB -> chunked
    let inbound = big_fetch_response(records).to_bytes(version).unwrap().freeze();
    let mut framed = ((inbound.len() as i32).to_be_bytes()).to_vec();
    framed.extend_from_slice(&inbound);

    let mut ch = match frame::read_frame_supplied(&mut framed.as_slice(), &pool).unwrap() {
        SuppliedFrame::Chunked(ch) => ch,
        _ => panic!("expected chunked"),
    };
    let shell = FetchResponseShell::decode_chained(version, &mut ch).unwrap();
    drop(ch);
    // Cargo slices still hold the chunks: nothing returned yet.
    assert_eq!(pool.stats().standby, 0);
    assert!(pool.stats().in_flight > 0);

    // Encode into an outbound frame (chunks pass by refcount), drop the shell.
    // The two 1 MiB cargo chunks stay pinned by the frame segments; the third
    // chunk's tiny cargo tail fell below the zero-copy threshold and was
    // coalesced (copied) into the scratch segment, so that chunk returns to
    // the pool EARLY — a feature, not a leak.
    let mut seg = SegmentedBuf::new();
    shell.encode(version, &mut seg).unwrap();
    let out = EncodedFrame::from_segments(seg);
    drop(shell);
    let s = pool.stats();
    assert!(s.in_flight >= 2, "big cargo chunks must stay pinned");
    assert!(s.standby <= 1, "at most the coalesced tail chunk returns early");

    // Once the outbound frame is dropped (== written), every chunk returns.
    drop(out);
    let s = pool.stats();
    assert_eq!(s.in_flight, 0);
    assert_eq!(s.standby, s.created.min(8));
}

#[test]
fn pooled_supplier_watermark_frees_excess() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(16, 1); // retain at most ONE standby chunk
    let mut wire = (48i32).to_be_bytes().to_vec();
    wire.extend_from_slice(&[0x61u8; 48]);
    let f = frame::read_frame_supplied(&mut wire.as_slice(), &pool).unwrap();
    assert_eq!(pool.stats().in_flight, 3);
    drop(f);
    let s = pool.stats();
    // 3 returned, but only 1 retained; 2 freed at the watermark.
    assert_eq!((s.in_flight, s.standby), (0, 1));
}

#[test]
fn failed_reads_do_not_leak_in_flight_or_lose_reuse() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(64, 8);

    // A flaky upstream: the frame declares 32 body bytes but the stream dies
    // after 10. Every read fails mid-body, AFTER acquire but BEFORE seal.
    for _ in 0..100 {
        let mut wire = (32i32).to_be_bytes().to_vec();
        wire.extend_from_slice(&[0x61u8; 10]);
        let err = frame::read_frame_supplied(&mut wire.as_slice(), &pool);
        assert!(err.is_err());
    }

    // Counters stay exact and the aborted chunk is restocked every time:
    // ONE allocation ever, 99 reuses, nothing in flight.
    let s = pool.stats();
    assert_eq!(s.in_flight, 0, "aborted reads must not leak in_flight");
    assert_eq!(s.created, 1, "aborted chunks must be restocked, not freed");
    assert_eq!(s.reused, 99);
    assert_eq!(s.standby, 1);
    assert_eq!(s.high_watermark, 1);

    // And a subsequent good frame reuses the same chunk.
    let mut wire = (5i32).to_be_bytes().to_vec();
    wire.extend_from_slice(b"hello");
    let f = frame::read_frame_supplied(&mut wire.as_slice(), &pool).unwrap();
    assert_eq!(pool.stats().created, 1);
    assert_eq!(pool.stats().reused, 100);
    drop(f);
    assert_eq!(pool.stats().in_flight, 0);
}

#[tokio::test]
async fn failed_async_reads_do_not_leak() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(16, 8);
    // Chunked path: 40-byte body (3 chunks) but the stream dies after 20 —
    // two chunks seal successfully, the third aborts mid-fill.
    for _ in 0..50 {
        let mut wire = (40i32).to_be_bytes().to_vec();
        wire.extend_from_slice(&[0x61u8; 20]);
        assert!(frame::read_frame_supplied_async(&mut wire.as_slice(), &pool)
            .await
            .is_err());
    }
    let s = pool.stats();
    assert_eq!(s.in_flight, 0);
    assert!(s.created <= 3, "steady-state flaky reads must reuse chunks");
}

/// An AsyncRead that yields its data then stalls forever (Pending) — the
/// canonical cancellation setup: the read future suspends mid-body and gets
/// DROPPED (by timeout/select/task abort), so neither Ok nor Err paths run.
struct StallingReader {
    data: Vec<u8>,
    pos: usize,
}

impl tokio::io::AsyncRead for StallingReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if self.pos < self.data.len() {
            let n = (self.data.len() - self.pos).min(buf.remaining());
            buf.put_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            std::task::Poll::Ready(Ok(()))
        } else {
            // Never wakes: the surrounding timeout cancels (drops) the future.
            std::task::Poll::Pending
        }
    }
}

#[tokio::test(start_paused = true)]
async fn cancelled_async_reads_do_not_leak() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(64, 8);

    // 100 reads cancelled mid-body: frame declares 32 bytes, stream stalls
    // after 10, the timeout DROPS the suspended future. Only Drop of live
    // locals runs — the AcquireGuard's Drop aborts the buffer back to the pool.
    for _ in 0..100 {
        let mut data = (32i32).to_be_bytes().to_vec();
        data.extend_from_slice(&[0x61u8; 10]);
        let mut reader = StallingReader { data, pos: 0 };
        let cancelled = tokio::time::timeout(
            std::time::Duration::from_millis(5),
            frame::read_frame_supplied_async(&mut reader, &pool),
        )
        .await;
        assert!(cancelled.is_err(), "read must be cancelled, not complete");
    }

    let s = pool.stats();
    assert_eq!(s.in_flight, 0, "cancelled reads must not leak in_flight");
    assert_eq!(s.created, 1, "cancelled chunks must be restocked, not freed");
    assert_eq!(s.reused, 99);
    assert_eq!(s.standby, 1);
    assert_eq!(s.high_watermark, 1);
}

#[tokio::test(start_paused = true)]
async fn cancelled_chunked_async_reads_do_not_leak() {
    use kafka_wire_codec::PooledSupplier;
    let pool = PooledSupplier::new(16, 8);

    // Chunked path: 40-byte body (3 chunks), stream stalls after 20 — chunk 1
    // seals, chunk 2 is mid-fill when the future is dropped. The sealed chunk
    // returns via its owner's Drop; the in-fill chunk via the guard's abort.
    for _ in 0..100 {
        let mut data = (40i32).to_be_bytes().to_vec();
        data.extend_from_slice(&[0x61u8; 20]);
        let mut reader = StallingReader { data, pos: 0 };
        let cancelled = tokio::time::timeout(
            std::time::Duration::from_millis(5),
            frame::read_frame_supplied_async(&mut reader, &pool),
        )
        .await;
        assert!(cancelled.is_err());
    }

    let s = pool.stats();
    assert_eq!(s.in_flight, 0, "cancelled chunked reads must not leak");
    assert!(s.created <= 2, "both chunks must recycle across cancellations");
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
