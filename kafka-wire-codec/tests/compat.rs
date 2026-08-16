//! Compatibility test against the Java `kafka-clients` library.
//!
//! A single consolidated fixture file (`compat-tests/fixtures/compat.bin`) is
//! produced by the Java `FixtureGenerator`, which serializes EVERY Kafka RPC
//! (request + response) at EVERY supported version, with all fields reflectively
//! populated (including tagged fields) and small/medium/large payload variants.
//!
//! For each record we assert the round-trip success criteria:
//!   1. decode succeeds,
//!   2. the whole body is consumed (no trailing/short read),
//!   3. re-encoding reproduces Java's bytes exactly (byte-for-byte).
//!
//! Requires Java 11+ and Maven. Set `SKIP_COMPAT_TESTS=1` to skip.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bytes::Buf;
use kafka_wire_codec::generated::dispatch;
use kafka_wire_codec::Bytes;

const MAGIC: &[u8; 4] = b"KCFX";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn fixture_file() -> PathBuf {
    workspace_root()
        .join("compat-tests")
        .join("fixtures")
        .join("compat.bin")
}

/// The Kafka version the bundled Rust code was generated from, read directly
/// from the compiled generated code — so the Java client used for the compat
/// test is guaranteed to match the Rust protocol definitions exactly.
fn generated_kafka_version() -> &'static str {
    kafka_wire_codec::KAFKA_VERSION
}

fn fixture_stale() -> bool {
    let f = fixture_file();
    if !f.exists() {
        return true;
    }
    // Stale if the sidecar version doesn't match the generated code's version.
    match fs::read_to_string(f.with_extension("version")) {
        Ok(sidecar) => sidecar.trim() != generated_kafka_version(),
        Err(_) => true,
    }
}

fn build_and_run_generator() {
    let compat_dir = workspace_root().join("compat-tests");
    let fixture = fixture_file();
    if let Some(parent) = fixture.parent() {
        fs::create_dir_all(parent).expect("create fixtures dir");
    }
    let kafka_version = generated_kafka_version();

    eprintln!(
        "Building Java fixture generator (kafka-clients {})...",
        kafka_version
    );
    let status = Command::new("mvn")
        .args([
            "package",
            "-q",
            "-DskipTests",
            &format!("-Dkafka.version={}", kafka_version),
        ])
        .current_dir(&compat_dir)
        .status()
        .expect("mvn not found — install Maven 3.x (or set SKIP_COMPAT_TESTS=1)");
    assert!(status.success(), "mvn package failed");

    eprintln!("Generating fixtures (this populates every message/version)...");
    let jar = compat_dir.join("target/kafka-compat-fixtures.jar");
    let deps = compat_dir.join("target/dependency/*");
    let classpath = format!("{}:{}", jar.display(), deps.display());
    let status = Command::new("java")
        .args([
            "-Xmx2g",
            "-cp",
            &classpath,
            &format!("-Dkafka.version={}", kafka_version),
            "com.kafka.compat.FixtureGenerator",
            fixture.to_str().unwrap(),
        ])
        .status()
        .expect("java not found — install JDK 11+");
    assert!(status.success(), "fixture generator failed");

    fs::write(fixture.with_extension("version"), generated_kafka_version()).ok();
}

fn ensure_fixtures() -> bool {
    if std::env::var("SKIP_COMPAT_TESTS").is_ok() {
        return false;
    }
    if fixture_stale() {
        build_and_run_generator();
    }
    true
}

/// Read a length-prefixed UTF-8 string from the fixture cursor.
fn read_string(r: &mut &[u8]) -> String {
    let len = r.get_i16() as usize;
    String::from_utf8(r.copy_to_bytes(len).to_vec()).unwrap()
}

struct Record {
    api_key: i16,
    version: i16,
    is_request: bool,
    label: String,
    body: Bytes,
}

#[test]
fn compat_roundtrip_all() {
    if !ensure_fixtures() {
        eprintln!("SKIP_COMPAT_TESTS set — skipping compat round-trip");
        return;
    }

    let raw = fs::read(fixture_file()).expect("read compat.bin");
    let mut r: &[u8] = &raw;

    assert_eq!(&r.copy_to_bytes(4)[..], MAGIC, "bad fixture magic");
    let format_version = r.get_i16();
    assert_eq!(format_version, 1, "unsupported fixture format version");
    let count = r.get_i32();
    let kafka_version = read_string(&mut r);
    eprintln!(
        "Compat fixtures: {} records, kafka {} (generated code: {})",
        count,
        kafka_version,
        generated_kafka_version()
    );
    // The Java client that produced the fixtures MUST match the version the Rust
    // code was generated from, or the comparison is meaningless.
    assert_eq!(
        kafka_version,
        generated_kafka_version(),
        "fixture Kafka version does not match generated code version"
    );

    let mut passed = 0u32;
    let mut skipped: Vec<String> = Vec::new();
    let mut defaults = 0u32;
    let mut failures: Vec<String> = Vec::new();
    // Every (api_key, is_request, version) cell exercised by the fixtures.
    let mut seen: std::collections::HashSet<(i16, bool, i16)> = std::collections::HashSet::new();
    // Sentinels proving the Java populator fills schema-declared TAGGED fields
    // with non-default values — otherwise the typed tagged-field codegen path
    // would pass the byte-for-byte check vacuously (nothing to encode).
    let mut fetch_node_endpoints_seen = false;
    let mut produce_node_endpoints_seen = false;
    let mut fetch_diverging_epoch_seen = false;
    // Records exercised through the shell (chunked-payload) path.
    let mut shell_passed = 0u32;

    for _ in 0..count {
        let api_key = r.get_i16();
        let version = r.get_i16();
        let is_request = r.get_u8() != 0;
        let label = read_string(&mut r);
        let body_len = r.get_i32() as usize;
        let body = r.copy_to_bytes(body_len);

        // A "default" record means Java's reflective population failed and it
        // fell back to a default-valued message: still a valid byte-level
        // check, but it exercises fewer field branches. Track it visibly.
        if label == "default" {
            defaults += 1;
        }

        let rec = Record {
            api_key,
            version,
            is_request,
            label,
            body,
        };
        // Tagged-field population sentinels (FetchResponse tag 0 at 16+,
        // partition-level DivergingEpoch tag 0 at 12+, ProduceResponse tag 0
        // at 10+).
        if api_key == 1 && !is_request && version >= 12 {
            let mut b = rec.body.clone();
            if let Ok(m) =
                kafka_wire_codec::generated::fetch_response::FetchResponse::decode(version, &mut b)
            {
                if version >= 16 && !m.node_endpoints.is_empty() {
                    fetch_node_endpoints_seen = true;
                }
                if std::env::var("COMPAT_DEBUG_TAGGED").is_ok() {
                    eprintln!(
                        "fetch resp v{} [{}]: node_endpoints={} topics={} first_partition={:?}",
                        version,
                        rec.label,
                        m.node_endpoints.len(),
                        m.responses.len(),
                        m.responses
                            .first()
                            .and_then(|t| t.partitions.first())
                            .map(|p| (p.diverging_epoch.clone(), p.tagged_fields.len()))
                    );
                }
                if m.responses.iter().flat_map(|t| t.partitions.iter()).any(|p| {
                    p.diverging_epoch
                        != kafka_wire_codec::generated::fetch_response::EpochEndOffset::default()
                }) {
                    fetch_diverging_epoch_seen = true;
                }
            }
        }
        if api_key == 0 && !is_request && version >= 10 {
            let mut b = rec.body.clone();
            if let Ok(m) = kafka_wire_codec::generated::produce_response::ProduceResponse::decode(
                version, &mut b,
            ) {
                if !m.node_endpoints.is_empty() {
                    produce_node_endpoints_seen = true;
                }
            }
        }

        // Shell (chunked) path: must reproduce the Java bytes exactly too.
        match shell_check(&rec) {
            Some(Ok(())) => shell_passed += 1,
            Some(Err(msg)) => failures.push(format!(
                "apikey={} {} v{} [{}]: {}",
                rec.api_key,
                if rec.is_request { "req" } else { "resp" },
                rec.version,
                rec.label,
                msg
            )),
            None => {}
        }

        match check(&rec) {
            CheckResult::Ok => {
                passed += 1;
                seen.insert((api_key, is_request, version));
            }
            CheckResult::Unsupported => {
                skipped.push(format!("apikey={} v{} req={}", api_key, version, is_request))
            }
            CheckResult::Fail(msg) => failures.push(msg),
        }
    }

    assert_eq!(r.remaining(), 0, "trailing bytes in fixture file");

    // ── 100% matrix coverage ──────────────────────────────────────────────────
    // Every RPC direction and every version the generated code claims to
    // support must have been exercised by at least one Java fixture record.
    let mut missing: Vec<String> = Vec::new();
    for &(api_key, is_request, vmin, vmax) in dispatch::MESSAGES {
        assert!(
            vmax != i16::MAX,
            "apikey={} has an unbounded version range — matrix check impossible",
            api_key
        );
        for v in vmin..=vmax {
            if !seen.contains(&(api_key, is_request, v)) {
                missing.push(format!(
                    "apikey={} {} v{} has no Java fixture record",
                    api_key,
                    if is_request { "req" } else { "resp" },
                    v
                ));
            }
        }
    }

    eprintln!(
        "Compat results: {} passed ({} default-populated, {} shell-verified), {} skipped, {} failed, {} matrix cells missing",
        passed,
        defaults,
        shell_passed,
        skipped.len(),
        failures.len(),
        missing.len()
    );

    if !failures.is_empty() {
        let shown: Vec<_> = failures.iter().take(40).cloned().collect();
        panic!(
            "{} compat round-trip failures (showing {}):\n{}",
            failures.len(),
            shown.len(),
            shown.join("\n")
        );
    }
    assert!(
        skipped.is_empty(),
        "fixture records with no Rust dispatch entry (Java knows an API we don't):\n{}",
        skipped.join("\n")
    );
    assert!(
        missing.is_empty(),
        "{} (api, direction, version) cells NOT covered by fixtures:\n{}",
        missing.len(),
        missing.join("\n")
    );
    // Every record must be fully populated: a "default" record means the Java
    // populator failed on some message shape and the record exercises almost
    // no field branches. Extend FixtureGenerator.populate/synth if this fires.
    assert_eq!(
        defaults, 0,
        "{} fixture records fell back to default population — coverage is weakened",
        defaults
    );
    assert!(passed > 0, "no records were checked");
    assert!(
        fetch_node_endpoints_seen,
        "no fixture populated FetchResponse.node_endpoints (tag 0) — tagged-field coverage is vacuous"
    );
    assert!(
        produce_node_endpoints_seen,
        "no fixture populated ProduceResponse.node_endpoints (tag 0) — tagged-field coverage is vacuous"
    );
    assert!(
        fetch_diverging_epoch_seen,
        "no fixture populated FetchResponse partition diverging_epoch (tag 0) — tagged-field coverage is vacuous"
    );
    assert!(
        shell_passed > 0,
        "no records-bearing fixtures went through the shell (chunked) path"
    );
}

enum CheckResult {
    Ok,
    Unsupported,
    Fail(String),
}

/// Shell (chunked-payload) round-trip for the records-bearing messages: split
/// the Java bytes into chunks (7-byte chunks for small bodies to torture every
/// boundary; 64 KiB for the multi-MiB ones), decode via `decode_chained`,
/// re-encode, and require byte-for-byte equality with the fixture.
/// Returns None for messages without a shell.
fn shell_check(rec: &Record) -> Option<Result<(), String>> {
    use kafka_wire_codec::generated::fetch_response::{FetchResponse, FetchResponseShell};
    use kafka_wire_codec::generated::fetch_snapshot_response::{
        FetchSnapshotResponse, FetchSnapshotResponseShell,
    };
    use kafka_wire_codec::generated::produce_request::{ProduceRequest, ProduceRequestShell};
    use kafka_wire_codec::generated::share_fetch_response::{
        ShareFetchResponse, ShareFetchResponseShell,
    };
    use kafka_wire_codec::ChunkChain;

    let chunk = if rec.body.len() > (1 << 20) {
        64 * 1024
    } else {
        7
    };
    let mut chunks = Vec::new();
    let mut off = 0;
    while off < rec.body.len() {
        let end = (off + chunk).min(rec.body.len());
        chunks.push(rec.body.slice(off..end));
        off = end;
    }
    let mut ch = ChunkChain::new(chunks);

    macro_rules! go {
        ($shell:ty) => {{
            let m = match <$shell>::decode_chained(rec.version, &mut ch) {
                Ok(m) => m,
                Err(e) => return Some(Err(format!("shell decode: {}", e))),
            };
            if !ch.is_empty() {
                return Some(Err(format!(
                    "shell decode left {} trailing bytes",
                    ch.remaining()
                )));
            }
            let size = match m.encoded_size(rec.version) {
                Ok(s) => s,
                Err(e) => return Some(Err(format!("shell encoded_size: {}", e))),
            };
            let mut buf = bytes::BytesMut::with_capacity(size);
            if let Err(e) = m.encode(rec.version, &mut buf) {
                return Some(Err(format!("shell encode: {}", e)));
            }
            if buf.len() != size {
                return Some(Err(format!(
                    "shell size-first mismatch: sized {} wrote {}",
                    size,
                    buf.len()
                )));
            }
            if buf[..] != rec.body[..] {
                return Some(Err(format!(
                    "shell byte mismatch (java={} shell={} bytes)",
                    rec.body.len(),
                    buf.len()
                )));
            }
            Some(Ok(()))
        }};
    }

    match (rec.api_key, rec.is_request) {
        (k, true) if k == ProduceRequest::API_KEY => go!(ProduceRequestShell),
        (k, false) if k == FetchResponse::API_KEY => go!(FetchResponseShell),
        (k, false) if k == ShareFetchResponse::API_KEY => go!(ShareFetchResponseShell),
        (k, false) if k == FetchSnapshotResponse::API_KEY => go!(FetchSnapshotResponseShell),
        _ => None,
    }
}

fn check(rec: &Record) -> CheckResult {
    let kind = if rec.is_request { "req" } else { "resp" };
    let tag = format!(
        "apikey={} {} v{} [{}]",
        rec.api_key, kind, rec.version, rec.label
    );

    match dispatch::roundtrip(rec.api_key, rec.is_request, rec.version, &rec.body) {
        Ok(reencoded) => {
            if reencoded == rec.body {
                CheckResult::Ok
            } else {
                CheckResult::Fail(format!(
                    "{}: byte mismatch (java={} rust={} bytes)",
                    tag,
                    rec.body.len(),
                    reencoded.len()
                ))
            }
        }
        Err(kafka_wire_codec::DecodeError::UnknownApiKey(_)) => CheckResult::Unsupported,
        Err(e) => CheckResult::Fail(format!("{}: {}", tag, e)),
    }
}
