# kafka-wire-codec

A Rust SDK for the Apache Kafka wire protocol, with **all message types generated
directly from Kafka's own published protocol schemas** and **verified byte-for-byte
against the official Java `kafka-clients` library**.

Generated from Kafka **4.3.1**.

---

## Design goals

1. **Schema-driven & regenerable.** All request/response types are generated from
   Kafka's JSON protocol definitions (`clients/src/main/resources/common/message/*.json`).
   The generated code lives in a dedicated folder and can be wiped and rebuilt for
   any Kafka release.
2. **Stream-friendly and byte-array-friendly.** Decode from an `std::io::Read` /
   `tokio::io::AsyncRead` stream, or directly from an in-memory `Bytes`.
3. **Best-effort zero copy.** Decoding borrows string/bytes/records fields as
   ref-counted slices of the source buffer — no allocation or copy. Frame reads
   can reuse a caller-owned buffer (`read_frame_into[_async]`) for steady-state
   zero-allocation receive loops.
4. **Simple APIs.** Decode a header, inspect it, then decode (or forward) the body.
   The generated `RequestKind`/`ResponseKind` enums decode any `(api_key, version)`
   into a typed message without hand-written match arms.
5. **Size-first encoding.** Encoders compute the exact wire size first, allocate
   once, and write — no reallocation; the buffer flushes to a socket in one write.
6. **Typed fields.** `string` fields are `StrBytes` (zero-copy, UTF-8 validated
   once at decode — `.as_str()` is free), `uuid` fields are `uuid::Uuid`, and the
   schemas' `entityType` annotations become newtypes (`TopicName`, `GroupId`,
   `TransactionalId`, `BrokerId`, `ProducerId`) so ids of different kinds can't be
   mixed up silently. Schema-declared **tagged fields** are typed too (e.g.
   `FetchResponse.node_endpoints`, partition-level `diverging_epoch`): they encode
   into the tagged section only when non-default — exactly Kafka's own rule — and
   unknown tags are preserved raw, interleaved in ascending tag order for
   byte-exact round-trips.

7. **Payload-heavy frames without huge buffers.** The records-bearing messages
   (`ProduceRequest`, `FetchResponse`, `ShareFetchResponse`,
   `FetchSnapshotResponse`) additionally generate **Shell** variants
   (`FetchResponseShell`, …): identical typed fields, except records payloads
   are `RecordsChunks` — zero-copy chains of pool-sized chunks. A pluggable
   `BufferSupplier` picks the read path per frame from the exact length prefix.

## Payload-heavy frames: the shell path

A 55 MiB fetch response normally forces a 55 MiB contiguous buffer. The shell
path reads it as pool-sized chunks instead, decodes everything *except* the
record batches (which stay as zero-copy chunk slices), and re-encodes by
splicing the chunks back out as shared frame segments — the payload is never
contiguous and never copied:

```rust
use kafka_wire_codec::generated::fetch_response::FetchResponseShell;
use kafka_wire_codec::{frame, DefaultSupplier, SuppliedFrame};

let supplier = DefaultSupplier::default();   // contiguous ≤ 1 MiB, chunked above
match frame::read_frame_supplied(&mut stream, &supplier)? {
    // Small frame: today's fast path, decode as usual.
    SuppliedFrame::Contiguous(mut body) => { /* Message::decode(...) */ }
    // Large frame: shell decode — all metadata fields (including per-partition
    // and trailing tagged fields) fully typed; records stay chunked.
    SuppliedFrame::Chunked(mut chain) => {
        let mut shell = FetchResponseShell::decode_chained(version, &mut chain)?;
        shell.node_endpoints[0].host = "proxy.example.com".into();   // rewrite
        let mut seg = kafka_wire_codec::SegmentedBuf::new();
        shell.encode(version, &mut seg)?;                            // zero-copy splice
        let frame = kafka_wire_codec::frame::EncodedFrame::from_segments(seg);
        // frame.write_to[_async](...) — vectored, chunks pass by refcount
    }
}
```

Implement `BufferSupplier` yourself to route chunk allocation through your own
pool or spool: `strategy(frame_len)` sees the exact frame length before any
body byte is read (threshold, admission control), and `acquire(len)` provides
the buffers (malloc, borrow, pin — the codec never knows). The shell path is
verified by the compat suite: every records-bearing Java fixture is chunk-split
(down to 7-byte chunks), shell-decoded, re-encoded, and byte-compared.

## Workspace layout

```
kafka-codegen/      Binary: reads Kafka schema JSON → emits Rust source
kafka-wire-codec/   Runtime crate (published to crates.io as `kafka-wire-codec`)
  src/codec/        Primitive encode/decode/size helpers (varint, compact, …)
  src/frame/        Length-prefix framing (sync + async) + EncodedFrame
  src/header.rs     RequestHeader / ResponseHeader
  src/message.rs    Encodable trait (generic, size-first encoding)
  src/types.rs      StrBytes + entity newtypes + RecordsChunks (chunked payloads)
  src/supply.rs     BufferSupplier / ReadStrategy (pluggable buffer policy)
  src/generated/    GENERATED — one module per message + dispatch.rs + kinds.rs + KAFKA_VERSION
compat-tests/       Java fixture generator (kafka-clients) for the compat test
scripts/regen.sh    Clean → fetch schemas → regenerate → retest
```

## Quick start

```rust
use kafka_wire_codec::generated::api_versions_request::ApiVersionsRequest;
use kafka_wire_codec::generated::dispatch;
use kafka_wire_codec::header::RequestHeader;
use kafka_wire_codec::frame::frame_request;
use kafka_wire_codec::Encodable;

// Build a request body.
let req = ApiVersionsRequest {
    client_software_name: "my-client".into(),
    client_software_version: "1.0".into(),
    ..Default::default()
};

let header = RequestHeader {
    api_key: ApiVersionsRequest::API_KEY,
    api_version: 3,
    correlation_id: 1,
    client_id: Some("my-client".into()),
    tagged_fields: vec![],
};

// Header versions are derived, not hand-maintained (this also encodes the
// KIP-511 quirk: ApiVersions RESPONSE headers are always v0).
let header_version = dispatch::request_header_version(ApiVersionsRequest::API_KEY, 3).unwrap();

// Size-first framing: one allocation, then write [len][header][body] to a stream.
let frame = frame_request(&header, header_version, &req, /*api_version=*/ 3)?;
frame.write_to(&mut std::io::stdout())?;          // sync
// frame.write_to_async(&mut socket).await?;       // async (tokio)

// Proxies forwarding large payloads (produce/fetch record batches) can use the
// zero-copy path instead: `Bytes` fields become refcounted frame segments and
// are never memcpy'd. Byte-identical output, single pass, no sizing walk.
// let frame = kafka_wire_codec::frame::frame_request_zero_copy(&header, header_version, &req, 3);
```

Decoding, header-first then body:

```rust
use kafka_wire_codec::frame::read_frame_into;
use kafka_wire_codec::header::RequestHeader;
use kafka_wire_codec::generated::dispatch;
use kafka_wire_codec::RequestKind;

// A caller-owned read buffer, reused across frames: once the previous frame's
// `Bytes` views are dropped, the next read reclaims the allocation — a
// steady-state receive loop allocates nothing. (BytesMut grows itself if a
// frame is bigger than the current capacity, so undersizing is never an error.)
let mut read_buf = bytes::BytesMut::with_capacity(64 * 1024);
let mut body: bytes::Bytes = read_frame_into(&mut stream, &mut read_buf)?;
// async: frame::read_frame_into_async(&mut socket, &mut read_buf).await?

// The first 4 bytes are api_key + api_version; derive the header version from them.
let api_key = i16::from_be_bytes([body[0], body[1]]);
let api_version = i16::from_be_bytes([body[2], body[3]]);
let header_version = dispatch::request_header_version(api_key, api_version).unwrap();

// Decode just the header; `body` is left as a zero-copy slice of the remaining bytes.
let header = RequestHeader::decode(&mut body, header_version)?;

// A proxy can stop here and forward `body` untouched — or decode it into a
// typed message with the generated dispatch enum (no hand-written match):
let request = RequestKind::decode(header.api_key, header.api_version, &mut body)?;
match request {
    RequestKind::Produce(p) => { /* p is a ProduceRequest */ }
    RequestKind::Fetch(f) => { /* f is a FetchRequest */ }
    other => println!("{} (api key {})", other.name(), other.api_key()),
}
```

### Typed fields

```rust
use kafka_wire_codec::{StrBytes, TopicName, BrokerId, Uuid};

// string fields are StrBytes: zero-copy slices of the frame, UTF-8 validated
// once at decode time — as_str() is free, and Deref gives you &str ergonomics.
let name: &str = &decoded.topic_data[0].name;           // TopicName -> StrBytes -> str

// entityType-annotated fields are newtypes; uuid fields are uuid::Uuid.
let topic = TopicName::from_static("events");
let broker: BrokerId = 3.into();
let topic_id: Uuid = metadata.topics[0].topic_id;       // real Uuid, not [u8; 16]
```

## Versioning model

- One struct per message; version-gating is applied at **runtime** via an `i16`
  version argument. Fields absent in a version simply keep their `Default`.
- **Version-range contract (symmetric):** decoding an unsupported version
  returns `Err(DecodeError::UnsupportedVersion)`, and encoding at one returns
  `Err(EncodeError::UnsupportedVersion)` — no panics on either side. Callers
  that pre-negotiate versions (`Encodable::supports_version()` or the
  `VALID_MIN/MAX_VERSION` constants) can treat the encode error as
  unreachable. The one remaining panic is a caller-constructed invariant
  violation: a `None` in a field that is not nullable at the requested version.
- The caller supplies the API version. Discover what a broker supports by sending
  an `ApiVersionsRequest` (api_key 18) on connect — exactly like every production
  Kafka client.
- The Kafka release the code was generated from is embedded as
  **`kafka_wire_codec::KAFKA_VERSION`** (e.g. `"4.3.1"`) and exposed to users.

### Crate version vs. Kafka version

The crate version and the Kafka schema tag are **deliberately independent**:

- **Crate semver describes the Rust API contract.** Major = breaking Rust API
  changes; minor = new APIs or a newer Kafka schema tag; patch = fixes at the
  same tag. The changelog states the Kafka tag for every release.
- **The Kafka tag is a recorded input, not the version number.** The same tag
  can ship in many crate releases (codegen improvements regenerate identical
  protocol coverage with a better API), and published crate versions are
  immutable — the only way to improve code generated from an old tag is a new
  crate release. Read the tag at runtime from `KAFKA_VERSION`; the compat suite
  guarantees it matches the `kafka-clients` jar the bytes were verified against.
- Because Kafka schemas are cumulative, moving to a newer tag only *adds*
  protocol versions; it is treated as a minor release. New Kafka versions add
  fields to existing structs, so **always construct messages with
  `..Default::default()`** — exhaustive struct literals may stop compiling
  across minor releases (and would silently miss new schema defaults anyway).

## Regenerating for a different Kafka version

```sh
scripts/regen.sh 4.3.1        # clean, fetch schemas, regenerate, run compat
SKIP_COMPAT=1 scripts/regen.sh 4.2.1   # regenerate only
```

Schemas are pulled from the GitHub **release tarball** (codeload), so there are no
API rate limits. Running `kafka-codegen` directly is also supported:

```sh
cargo run -p kafka-codegen                 # interactive: pick from 10 newest releases (or type a tag)
cargo run -p kafka-codegen -- 4.3.1        # non-interactive
KAFKA_SCHEMA_DIR=/path/to/message cargo run -p kafka-codegen -- 4.3.1   # offline, from local JSON
```

Set `GITHUB_TOKEN` to raise the API rate limit if you use the API-based path.

## Compatibility test

The compat test (`kafka-wire-codec/tests/compat.rs`) is the correctness backbone.

- A Java `FixtureGenerator` enumerates **every RPC (request + response) at every
  supported version**, reflectively populates **every field** — including nested
  structs, collections, **tagged fields**, and nullable structs/arrays — and emits
  **small / medium / large (up to 55 MiB)** payload variants for `records`/`bytes`
  fields.
- Everything is serialized with the official `kafka-clients` JAR into one
  self-describing fixture file.
- For each record the Rust side asserts the **round-trip success criteria**:
  1. decode succeeds,
  2. the entire body is consumed (no trailing/short read),
  3. re-encoding reproduces the Java bytes **exactly, byte-for-byte**.

The Java client version is taken from `kafka_wire_codec::KAFKA_VERSION`, so the
fixtures are always built against the same Kafka release the Rust code was
generated from (the test asserts this).

```sh
cargo test --test compat            # builds fixtures via Maven, then verifies
SKIP_COMPAT_TESTS=1 cargo test      # skip (no Java/Maven available)
```

Current status: **1002 records, 0 failed** against `kafka-clients:4.3.1`. The
test also asserts the fixtures genuinely populate schema-declared tagged fields
(e.g. `FetchResponse.node_endpoints`), so the typed tagged-field path can never
pass vacuously.

Requires JDK 11+ and Maven on `PATH`.

## Performance characteristics

**Decode — zero copy.** `string` fields decode to `StrBytes` and `bytes`/`records`
fields to `Bytes` — both are slices sharing the source buffer's allocation (via
`Bytes::split_to`); there is no per-field copy (UTF-8 is validated in place, once).
A frame is read with a single allocation — or **zero** allocations in steady state
with `read_frame_into[_async]`, which appends into a caller-owned `BytesMut` and
reclaims it once the previous frame's views are dropped. The header can be decoded
while the body remains a borrowed slice for deferred decoding or pass-through.

**Encode — size-first, single allocation.** `encoded_size()` computes the exact
size, then `encode()` writes into one exact-capacity `BytesMut` (no reallocation),
which is flushed to the socket in a single write. The generic `Encodable` trait and
`frame_request` / `frame_response` helpers expose this uniformly.

> The one remaining copy on the encode path is large `records`/`bytes` payloads
> being written into the frame buffer (`put_slice`). Avoiding that for multi-MiB
> payloads would require vectored/scatter-gather writes — a possible future
> optimization. Everything else is allocation-minimal.

**Generated-loop efficiency.** Fixed-size primitive arrays compute their size with
a single multiplication (no element loop); only variable-size element arrays
(structs, strings) loop, which is unavoidable. Encoding never calls `encoded_size`
internally, so there is no O(n²) sizing. The two passes (size then write) are
intentional — they are what enables the single, exact allocation.

## License

Protocol schemas belong to the Apache Kafka project (Apache-2.0).
