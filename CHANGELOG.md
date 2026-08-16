# Changelog

## 0.7.0 — 2026-08-16

Pool observability + tunability (all always-on, free when idle):

- `PoolStats` gains `freed` (watermark rejections + trims; invariant
  `created == in_flight + standby + freed`), `aborted` (failed/cancelled
  reads), `lock_contended`, and `lock_wait_nanos`. The struct is now
  `#[non_exhaustive]` so future counters are non-breaking.
- Lock-wait timing costs nothing when uncontended: the standby lock is taken
  `try_lock`-first, and clocks are read only when a wait actually happens —
  so there is no profiling switch; everything is always on.
- `PooledSupplier::contiguous_max(n)` (construction-time) decouples the
  contiguous threshold from `chunk_size` for callers who want small frames
  chunk-pooled but mid-size frames chunked.
- Concurrency regression test: 8 threads × 500 frames, counters balance
  exactly.


## 0.6.2 — 2026-08-16

Bugfix: **async cancellation no longer leaks pool counters or reuse.** An
async read has three exits, not two: Ok (seal), Err (abort, fixed in 0.6.1),
and the future being DROPPED at a suspension point (timeout, `select!`, task
abort) — where neither call site runs, only `Drop` of live locals. Acquired
buffers are now held in an internal drop guard for their whole
acquire-to-seal window: success defuses the guard and seals; error returns
and cancellation both drop it, which runs `abort`. All four read paths
(sync/async × contiguous/chunked) use the guard; sealed sibling chunks of a
cancelled chunked read return via their owners' drops as before. The
`BufferSupplier` pairing contract now explicitly covers cancellation.
Regression-tested with 100 timeouts cancelling reads mid-body (contiguous
and chunked): `created` stays 1, `in_flight` returns to 0, reuse intact.


## 0.6.1 — 2026-08-16

Bugfix: **aborted reads no longer leak pool counters or reuse.** `acquire`
incremented `in_flight`, but the decrement lived in the seal-attached drop
hook — so a read failing mid-body (flaky upstream) leaked one `in_flight`
count per failure, inflated `high_watermark` forever, and freed the chunk
instead of restocking it. New defaulted `BufferSupplier::abort(buf)` completes
the pairing contract (every `acquire` is followed by exactly one `seal` on
success or `abort` on failure, called by the codec at all four read-failure
sites); `PooledSupplier::abort` decrements and restocks, so flaky peers cost
neither counter accuracy nor reuse. Regression-tested with 100 consecutive
mid-body failures: `created` stays 1, `in_flight` returns to 0. Restock takes
the standby mutex only for a `Vec::push`; watermark-excess frees happen
outside the lock.


## 0.6.0 — 2026-08-16

Kafka schema tag: **4.3.1** (unchanged). Compat: 1002/1002, 68 shell-verified.

The chunk return path + a built-in pool:

- `BufferSupplier::seal(buf) -> Bytes` (defaulted: freeze, so 0.5.0 suppliers
  keep working unchanged): the codec now routes every filled buffer through
  the supplier, giving pools a real reclaim hook. This fixes a 0.5.0 design
  gap — `acquire` hands out unique `BytesMut`, so without `seal` a pooling
  supplier had no way to ever see its chunks again.
- `PooledSupplier::new(chunk_size, max_standby)`: batteries-included uniform
  chunk pool, no trait implementation needed. Chunks are wrapped in a
  drop-returning owner (`Bytes::from_owner`), so they return to the pool when
  the LAST slice referencing them drops — on whatever thread finishes the
  outbound write. Frames that fit one chunk are read contiguously into a
  single pooled block; `stats()` (in-flight / standby / created / reused /
  high-watermark) and `trim()` included. Chunks whose only surviving content
  was coalesced below the zero-copy threshold return early.
- `bytes` dependency raised to 1.9 (for `Bytes::from_owner`).

## 0.5.0 — 2026-08-16

Kafka schema tag: **4.3.1** (unchanged). Compat: 1002/1002 records
byte-for-byte, including 68 shell-path verifications.

New — the shell (chunked-payload) path:

- Generated `*Shell` variants for the records-bearing messages
  (`ProduceRequestShell`, `FetchResponseShell`, `ShareFetchResponseShell`,
  `FetchSnapshotResponseShell` + nested): identical typed fields, but records
  payloads are `RecordsChunks` — zero-copy chains of `Bytes` chunks. Payload-
  heavy frames never need one contiguous buffer.
- `Shell::decode_chained(version, &mut ChunkChain)` decodes everything except
  the record batches (per-partition and trailing tagged fields included);
  batches come out as zero-copy slices of the read chunks. `Shell::encode`
  splices the chunks back out as shared segments (`SegmentedBuf` /
  `EncodedFrame` vectored writes) — never contiguous, never copied.
- `BufferSupplier` trait + `ReadStrategy`: per-frame buffer policy decided
  from the exact length prefix before any body byte is read. `strategy()` is
  the threshold/admission point; `acquire()` the provider (pool, malloc,
  spool — invisible to the codec). `DefaultSupplier` (1 MiB threshold and
  chunks) works with zero configuration.
- `frame::read_frame_supplied[_async][_with_limit]` → `SuppliedFrame::
  {Contiguous, Chunked}`.
- The compat suite chunk-splits every records-bearing Java fixture (7-byte
  chunks for small bodies to torture boundaries, 64 KiB for multi-MiB ones),
  shell-decodes, re-encodes, and requires byte-for-byte equality.

## 0.4.0 — 2026-08-15

Kafka schema tag: **4.3.1** (unchanged). Compat: 1002/1002 records
byte-for-byte against `kafka-clients:4.3.1`.

Breaking — symmetric Result-based encoding (matches kafka-protocol's shape):

- Top-level `encoded_size`/`encode` now return `Result<_, EncodeError>` instead
  of panicking on an out-of-range version; the new
  `EncodeError::UnsupportedVersion` mirrors the decode-side error. All 356
  generated version asserts are gone. Tombstone (version-less) APIs return
  `Err` instead of panicking too.
- `Encodable::{wire_size, write, to_bytes}` and
  `EncodableZeroCopy::{write_segmented, to_segments}` return `Result`;
  the eight `frame_*` builders return `Result<EncodedFrame, EncodeError>`.
- `RequestKind`/`ResponseKind`: `encoded_size`/`encode`/`to_bytes` return
  `Result`, and the new `to_segments()` gives the Kinds the full zero-copy
  surface (the enum-level analogue of `EncodableZeroCopy::to_segments`; a
  literal trait impl is impossible because `Encodable` carries per-message
  consts an enum spanning every API cannot provide).
- Nested (non-top-level) struct encoding stays infallible — it is only
  reachable through a top-level message that already validated the version.
- The one remaining panic is a caller-constructed invariant violation
  (`None` in a non-nullable-at-this-version field), documented on `Encodable`.

## 0.3.0 — 2026-08-15

Kafka schema tag: **4.3.1** (unchanged). Compat: 1002/1002 records
byte-for-byte against `kafka-clients:4.3.1`.

Breaking — typed tagged fields:

- Schema-declared tagged fields are now **typed struct fields** (e.g.
  `FetchResponse.node_endpoints: Vec<NodeEndpoint>`, `FetchRequest.replica_state`,
  partition-level `diverging_epoch`/`current_leader`, `FetchPartition
  .replica_directory_id: Uuid`) instead of raw entries in `tagged_fields`.
  Encoding follows Kafka's rule: a tagged field is written only when it differs
  from its schema default; an omitted tag decodes to that default. Unknown tags
  are still preserved in `tagged_fields` and re-encoded interleaved in
  ascending tag order for byte-exact round-trips.
- All generated structs (and `RequestKind`/`ResponseKind`) now derive
  `PartialEq`.

New:

- `Encodable::supports_version()`, plus a documented version-range contract:
  decode returns `Err(UnsupportedVersion)`, encode at an out-of-range version
  is a documented programmer-error panic.
- `frame_request_kind` / `frame_response_kind` (+ `_zero_copy` variants):
  framing helpers that take `RequestKind`/`ResponseKind` directly.

Compat suite:

- Fixed two Java fixture-generator bugs that silently left every
  `*Collection`-typed tagged field (and deeply nested structs) empty in the
  fixtures: reflective population no longer touches
  `ImplicitLinkedHashCollection.Element#setNext/setPrev` (which marked elements
  as already-inserted so `add()` dropped them), and `MAX_DEPTH` was raised so
  partition-level nested structs are populated. The suite now asserts tagged
  sentinels (Fetch/Produce `node_endpoints`, Fetch `diverging_epoch`) are
  genuinely populated — tagged coverage can never regress to vacuous.

## 0.2.0 — 2026-08-15

Kafka schema tag: **4.3.1** (unchanged). Compat: 964/964 records byte-for-byte
against `kafka-clients:4.3.1`.

Breaking — typed field API:

- `string` fields are now `StrBytes` (zero-copy `Bytes` wrapper, UTF-8 validated
  once at decode; `Deref<Target = str>`, `From<&'static str>`/`From<String>`).
  Invalid UTF-8 on the wire is rejected with the new `DecodeError::InvalidUtf8`.
- Schema `entityType` annotations generate newtypes: `TopicName`, `GroupId`,
  `TransactionalId` (StrBytes-backed) and `BrokerId(i32)`, `ProducerId(i64)`.
- `uuid` fields are `uuid::Uuid` (re-exported) instead of `[u8; 16]`.
- `RequestHeader::client_id` is `Option<StrBytes>`.

New:

- Generated `RequestKind` / `ResponseKind` enums (`generated/kinds.rs`): decode
  any message by `(api_key, version)` into a typed variant — no hand-written
  `match api_key` needed. With `api_key()`, `name()`, `encoded_size()`,
  `encode()`, `to_bytes()`, and `From<message>` impls.
- `frame::read_frame_into` / `read_frame_into_async` (+ `_with_limit`): read
  frames into a caller-owned `BytesMut` that is reclaimed across frames —
  steady-state receive loops allocate nothing. Undersized buffers regrow
  automatically (never an error).

## 0.1.0 — 2026-08-15

Initial release. Kafka schema tag: **4.3.1**. Zero-copy decode, size-first
single-allocation encode, zero-copy segmented encode (`SegmentedBuf` /
`EncodedFrame`), sync + async framing, generated dispatch table, Java
compat suite with 100% (api, direction, version) matrix coverage.
