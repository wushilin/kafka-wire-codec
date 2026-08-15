# Changelog

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
