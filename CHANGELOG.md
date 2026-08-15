# Changelog

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
