mod generator;
mod schema;

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

const GITHUB_API: &str = "https://api.github.com";
const KAFKA_REPO: &str = "apache/kafka";
const SCHEMA_PATH: &str = "clients/src/main/resources/common/message";

/// Resolve the generated output dir relative to this crate's manifest, not the
/// process CWD — `cargo run` runs from the workspace root, which would otherwise
/// push files outside the workspace.
fn generated_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("kafka-codegen has a parent workspace dir")
        .join("kafka-wire-codec")
        .join("src")
        .join("generated")
}

fn main() -> Result<()> {
    // Optional GITHUB_TOKEN raises the API rate limit from 60/hr to 5000/hr.
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            if let Ok(mut v) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))
            {
                v.set_sensitive(true);
                headers.insert(reqwest::header::AUTHORIZATION, v);
            }
        }
    }
    let client = Client::builder()
        .user_agent("kafka-wire-protocol-codegen/0.1")
        .default_headers(headers)
        .build()?;

    // Non-interactive selection: first CLI arg or KAFKA_CODEGEN_TAG env var.
    // This makes the generator scriptable (CI, compat tests) while still
    // offering an interactive picker when run by a human with no tag given.
    let preset_tag = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("KAFKA_CODEGEN_TAG").ok());

    // When a tag is given explicitly we skip listing all tags — that avoids an
    // extra (rate-limited) GitHub API call and works offline-ish via raw fetches.
    let chosen_tag = match preset_tag {
        Some(tag) => {
            println!("Using specified Kafka tag: {}", tag);
            tag
        }
        None => {
            println!("Fetching Kafka release tags from GitHub...");
            let tags = fetch_tags(&client)?;
            if tags.is_empty() {
                anyhow::bail!("No tags found in apache/kafka repository");
            }
            // Show the 10 newest STABLE releases (drop -rc / -incubating etc.),
            // plus a final option to type any tag by hand.
            let mut stable: Vec<String> = tags
                .into_iter()
                .map(|t| t.name)
                .filter(|n| !n.contains('-'))
                .collect();
            stable.sort_by_key(|v| std::cmp::Reverse(version_key(v)));
            stable.dedup();
            stable.truncate(10);

            let mut items = stable.clone();
            items.push("Enter a custom tag…".to_string());
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select Kafka version to generate from")
                .items(&items)
                .default(0)
                .interact()?;

            if selection == stable.len() {
                let custom: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter Kafka tag (e.g. 4.3.1)")
                    .interact_text()?;
                custom.trim().to_string()
            } else {
                stable[selection].clone()
            }
        }
    };
    let chosen_tag = &chosen_tag;
    println!("Generating from tag: {}", chosen_tag);

    // Load schema (filename, content) pairs from a local dir (KAFKA_SCHEMA_DIR,
    // offline/reliable) or from GitHub. The local dir is populated by the
    // regenerate script from the release tarball — no API rate limits.
    let schema_dir = std::env::var("KAFKA_SCHEMA_DIR").ok();
    let schemas: Vec<(String, String)> = match &schema_dir {
        Some(dir) => {
            println!("Reading schemas from local dir: {}", dir);
            load_schemas_from_dir(dir)?
        }
        None => {
            let files = fetch_schema_file_list(&client, chosen_tag)?;
            println!("Found {} schema files on GitHub", files.len());
            files
                .into_iter()
                .map(|f| {
                    let url = format!(
                        "https://raw.githubusercontent.com/{}/{}/{}/{}",
                        KAFKA_REPO, chosen_tag, SCHEMA_PATH, f
                    );
                    // error_for_status: a 404/429 body must not be silently
                    // treated as schema text (it would be skipped as
                    // "non-RPC", leaving an incomplete generated tree).
                    let content = client.get(&url).send()?.error_for_status()?.text()?;
                    Ok((f, content))
                })
                .collect::<Result<Vec<_>>>()?
        }
    };
    println!("Loaded {} schema files", schemas.len());

    let out_dir = generated_dir();

    // Wipe and recreate generated directory
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).context("Failed to remove existing generated directory")?;
    }
    fs::create_dir_all(&out_dir)?;

    let pb = ProgressBar::new(schemas.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")?,
    );

    let mut mod_entries: Vec<String> = Vec::new();
    // (api_key, is_request, module_name, struct_name, min_version, max_version,
    // flexible_min_version) for the dispatch table, the coverage matrix, and
    // the header-version derivation functions.
    let mut dispatch_entries: Vec<(i16, bool, String, String, i16, i16, i16)> = Vec::new();
    // api_key -> base API name for api_constants.rs.
    let mut api_key_entries: BTreeMap<i16, String> = BTreeMap::new();

    for (file_name, raw) in &schemas {
        pb.set_message(file_name.clone());

        // Kafka JSON files begin with // license comment lines — strip them before parsing.
        let json_text: String = raw
            .lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");

        // Skip non-JSON or malformed files gracefully. Header/data schema files
        // (RequestHeader, ResponseHeader, *Record, …) are not RPC messages and are
        // expected to be skipped silently. But a parse failure on a file that
        // claims to be a request/response is a real coverage gap — warn loudly via
        // eprintln (pb.println is swallowed when output isn't a TTY).
        let looks_like_rpc = json_text.contains("\"type\": \"request\"")
            || json_text.contains("\"type\": \"response\"");
        let spec: schema::MessageSpec = match serde_json::from_str(&json_text) {
            Ok(s) => s,
            Err(e) => {
                if looks_like_rpc {
                    eprintln!("WARNING: failed to parse RPC schema {}: {}", file_name, e);
                }
                pb.inc(1);
                continue;
            }
        };

        // Only generate request/response messages (skip headers/data files)
        if spec.msg_type != "request" && spec.msg_type != "response" {
            pb.inc(1);
            continue;
        }

        let code = generator::generate_message(&spec);
        let mod_name = spec.name.to_snake_case();
        let out_file = out_dir.join(format!("{}.rs", mod_name));
        fs::write(&out_file, code)?;
        mod_entries.push(mod_name.clone());

        if let Some(api_key) = spec.api_key {
            if api_key >= 0 {
                let base_name = spec
                    .name
                    .strip_suffix("Request")
                    .or_else(|| spec.name.strip_suffix("Response"))
                    .unwrap_or(&spec.name)
                    .to_string();
                api_key_entries.entry(api_key).or_insert(base_name);
                let (vmin, vmax) = schema::parse_versions(&spec.valid_versions);
                let (flex_min, _) = schema::parse_versions(&spec.flexible_versions);
                dispatch_entries.push((
                    api_key,
                    spec.msg_type == "request",
                    mod_name,
                    spec.name.clone(),
                    vmin,
                    vmax,
                    flex_min,
                ));
            }
        }

        pb.inc(1);
    }
    pb.finish_with_message("Done");

    // Emit the dispatch table so the compat test can round-trip ANY message
    // by (api_key, is_request, version) without hand-maintaining a match arm.
    fs::write(
        out_dir.join("dispatch.rs"),
        generate_dispatch(&dispatch_entries),
    )?;
    mod_entries.push("dispatch".to_string());

    // Emit the typed RequestKind/ResponseKind dispatch enums.
    fs::write(out_dir.join("kinds.rs"), generate_kinds(&dispatch_entries))?;
    mod_entries.push("kinds".to_string());

    // Emit the API-key constants once per Kafka API.
    fs::write(
        out_dir.join("api_constants.rs"),
        generate_api_constants(&api_key_entries),
    )?;
    mod_entries.push("api_constants".to_string());

    // Write mod.rs
    let mut mod_rs = String::from(
        "// This directory is fully generated by kafka-codegen.\n\
         // Do not edit manually — run `cargo run -p kafka-codegen` to regenerate.\n\n",
    );
    mod_rs.push_str(
        "/// The Apache Kafka release tag these protocol definitions were generated from.\n",
    );
    mod_rs.push_str(&format!(
        "pub const KAFKA_VERSION: &str = \"{}\";\n\n",
        chosen_tag
    ));
    mod_entries.sort();
    for m in &mod_entries {
        mod_rs.push_str(&format!("pub mod {};\n", m));
    }
    fs::write(out_dir.join("mod.rs"), mod_rs)?;

    println!(
        "Generated {} message modules from tag {}",
        mod_entries.len(),
        chosen_tag
    );
    println!("Output: {}", out_dir.display());

    Ok(())
}

/// Emit `dispatch.rs`: a single `roundtrip()` that decodes any message by
/// (api_key, is_request, version), re-encodes it, and returns the bytes — plus a
/// `decode_debug()` for inspection. Auto-covers every generated message type.
fn generate_dispatch(entries: &[(i16, bool, String, String, i16, i16, i16)]) -> String {
    let mut out = String::new();
    out.push_str("// Generated dispatch table — do not edit.\n");
    out.push_str("use bytes::{Bytes, BytesMut};\n");
    out.push_str("use crate::error::DecodeError;\n\n");

    // Full coverage matrix: one row per generated RPC direction, with its
    // supported version range. The compat test asserts every cell of this
    // matrix is exercised by the Java fixtures — provable 100% coverage.
    let mut sorted_matrix = entries.to_vec();
    sorted_matrix.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    out.push_str("/// Every generated RPC: (api_key, is_request, min_version, max_version).\n");
    out.push_str("pub const MESSAGES: &[(i16, bool, i16, i16)] = &[\n");
    for (api_key, is_request, _, name, vmin, vmax, _) in &sorted_matrix {
        out.push_str(&format!(
            "    ({}, {}, {}, {}), // {}\n",
            api_key,
            is_request,
            vmin,
            if *vmax == i16::MAX {
                "i16::MAX".to_string()
            } else {
                vmax.to_string()
            },
            name
        ));
    }
    out.push_str("];\n\n");

    // Supported-version lookup: what a proxy needs to build an
    // ApiVersionsResponse advertising its ranges, or to clamp a client's
    // requested version. Tombstone APIs (empty version range) return None.
    out.push_str("/// Supported (min, max) protocol versions for `api_key`, taken from the\n");
    out.push_str("/// request schema. None for unknown api keys and retired (version-less) APIs.\n");
    out.push_str("pub fn supported_request_versions(api_key: i16) -> Option<(i16, i16)> {\n");
    out.push_str("    match api_key {\n");
    for (key, is_request, _, name, vmin, vmax, _) in &sorted_matrix {
        if !*is_request || vmin > vmax {
            continue;
        }
        out.push_str(&format!(
            "        {} => Some(({}, {})), // {}\n",
            key, vmin, vmax, name
        ));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("/// Supported (min, max) protocol versions for the RESPONSE of `api_key`.\n");
    out.push_str("/// None for unknown api keys and retired (version-less) APIs.\n");
    out.push_str("pub fn supported_response_versions(api_key: i16) -> Option<(i16, i16)> {\n");
    out.push_str("    match api_key {\n");
    for (key, is_request, _, name, vmin, vmax, _) in &sorted_matrix {
        if *is_request || vmin > vmax {
            continue;
        }
        out.push_str(&format!(
            "        {} => Some(({}, {})), // {}\n",
            key, vmin, vmax, name
        ));
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Header-version derivation (mirrors Java's ApiMessageType.headerVersion):
    // flexible request => header v2 else v1; flexible response => header v1
    // else v0; ApiVersionsResponse is ALWAYS header v0 (KIP-511) so that
    // pre-flexible clients can still parse an UNSUPPORTED_VERSION error; and
    // ControlledShutdown request v0 (if present) uses header v0.
    out.push_str("/// Header version for a request of `api_key` at `api_version`.\n");
    out.push_str("/// Returns None for unknown api keys.\n");
    out.push_str("pub fn request_header_version(api_key: i16, api_version: i16) -> Option<i16> {\n");
    out.push_str("    match api_key {\n");
    for (key, is_request, _, name, _, _, flex_min) in &sorted_matrix {
        if !*is_request {
            continue;
        }
        let flexible = flex_expr(*flex_min);
        if *key == 7 {
            out.push_str(&format!(
                "        {} => Some(if api_version == 0 {{ 0 }} else if {} {{ 2 }} else {{ 1 }}), // {}\n",
                key, flexible, name
            ));
        } else {
            out.push_str(&format!(
                "        {} => Some(if {} {{ 2 }} else {{ 1 }}), // {}\n",
                key, flexible, name
            ));
        }
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("/// Header version for a response of `api_key` at `api_version`.\n");
    out.push_str("/// Returns None for unknown api keys.\n");
    out.push_str("pub fn response_header_version(api_key: i16, api_version: i16) -> Option<i16> {\n");
    out.push_str("    match api_key {\n");
    for (key, is_request, _, name, _, _, flex_min) in &sorted_matrix {
        if *is_request {
            continue;
        }
        if *key == 18 {
            out.push_str(&format!(
                "        {} => Some(0), // {} — always header v0 (KIP-511)\n",
                key, name
            ));
        } else {
            out.push_str(&format!(
                "        {} => Some(if {} {{ 1 }} else {{ 0 }}), // {}\n",
                key,
                flex_expr(*flex_min),
                name
            ));
        }
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("/// Decode `body` as the message identified by (api_key, is_request) at\n");
    out.push_str("/// `version`, assert the whole body was consumed, then re-encode and return\n");
    out.push_str("/// the produced bytes. The compat test compares these against the input.\n");
    out.push_str("pub fn roundtrip(api_key: i16, is_request: bool, version: i16, body: &Bytes)\n");
    out.push_str("    -> Result<Bytes, DecodeError>\n{\n");
    out.push_str("    let mut buf = body.clone();\n");
    out.push_str("    let encoded = match (api_key, is_request) {\n");

    // Stable order: by api_key, then request before response.
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    for (api_key, is_request, module, name, _, _, _) in &sorted {
        // The version was just decoded successfully, so re-encoding at it
        // cannot hit EncodeError::UnsupportedVersion.
        out.push_str(&format!(
            "        ({}, {}) => {{\n            let m = super::{}::{}::decode(version, &mut buf)?;\n            let mut e = BytesMut::with_capacity(m.encoded_size(version).expect(\"just decoded at this version\"));\n            m.encode(version, &mut e).expect(\"just decoded at this version\");\n            e\n        }}\n",
            api_key, is_request, module, name
        ));
    }

    out.push_str("        _ => return Err(DecodeError::UnknownApiKey(api_key)),\n");
    out.push_str("    };\n");
    out.push_str("    if !buf.is_empty() {\n");
    out.push_str("        return Err(DecodeError::TrailingBytes { remaining: buf.len() });\n");
    out.push_str("    }\n");
    out.push_str("    Ok(encoded.freeze())\n");
    out.push_str("}\n");

    out
}

/// Emit `kinds.rs`: `RequestKind`/`ResponseKind` enums with one variant per
/// generated RPC, so callers dispatch on a typed message instead of writing a
/// 50+-arm `match api_key` by hand.
fn generate_kinds(entries: &[(i16, bool, String, String, i16, i16, i16)]) -> String {
    let mut out = String::new();
    out.push_str("// Generated typed dispatch enums — do not edit.\n");
    out.push_str("use bytes::{Bytes, BytesMut};\n");
    out.push_str("use crate::codec::{SegmentedBuf, WireBuf};\n");
    out.push_str("use crate::error::{DecodeError, EncodeError};\n\n");

    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    for is_request in [true, false] {
        let kind = if is_request {
            "RequestKind"
        } else {
            "ResponseKind"
        };
        let word = if is_request { "request" } else { "response" };
        let rows: Vec<_> = sorted.iter().filter(|e| e.1 == is_request).collect();
        let variant = |name: &str| -> String {
            name.strip_suffix("Request")
                .or_else(|| name.strip_suffix("Response"))
                .unwrap_or(name)
                .to_string()
        };

        out.push_str(&format!(
            "/// Every Kafka {word}, as one typed enum: decode by api key, match by variant.\n"
        ));
        out.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        out.push_str(&format!("pub enum {} {{\n", kind));
        for (_, _, module, name, _, _, _) in &rows {
            out.push_str(&format!(
                "    {}(super::{}::{}),\n",
                variant(name),
                module,
                name
            ));
        }
        out.push_str("}\n\n");

        out.push_str(&format!("impl {} {{\n", kind));

        out.push_str(&format!(
            "    /// Decode the {word} body for `api_key` at `version`.\n"
        ));
        out.push_str(
            "    /// Returns `DecodeError::UnknownApiKey` for api keys this build doesn't know.\n",
        );
        out.push_str("    pub fn decode(api_key: i16, version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {\n");
        out.push_str("        match api_key {\n");
        for (api_key, _, module, name, _, _, _) in &rows {
            out.push_str(&format!(
                "            {} => Ok(Self::{}(super::{}::{}::decode(version, buf)?)),\n",
                api_key,
                variant(name),
                module,
                name
            ));
        }
        out.push_str("            _ => Err(DecodeError::UnknownApiKey(api_key)),\n");
        out.push_str("        }\n    }\n\n");

        out.push_str("    /// The Kafka API key of the contained message.\n");
        out.push_str("    pub fn api_key(&self) -> i16 {\n");
        out.push_str("        match self {\n");
        for (api_key, _, _, name, _, _, _) in &rows {
            out.push_str(&format!(
                "            Self::{}(_) => {},\n",
                variant(name),
                api_key
            ));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    /// The API name of the contained message (e.g. \"Produce\").\n");
        out.push_str("    pub fn name(&self) -> &'static str {\n");
        out.push_str("        match self {\n");
        for (_, _, _, name, _, _, _) in &rows {
            let v = variant(name);
            out.push_str(&format!("            Self::{}(_) => \"{}\",\n", v, v));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    /// Exact encoded size at `version` (size-first encoding).\n");
        out.push_str("    pub fn encoded_size(&self, version: i16) -> Result<usize, EncodeError> {\n");
        out.push_str("        match self {\n");
        for (_, _, _, name, _, _, _) in &rows {
            out.push_str(&format!(
                "            Self::{}(m) => m.encoded_size(version),\n",
                variant(name)
            ));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    /// Encode the contained message at `version`.\n");
        out.push_str(
            "    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) -> Result<(), EncodeError> {\n",
        );
        out.push_str("        match self {\n");
        for (_, _, _, name, _, _, _) in &rows {
            out.push_str(&format!(
                "            Self::{}(m) => m.encode(version, buf),\n",
                variant(name)
            ));
        }
        out.push_str("        }\n    }\n\n");

        out.push_str("    /// Size-first encode into a freshly allocated, exact-capacity buffer.\n");
        out.push_str("    pub fn to_bytes(&self, version: i16) -> Result<BytesMut, EncodeError> {\n");
        out.push_str(
            "        let mut buf = BytesMut::with_capacity(self.encoded_size(version)?);\n",
        );
        out.push_str("        self.encode(version, &mut buf)?;\n");
        out.push_str("        Ok(buf)\n    }\n\n");

        out.push_str("    /// Zero-copy, single-pass encode (the enum-level analogue of\n");
        out.push_str("    /// `EncodableZeroCopy::to_segments`): large `Bytes` payloads become\n");
        out.push_str("    /// refcounted segments instead of being copied.\n");
        out.push_str("    pub fn to_segments(&self, version: i16) -> Result<SegmentedBuf, EncodeError> {\n");
        out.push_str("        let mut buf = SegmentedBuf::new();\n");
        out.push_str("        self.encode(version, &mut buf)?;\n");
        out.push_str("        Ok(buf)\n    }\n");
        out.push_str("}\n\n");

        for (_, _, module, name, _, _, _) in &rows {
            out.push_str(&format!(
                "impl From<super::{}::{}> for {} {{\n    fn from(m: super::{}::{}) -> Self {{\n        Self::{}(m)\n    }}\n}}\n\n",
                module,
                name,
                kind,
                module,
                name,
                variant(name)
            ));
        }
    }

    out
}

/// Emit `api_constants.rs`: named constants for every Kafka API key, plus a
/// compact enum for typed matches and conversions.
fn generate_api_constants(entries: &BTreeMap<i16, String>) -> String {
    let mut out = String::new();
    out.push_str("// Generated API-key constants — do not edit.\n\n");
    out.push_str("#[repr(i16)]\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\n");
    out.push_str("pub enum ApiKey {\n");
    for (api_key, name) in entries {
        out.push_str(&format!("    {} = {},\n", name, api_key));
    }
    out.push_str("}\n\n");

    out.push_str("impl ApiKey {\n");
    for (api_key, name) in entries {
        out.push_str(&format!(
            "    pub const {}: ApiKey = ApiKey::{};\n",
            const_name(name),
            name
        ));
        out.push_str(&format!(
            "    pub const {}_KEY: i16 = {};\n",
            const_name(name),
            api_key
        ));
    }
    out.push('\n');
    out.push_str("    pub const fn as_i16(self) -> i16 {\n");
    out.push_str("        self as i16\n");
    out.push_str("    }\n\n");
    out.push_str("    pub const fn from_i16(api_key: i16) -> Option<ApiKey> {\n");
    out.push_str("        match api_key {\n");
    for (api_key, name) in entries {
        out.push_str(&format!(
            "            {} => Some(ApiKey::{}),\n",
            api_key,
            name
        ));
    }
    out.push_str("            _ => None,\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}

/// Emit `api_version >= flex_min` (or a constant when the message is never /
/// always flexible) for the header-version derivation functions.
fn flex_expr(flex_min: i16) -> String {
    if flex_min == 0 {
        "true".to_string()
    } else if flex_min == i16::MAX {
        "false".to_string()
    } else {
        format!("api_version >= {}", flex_min)
    }
}

fn const_name(name: &str) -> String {
    name.to_shouty_snake_case()
}

// ── GitHub API helpers ────────────────────────────────────────────────────────

/// Read all `*.json` schema files from a local directory as (filename, content).
fn load_schemas_from_dir(dir: &str) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading schema dir {}", dir))? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let content = fs::read_to_string(&path)?;
            out.push((name, content));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Parse a dotted version tag like "4.3.1" into comparable numeric components.
fn version_key(s: &str) -> Vec<u32> {
    s.split('.')
        .map(|p| p.parse::<u32>().unwrap_or(0))
        .collect()
}

#[derive(Deserialize)]
struct GitTag {
    name: String,
}

fn fetch_tags(client: &Client) -> Result<Vec<GitTag>> {
    let mut all_tags: Vec<GitTag> = Vec::new();
    let mut page = 1u32;

    loop {
        let url = format!(
            "{}/repos/{}/tags?per_page=100&page={}",
            GITHUB_API, KAFKA_REPO, page
        );
        let resp: Vec<GitTag> = client.get(&url).send()?.json()?;
        if resp.is_empty() {
            break;
        }
        all_tags.extend(resp);
        page += 1;
        if page > 5 {
            break; // cap at 500 tags
        }
    }

    // Keep only version tags (e.g. 4.3.1, 3.8.0) sorted newest first
    all_tags.retain(|t| {
        let n = &t.name;
        n.chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    });

    Ok(all_tags)
}

#[derive(Deserialize)]
struct GithubTreeItem {
    path: String,
    #[serde(rename = "type")]
    item_type: String,
}

#[derive(Deserialize)]
struct GithubTree {
    tree: Vec<GithubTreeItem>,
    truncated: bool,
}

fn fetch_schema_file_list(client: &Client, tag: &str) -> Result<Vec<String>> {
    // Use the tree API with recursive=1 to list files efficiently
    let url = format!(
        "{}/repos/{}/git/trees/{}?recursive=1",
        GITHUB_API, KAFKA_REPO, tag
    );
    let resp = client.get(&url).send()?;
    if resp.status() == reqwest::StatusCode::FORBIDDEN
        || resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS
    {
        anyhow::bail!(
            "GitHub API rate limit hit (status {}). Set GITHUB_TOKEN to raise the \
             limit to 5000/hr, or wait for the limit to reset.",
            resp.status()
        );
    }
    let tree: GithubTree = resp
        .json()
        .context("Failed to parse GitHub tree API response")?;

    if tree.truncated {
        eprintln!("Warning: GitHub tree response was truncated");
    }

    let prefix = format!("{}/", SCHEMA_PATH);
    let files: Vec<String> = tree
        .tree
        .into_iter()
        .filter(|item| {
            item.item_type == "blob"
                && item.path.starts_with(&prefix)
                && item.path.ends_with(".json")
        })
        .map(|item| item.path[prefix.len()..].to_string())
        .collect();

    Ok(files)
}
