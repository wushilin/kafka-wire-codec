use crate::schema::*;
use heck::ToSnakeCase;

/// Convert a Kafka field name to a valid Rust field identifier, escaping
/// reserved keywords as raw identifiers (e.g. `type` -> `r#type`, `match` -> `r#match`).
fn field_ident(name: &str) -> String {
    let snake = name.to_snake_case();
    // Keywords that cannot be raw identifiers get a trailing underscore instead.
    const NON_RAW: &[&str] = &["crate", "self", "super", "Self"];
    // Strict + reserved keywords that are valid as raw identifiers.
    const RAW: &[&str] = &[
        "as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "static", "struct", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
        "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
        "typeof", "unsized", "virtual", "yield", "try", "union",
    ];
    if NON_RAW.contains(&snake.as_str()) {
        format!("{}_", snake)
    } else if RAW.contains(&snake.as_str()) {
        format!("r#{}", snake)
    } else {
        snake
    }
}

/// Emit `if version >= flex_min { compact } else { legacy }`, constant-folded
/// when the message is never (flex_min = i16::MAX) or always (flex_min = 0)
/// flexible — avoiding dead `version >= 32767` guards in the generated code.
fn flex_cond(flex_min: i16, compact: &str, legacy: &str) -> String {
    if flex_min <= 0 {
        compact.to_string()
    } else if flex_min == i16::MAX {
        legacy.to_string()
    } else {
        format!(
            "if version >= {} {{ {} }} else {{ {} }}",
            flex_min, compact, legacy
        )
    }
}

/// Guard a statement on flexibility: `if version >= flex_min { stmt }`,
/// constant-folded to `stmt` (always flexible) or nothing (never flexible).
fn flex_stmt(flex_min: i16, stmt: &str) -> String {
    if flex_min <= 0 {
        stmt.to_string()
    } else if flex_min == i16::MAX {
        String::new()
    } else {
        format!("if version >= {} {{ {} }}", flex_min, stmt)
    }
}

/// Extra codegen inputs for the top-level message struct (vs nested structs).
struct TopLevel {
    api_key: i16,
    valid_min: i16,
    valid_max: i16,
    valid_max_str: String,
}

pub fn generate_message(spec: &MessageSpec) -> String {
    let mut out = String::new();
    let (valid_min, valid_max) = parse_versions(&spec.valid_versions);
    let (flex_min, _) = parse_versions(&spec.flexible_versions);

    // Collect all struct definitions (common + inline field structs)
    let mut structs: Vec<StructSpec> = spec.common_structs.clone();
    collect_inline_structs(&spec.fields, &mut structs);

    // Never-flexible messages fold their version guards away entirely, which
    // can leave `version` unused; bounded field ranges use the natural
    // `version >= a && version <= b` form.
    out.push_str("#![allow(unused_variables, clippy::manual_range_contains)]\n\n");
    out.push_str("use bytes::Bytes;\n");
    out.push_str("use crate::codec::*;\n");
    out.push_str("use crate::error::DecodeError;\n\n");

    // Emit common/inline structs first
    for s in &structs {
        out.push_str(&emit_struct(&s.name, None, &s.fields, flex_min, None));
        out.push('\n');
    }

    let valid_max_str = if valid_max == i16::MAX {
        "i16::MAX".to_string()
    } else {
        valid_max.to_string()
    };
    let doc = if valid_min > valid_max {
        "Retired API (validVersions: \"none\"): kept for the api-key namespace; no version can be encoded or decoded.".to_string()
    } else {
        format!(
            "Valid versions: {}-{}.",
            valid_min,
            if valid_max == i16::MAX {
                "latest".to_string()
            } else {
                valid_max.to_string()
            }
        )
    };
    out.push_str(&emit_struct(
        &spec.name,
        Some(&doc),
        &spec.fields,
        flex_min,
        Some(&TopLevel {
            api_key: spec.api_key.unwrap_or(-1),
            valid_min,
            valid_max,
            valid_max_str: valid_max_str.clone(),
        }),
    ));

    // Encodable impl — forwards to the inherent methods, enabling generic
    // size-first framing (see the runtime crate's frame module).
    out.push_str(&format!("\nimpl crate::Encodable for {} {{\n", spec.name));
    out.push_str(&format!(
        "    const API_KEY: i16 = {};\n",
        spec.api_key.unwrap_or(-1)
    ));
    out.push_str(&format!(
        "    const VALID_MIN_VERSION: i16 = {};\n",
        valid_min
    ));
    out.push_str(&format!(
        "    const VALID_MAX_VERSION: i16 = {};\n",
        valid_max_str
    ));
    out.push_str(&format!(
        "    const FLEXIBLE_MIN_VERSION: i16 = {};\n",
        if flex_min == i16::MAX {
            "i16::MAX".to_string()
        } else {
            flex_min.to_string()
        }
    ));
    out.push_str("    fn wire_size(&self, version: i16) -> usize { self.encoded_size(version) }\n");
    out.push_str(
        "    fn write(&self, version: i16, buf: &mut bytes::BytesMut) { self.encode(version, buf) }\n",
    );
    out.push_str("    fn read(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> { Self::decode(version, buf) }\n");
    out.push_str("}\n");

    // Zero-copy encode entry point (separate trait keeps Encodable non-generic).
    out.push_str(&format!(
        "\nimpl crate::EncodableZeroCopy for {} {{\n",
        spec.name
    ));
    out.push_str(
        "    fn write_segmented(&self, version: i16, buf: &mut SegmentedBuf) { self.encode(version, buf) }\n",
    );
    out.push_str("}\n");

    out
}

/// Emit one struct: declaration, Default impl (with schema defaults), and the
/// encoded_size/encode/decode impl. `top` is Some for the top-level message,
/// which additionally gets API constants and version validation.
fn emit_struct(
    name: &str,
    doc: Option<&str>,
    fields: &[FieldSpec],
    flex_min: i16,
    top: Option<&TopLevel>,
) -> String {
    let mut out = String::new();

    // Emit a manual Default impl only when a schema default differs from the
    // derived zero-value default (Kafka semantics: e.g. FetchRequest
    // replica_id = -1, max_bytes = 0x7fffffff); derive it otherwise.
    let defaults: Vec<(String, String)> = fields
        .iter()
        .filter(|f| f.tag.is_none())
        .map(|f| (field_ident(&f.name), default_value_expr(f)))
        .collect();
    let all_trivial = defaults.iter().all(|(_, expr)| {
        matches!(
            expr.as_str(),
            "0" | "0.0" | "false" | "None" | "Bytes::new()" | "Vec::new()" | "[0u8; 16]"
        ) || expr.ends_with("::default()")
    });

    if let Some(doc) = doc {
        out.push_str(&format!("/// {}\n", doc));
    }
    if all_trivial {
        out.push_str("#[derive(Debug, Clone, Default)]\n");
    } else {
        out.push_str("#[derive(Debug, Clone)]\n");
    }
    out.push_str(&format!("pub struct {} {{\n", name));
    for field in fields {
        out.push_str(&generate_field_decl(field));
    }
    out.push_str("    /// Raw tagged fields (flexible versions), in ascending tag order.\n");
    out.push_str("    pub tagged_fields: Vec<(u32, Bytes)>,\n");
    out.push_str("}\n\n");

    if !all_trivial {
        out.push_str(&format!("impl Default for {} {{\n", name));
        out.push_str("    fn default() -> Self {\n");
        out.push_str("        Self {\n");
        for (fname, expr) in &defaults {
            out.push_str(&format!("            {}: {},\n", fname, expr));
        }
        out.push_str("            tagged_fields: Vec::new(),\n");
        out.push_str("        }\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }

    out.push_str(&format!("impl {} {{\n", name));

    let version_check_size_encode = if let Some(t) = top {
        out.push_str(&format!("    pub const API_KEY: i16 = {};\n", t.api_key));
        out.push_str(&format!(
            "    pub const VALID_MIN_VERSION: i16 = {};\n",
            t.valid_min
        ));
        out.push_str(&format!(
            "    pub const VALID_MAX_VERSION: i16 = {};\n",
            t.valid_max_str
        ));
        out.push_str(&format!(
            "    /// First flexible (tagged-fields) version; `i16::MAX` if never flexible.\n    pub const FLEXIBLE_MIN_VERSION: i16 = {};\n\n",
            if flex_min == i16::MAX {
                "i16::MAX".to_string()
            } else {
                flex_min.to_string()
            }
        ));
        if t.valid_min > t.valid_max {
            // Tombstone API (validVersions: "none", e.g. ControlledShutdown in
            // 4.x): no version can be encoded or decoded. Emit explicit stubs.
            out.push_str("    pub fn encoded_size(&self, version: i16) -> usize {\n");
            out.push_str("        panic!(\"api key {} supports no protocol versions (requested {})\", Self::API_KEY, version);\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn encode<B: WireBuf>(&self, version: i16, _buf: &mut B) {\n");
            out.push_str("        panic!(\"api key {} supports no protocol versions (requested {})\", Self::API_KEY, version);\n");
            out.push_str("    }\n\n");
            out.push_str("    pub fn decode(version: i16, _buf: &mut Bytes) -> Result<Self, DecodeError> {\n");
            out.push_str("        Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version })\n");
            out.push_str("    }\n");
            out.push_str("}\n");
            return out;
        }
        "        assert!((Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version),\n            \"unsupported version {} for api key {}\", version, Self::API_KEY);\n"
    } else {
        ""
    };

    // encoded_size
    out.push_str("    pub fn encoded_size(&self, version: i16) -> usize {\n");
    out.push_str(version_check_size_encode);
    out.push_str("        let mut size = 0usize;\n");
    for field in fields {
        out.push_str(&generate_field_size(field, flex_min));
    }
    if flex_min != i16::MAX {
        out.push_str(&format!(
            "        {}\n",
            flex_stmt(flex_min, "size += tagged_fields_size(&self.tagged_fields);")
        ));
    }
    out.push_str("        size\n");
    out.push_str("    }\n\n");

    // encode — generic over the sink so the same code drives both the
    // contiguous (BytesMut) and zero-copy (SegmentedBuf) paths.
    out.push_str("    pub fn encode<B: WireBuf>(&self, version: i16, buf: &mut B) {\n");
    out.push_str(version_check_size_encode);
    for field in fields {
        out.push_str(&generate_field_encode(field, flex_min));
    }
    if flex_min != i16::MAX {
        out.push_str(&format!(
            "        {}\n",
            flex_stmt(flex_min, "put_tagged_fields(buf, &self.tagged_fields);")
        ));
    }
    out.push_str("    }\n\n");

    // decode
    out.push_str("    pub fn decode(version: i16, buf: &mut Bytes) -> Result<Self, DecodeError> {\n");
    if top.is_some() {
        out.push_str(
            "        if !(Self::VALID_MIN_VERSION..=Self::VALID_MAX_VERSION).contains(&version) {\n",
        );
        out.push_str("            return Err(DecodeError::UnsupportedVersion { api_key: Self::API_KEY, version });\n");
        out.push_str("        }\n");
    }
    out.push_str(&format!("        let mut msg = {}::default();\n", name));
    for field in fields {
        out.push_str(&generate_field_decode(field, flex_min));
    }
    if flex_min != i16::MAX {
        out.push_str(&format!(
            "        {}\n",
            flex_stmt(flex_min, "msg.tagged_fields = get_tagged_fields(buf)?;")
        ));
    }
    out.push_str("        Ok(msg)\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}

fn generate_field_decl(field: &FieldSpec) -> String {
    // Tagged fields are preserved via the raw tagged-fields section, not inline.
    if field.tag.is_some() {
        return String::new();
    }
    let (rust_type, optional) = field_rust_type(field);
    let fname = field_ident(&field.name);
    let mut out = String::new();
    if !field.about.is_empty() {
        out.push_str(&format!(
            "    /// {}\n",
            field.about.replace('\n', " ").trim()
        ));
    }
    if optional {
        out.push_str(&format!("    pub {}: Option<{}>,\n", fname, rust_type));
    } else {
        out.push_str(&format!("    pub {}: {},\n", fname, rust_type));
    }
    out
}

/// The guard expression for "null is legal at this version", or None when the
/// field is not nullable at any version.
fn nullable_guard(field: &FieldSpec) -> Option<String> {
    if field.nullable_versions.is_empty() {
        return None;
    }
    let (nmin, nmax) = parse_versions(&field.nullable_versions);
    Some(version_guard_expr(nmin, nmax))
}

fn generate_field_size(field: &FieldSpec, flex_min: i16) -> String {
    if field.tag.is_some() {
        return String::new();
    }
    let fname = field_ident(&field.name);
    let (_, ver_min, ver_max) = field_version_guard(field);
    let is_array = field.field_type.starts_with("[]");
    let nguard = nullable_guard(field);
    let inner_type = if is_array {
        field.field_type[2..].trim().to_string()
    } else {
        field.field_type.clone()
    };

    let mut lines = String::new();
    let guard = version_guard_expr(ver_min, ver_max);
    lines.push_str(&guard_open(&guard));

    if is_array {
        let elem_size = element_size_expr(&inner_type, "item", flex_min);
        // Length prefix + each element, computed over a bound `arr` slice.
        let mut body = String::new();
        body.push_str(&format!(
            "                {}\n",
            flex_cond(flex_min, "size += uvarint_size(arr.len() as u64 + 1);", "size += 4;")
        ));
        if let Ok(n) = elem_size.parse::<usize>() {
            // Fixed-size elements: multiply rather than loop (avoids unused `item`).
            if n == 1 {
                body.push_str("                size += arr.len();\n");
            } else {
                body.push_str(&format!("                size += arr.len() * {};\n", n));
            }
        } else {
            body.push_str("                for item in arr {\n");
            body.push_str(&format!("                    size += {};\n", elem_size));
            body.push_str("                }\n");
        }
        if let Some(ng) = &nguard {
            lines.push_str(&format!("            match &self.{} {{\n", fname));
            lines.push_str("                Some(arr) => {\n");
            lines.push_str(&body);
            lines.push_str("                }\n");
            lines.push_str("                None => {\n");
            if ng != "true" {
                lines.push_str(&format!(
                    "                    assert!({ng}, \"field {fname} is None but not nullable at version {{}}\", version);\n"
                ));
            }
            lines.push_str(&format!(
                "                    {}\n",
                flex_cond(flex_min, "size += 1;", "size += 4;")
            ));
            lines.push_str("                }\n");
            lines.push_str("            }\n");
        } else {
            lines.push_str(&format!("            {{ let arr = &self.{};\n", fname));
            lines.push_str(&body);
            lines.push_str("            }\n");
        }
    } else {
        lines.push_str(&format!(
            "            size += {};\n",
            scalar_size_expr(&inner_type, &fname, nguard.as_deref(), flex_min)
        ));
    }

    lines.push_str("        }\n");
    lines
}

fn generate_field_encode(field: &FieldSpec, flex_min: i16) -> String {
    if field.tag.is_some() {
        return String::new();
    }
    let fname = field_ident(&field.name);
    let (_, ver_min, ver_max) = field_version_guard(field);
    let is_array = field.field_type.starts_with("[]");
    let nguard = nullable_guard(field);
    let inner_type = if is_array {
        field.field_type[2..].trim().to_string()
    } else {
        field.field_type.clone()
    };

    let mut lines = String::new();
    let guard = version_guard_expr(ver_min, ver_max);
    lines.push_str(&guard_open(&guard));

    if is_array {
        let elem_enc = element_encode_expr(&inner_type, "item", flex_min);
        let mut body = String::new();
        body.push_str(&format!(
            "                {}\n",
            flex_cond(flex_min, "put_uvarint(buf, arr.len() as u64 + 1);", "put_i32(buf, arr.len() as i32);")
        ));
        body.push_str(&format!(
            "                for item in arr {{ {} }}\n",
            elem_enc
        ));
        if let Some(ng) = &nguard {
            lines.push_str(&format!("            match &self.{} {{\n", fname));
            lines.push_str("                Some(arr) => {\n");
            lines.push_str(&body);
            lines.push_str("                }\n");
            lines.push_str("                None => {\n");
            if ng != "true" {
                lines.push_str(&format!(
                    "                    assert!({ng}, \"field {fname} is None but not nullable at version {{}}\", version);\n"
                ));
            }
            lines.push_str(&format!(
                "                    {}\n",
                flex_cond(flex_min, "put_uvarint(buf, 0);", "put_i32(buf, -1);")
            ));
            lines.push_str("                }\n");
            lines.push_str("            }\n");
        } else {
            lines.push_str(&format!("            {{ let arr = &self.{};\n", fname));
            lines.push_str(&body);
            lines.push_str("            }\n");
        }
    } else {
        lines.push_str(&format!(
            "            {};\n",
            scalar_encode_expr(&inner_type, &fname, nguard.as_deref(), flex_min)
        ));
    }

    lines.push_str("        }\n");
    lines
}

fn generate_field_decode(field: &FieldSpec, flex_min: i16) -> String {
    if field.tag.is_some() {
        return String::new();
    }
    let fname = field_ident(&field.name);
    let (_, ver_min, ver_max) = field_version_guard(field);
    let is_array = field.field_type.starts_with("[]");
    let nguard = nullable_guard(field);
    let inner_type = if is_array {
        field.field_type[2..].trim().to_string()
    } else {
        field.field_type.clone()
    };

    let mut lines = String::new();
    let guard = version_guard_expr(ver_min, ver_max);
    lines.push_str(&guard_open(&guard));

    if is_array {
        let elem_dec = element_decode_expr(&inner_type, flex_min);
        // Read the length prefix as an Option: None = null (-1 / compact 0).
        lines.push_str(&format!(
            "            let len_opt = {};\n",
            flex_cond(
                flex_min,
                "{ let n = get_uvarint32(buf)?; if n == 0 { None } else { Some((n - 1) as usize) } }",
                "{ let n = get_i32(buf)?; if n < 0 { None } else { Some(n as usize) } }"
            )
        ));
        // Decode loop: capacity is clamped by remaining bytes so a malicious
        // length prefix cannot force a huge upfront allocation.
        let loop_body = format!(
            "let mut items = Vec::with_capacity(count.min(buf.len()));\n                for _ in 0..count {{ items.push({}?); }}\n",
            elem_dec
        );
        if let Some(ng) = &nguard {
            lines.push_str(&format!("            msg.{} = match len_opt {{\n", fname));
            lines.push_str("                Some(count) => {\n");
            lines.push_str(&format!("                {}", loop_body));
            lines.push_str("                Some(items)\n");
            lines.push_str("                }\n");
            if ng == "true" {
                lines.push_str("                None => None,\n");
            } else {
                lines.push_str(&format!(
                    "                None => {{ if {ng} {{ None }} else {{ return Err(DecodeError::NullForNonNullable); }} }}\n"
                ));
            }
            lines.push_str("            };\n");
        } else {
            lines.push_str(
                "            let count = len_opt.ok_or(DecodeError::NullForNonNullable)?;\n",
            );
            lines.push_str(&format!("            {{ {}", loop_body));
            lines.push_str(&format!("            msg.{} = items; }}\n", fname));
        }
    } else {
        lines.push_str(&format!(
            "            msg.{} = {};\n",
            fname,
            scalar_decode_expr(&inner_type, nguard.as_deref(), flex_min)
        ));
    }

    lines.push_str("        }\n");
    lines
}

// ── Type mapping helpers ──────────────────────────────────────────────────────

/// Returns (rust_type, is_optional). Optionality comes ONLY from nullability —
/// version gating is handled at runtime, with absent fields retaining their
/// `Default` value (matching Kafka's own codegen).
fn field_rust_type(field: &FieldSpec) -> (String, bool) {
    let is_array = field.field_type.starts_with("[]");
    let is_nullable = !field.nullable_versions.is_empty();

    if is_array {
        // Nullable arrays are Option<Vec<T>> so null is distinct from empty.
        let elem = field.field_type[2..].trim();
        (format!("Vec<{}>", primitive_rust_type(elem)), is_nullable)
    } else {
        // Any nullable non-array field is Option<T>: string/bytes/records use the
        // null length encoding; nullable structs use a presence byte (-1 = null).
        (primitive_rust_type(&field.field_type), is_nullable)
    }
}

fn primitive_rust_type(t: &str) -> String {
    match t {
        "int8" => "i8",
        "int16" => "i16",
        "int32" => "i32",
        "int64" => "i64",
        "uint16" => "u16",
        "uint32" => "u32",
        "float64" => "f64",
        "bool" => "bool",
        "string" => "Bytes",
        "bytes" | "records" => "Bytes",
        "uuid" => "[u8; 16]",
        other => other, // struct type
    }
    .to_string()
}

/// The Default-impl initializer for a field, honoring the schema `default`
/// (Kafka semantics: absent fields decode to this value, and default-built
/// messages encode it).
fn default_value_expr(field: &FieldSpec) -> String {
    let is_array = field.field_type.starts_with("[]");
    let is_nullable = !field.nullable_versions.is_empty();
    let t = if is_array {
        field.field_type[2..].trim()
    } else {
        field.field_type.as_str()
    };

    // Normalize the schema default into a string form ("null" means wire null).
    let raw: Option<String> = field.default.as_ref().and_then(|v| match v {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(b) => Some(b.to_string()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::String(s) => Some(s.trim().to_string()),
        _ => None,
    });
    let is_null_default = raw.as_deref() == Some("null");

    if is_array {
        return match (is_nullable, is_null_default) {
            (true, true) => "None".to_string(),
            (true, false) => "Some(Vec::new())".to_string(),
            _ => "Vec::new()".to_string(),
        };
    }

    match t {
        "int8" | "int16" | "int32" | "int64" | "uint16" | "uint32" => raw
            .filter(|s| !s.is_empty())
            .map(|s| int_literal(&s))
            .unwrap_or_else(|| "0".to_string()),
        "float64" => raw
            .map(|s| {
                if s.contains('.') || s.contains('e') {
                    format!("{}f64", s)
                } else {
                    format!("{}.0f64", s)
                }
            })
            .unwrap_or_else(|| "0.0".to_string()),
        "bool" => raw.unwrap_or_else(|| "false".to_string()),
        "string" => {
            let inner = match raw {
                Some(ref s) if !is_null_default && !s.is_empty() => {
                    format!("Bytes::from_static(b\"{}\")", escape_bytes(s))
                }
                _ => "Bytes::new()".to_string(),
            };
            if is_nullable {
                if is_null_default {
                    "None".to_string()
                } else {
                    // Kafka's default for a nullable string without an explicit
                    // "null" default is the empty string, not null.
                    format!("Some({})", inner)
                }
            } else {
                inner
            }
        }
        "bytes" | "records" => {
            if is_nullable {
                if is_null_default {
                    "None".to_string()
                } else {
                    "Some(Bytes::new())".to_string()
                }
            } else {
                "Bytes::new()".to_string()
            }
        }
        "uuid" => "[0u8; 16]".to_string(),
        // Nullable structs default to null (matching Kafka's generated code).
        other => {
            if is_nullable {
                "None".to_string()
            } else {
                format!("{}::default()", other)
            }
        }
    }
}

/// Render a schema integer default ("-1", "0x7fffffff", "9223372036854775807")
/// as a Rust literal.
fn int_literal(s: &str) -> String {
    let s = s.trim();
    // Rust accepts 0x literals and negative decimals directly; validate so a
    // malformed schema default fails codegen instead of emitting garbage.
    let ok = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i128::from_str_radix(hex, 16).is_ok()
    } else {
        s.parse::<i128>().is_ok()
    };
    assert!(ok, "unparseable integer default in schema: {:?}", s);
    s.to_string()
}

fn escape_bytes(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            c if c.is_ascii() && !c.is_ascii_control() => vec![c],
            c => format!("\\u{{{:x}}}", c as u32).chars().collect(),
        })
        .collect()
}

/// `nguard`: Some(expr) = the version-range guard under which null is legal;
/// None = the field is never nullable. Outside the nullable range, a `None`
/// value is a caller bug (panic on encode) and a wire null is a protocol
/// error (DecodeError::NullForNonNullable on decode) — matching Kafka.
fn scalar_size_expr(t: &str, fname: &str, nguard: Option<&str>, flex_min: i16) -> String {
    match t {
        "int8" | "bool" => "1".to_string(),
        "int16" | "uint16" => "2".to_string(),
        "int32" | "uint32" => "4".to_string(),
        "int64" | "float64" => "8".to_string(),
        "uuid" => "16".to_string(),
        "string" => {
            let nullable = flex_cond(
                flex_min,
                &format!("compact_nullable_string_size(self.{fname}.as_deref())"),
                &format!("nullable_string_size(self.{fname}.as_deref())"),
            );
            match nguard {
                Some("true") => nullable,
                Some(ng) => format!(
                    "if {ng} {{ {nullable} }} else {{ let v = self.{fname}.as_deref().expect(\"field {fname} is None but not nullable at this version\"); {} }}",
                    flex_cond(flex_min, "compact_string_size(v)", "string_size(v)")
                ),
                None => flex_cond(
                    flex_min,
                    &format!("compact_string_size(&self.{fname})"),
                    &format!("string_size(&self.{fname})"),
                ),
            }
        }
        "bytes" | "records" => {
            let nullable = flex_cond(
                flex_min,
                &format!("compact_nullable_bytes_size(self.{fname}.as_deref())"),
                &format!("nullable_bytes_size(self.{fname}.as_deref())"),
            );
            match nguard {
                Some("true") => nullable,
                Some(ng) => format!(
                    "if {ng} {{ {nullable} }} else {{ let v = self.{fname}.as_deref().expect(\"field {fname} is None but not nullable at this version\"); {} }}",
                    flex_cond(flex_min, "compact_bytes_size(v)", "bytes_size(v)")
                ),
                None => flex_cond(
                    flex_min,
                    &format!("compact_bytes_size(&self.{fname})"),
                    &format!("bytes_size(&self.{fname})"),
                ),
            }
        }
        // Struct field. Nullable structs are prefixed with a presence byte.
        _ => match nguard {
            Some("true") => format!("1 + self.{fname}.as_ref().map_or(0, |v| v.encoded_size(version))"),
            Some(ng) => format!(
                "if {ng} {{ 1 + self.{fname}.as_ref().map_or(0, |v| v.encoded_size(version)) }} else {{ self.{fname}.as_ref().expect(\"field {fname} is None but not nullable at this version\").encoded_size(version) }}"
            ),
            None => format!("self.{}.encoded_size(version)", fname),
        },
    }
}

fn scalar_encode_expr(t: &str, fname: &str, nguard: Option<&str>, flex_min: i16) -> String {
    match t {
        "int8" => format!("put_i8(buf, self.{})", fname),
        "int16" => format!("put_i16(buf, self.{})", fname),
        "int32" => format!("put_i32(buf, self.{})", fname),
        "int64" => format!("put_i64(buf, self.{})", fname),
        "uint16" => format!("put_u16(buf, self.{})", fname),
        "uint32" => format!("put_u32(buf, self.{})", fname),
        "float64" => format!("put_f64(buf, self.{})", fname),
        "bool" => format!("put_bool(buf, self.{})", fname),
        "uuid" => format!("put_uuid(buf, &self.{})", fname),
        "string" => {
            let nullable = flex_cond(
                flex_min,
                &format!("put_compact_nullable_string(buf, self.{fname}.as_deref())"),
                &format!("put_nullable_string(buf, self.{fname}.as_deref())"),
            );
            match nguard {
                Some("true") => nullable,
                Some(ng) => format!(
                    "if {ng} {{ {nullable} }} else {{ let v = self.{fname}.as_deref().expect(\"field {fname} is None but not nullable at this version\"); {} }}",
                    flex_cond(flex_min, "put_compact_string(buf, v)", "put_string(buf, v)")
                ),
                None => flex_cond(
                    flex_min,
                    &format!("put_compact_string(buf, &self.{fname})"),
                    &format!("put_string(buf, &self.{fname})"),
                ),
            }
        }
        // _zc variants pass the Bytes handle through so a zero-copy sink can
        // retain large payloads (e.g. record batches) without copying.
        "bytes" | "records" => {
            let nullable = flex_cond(
                flex_min,
                &format!("put_compact_nullable_bytes_zc(buf, self.{fname}.as_ref())"),
                &format!("put_nullable_bytes_zc(buf, self.{fname}.as_ref())"),
            );
            match nguard {
                Some("true") => nullable,
                Some(ng) => format!(
                    "if {ng} {{ {nullable} }} else {{ let v = self.{fname}.as_ref().expect(\"field {fname} is None but not nullable at this version\"); {} }}",
                    flex_cond(flex_min, "put_compact_bytes_zc(buf, v)", "put_bytes_zc(buf, v)")
                ),
                None => flex_cond(
                    flex_min,
                    &format!("put_compact_bytes_zc(buf, &self.{fname})"),
                    &format!("put_bytes_zc(buf, &self.{fname})"),
                ),
            }
        }
        // Nullable struct: Kafka writes -1 for null, and any byte >= 0 means
        // present (its readers use `readByte() < 0 => null`).
        _ => match nguard {
            Some("true") => format!(
                "match &self.{fname} {{ Some(v) => {{ put_i8(buf, 1); v.encode(version, buf); }}, None => put_i8(buf, -1) }}"
            ),
            Some(ng) => format!(
                "if {ng} {{ match &self.{fname} {{ Some(v) => {{ put_i8(buf, 1); v.encode(version, buf); }}, None => put_i8(buf, -1) }} }} else {{ self.{fname}.as_ref().expect(\"field {fname} is None but not nullable at this version\").encode(version, buf) }}"
            ),
            None => format!("self.{}.encode(version, buf)", fname),
        },
    }
}

fn scalar_decode_expr(t: &str, nguard: Option<&str>, flex_min: i16) -> String {
    match t {
        "int8" => "get_i8(buf)?".to_string(),
        "int16" => "get_i16(buf)?".to_string(),
        "int32" => "get_i32(buf)?".to_string(),
        "int64" => "get_i64(buf)?".to_string(),
        "uint16" => "get_u16(buf)?".to_string(),
        "uint32" => "get_u32(buf)?".to_string(),
        "float64" => "get_f64(buf)?".to_string(),
        "bool" => "get_bool(buf)?".to_string(),
        "uuid" => "get_uuid(buf)?".to_string(),
        "string" => {
            let read = flex_cond(flex_min, "get_compact_string(buf)?", "get_string(buf)?");
            match nguard {
                Some("true") => read,
                Some(ng) => format!(
                    "{{ let v = {read}; if {ng} {{ v }} else {{ Some(v.ok_or(DecodeError::NullForNonNullable)?) }} }}"
                ),
                None => format!("({read}).ok_or(DecodeError::NullForNonNullable)?"),
            }
        }
        "bytes" | "records" => {
            let read = flex_cond(flex_min, "get_compact_bytes(buf)?", "get_bytes(buf)?");
            match nguard {
                Some("true") => read,
                Some(ng) => format!(
                    "{{ let v = {read}; if {ng} {{ v }} else {{ Some(v.ok_or(DecodeError::NullForNonNullable)?) }} }}"
                ),
                None => format!("({read}).ok_or(DecodeError::NullForNonNullable)?"),
            }
        }
        other => match nguard {
            // Kafka's presence byte: < 0 means null, >= 0 means present.
            Some("true") => format!(
                "if get_i8(buf)? < 0 {{ None }} else {{ Some({other}::decode(version, buf)?) }}"
            ),
            Some(ng) => format!(
                "if {ng} {{ if get_i8(buf)? < 0 {{ None }} else {{ Some({other}::decode(version, buf)?) }} }} else {{ Some({other}::decode(version, buf)?) }}"
            ),
            None => format!("{}::decode(version, buf)?", other),
        },
    }
}

fn element_size_expr(t: &str, item: &str, flex_min: i16) -> String {
    match t {
        "int8" | "bool" => "1".to_string(),
        "int16" | "uint16" => "2".to_string(),
        "int32" | "uint32" => "4".to_string(),
        "int64" | "float64" => "8".to_string(),
        "uuid" => "16".to_string(),
        "string" => flex_cond(
            flex_min,
            &format!("compact_string_size({item})"),
            &format!("string_size({item})"),
        ),
        "bytes" | "records" => flex_cond(
            flex_min,
            &format!("compact_bytes_size({item})"),
            &format!("bytes_size({item})"),
        ),
        _ => format!("{}.encoded_size(version)", item),
    }
}

fn element_encode_expr(t: &str, item: &str, flex_min: i16) -> String {
    match t {
        "int8" => format!("put_i8(buf, *{});", item),
        "int16" => format!("put_i16(buf, *{});", item),
        "int32" => format!("put_i32(buf, *{});", item),
        "int64" => format!("put_i64(buf, *{});", item),
        "uint16" => format!("put_u16(buf, *{});", item),
        "uint32" => format!("put_u32(buf, *{});", item),
        "float64" => format!("put_f64(buf, *{});", item),
        "bool" => format!("put_bool(buf, *{});", item),
        "uuid" => format!("put_uuid(buf, {});", item),
        "string" => flex_cond(
            flex_min,
            &format!("put_compact_string(buf, {item});"),
            &format!("put_string(buf, {item});"),
        ),
        "bytes" | "records" => flex_cond(
            flex_min,
            &format!("put_compact_bytes_zc(buf, {item});"),
            &format!("put_bytes_zc(buf, {item});"),
        ),
        _ => format!("{}.encode(version, buf);", item),
    }
}

fn element_decode_expr(t: &str, flex_min: i16) -> String {
    match t {
        "int8" => "get_i8(buf)".to_string(),
        "int16" => "get_i16(buf)".to_string(),
        "int32" => "get_i32(buf)".to_string(),
        "int64" => "get_i64(buf)".to_string(),
        "uint16" => "get_u16(buf)".to_string(),
        "uint32" => "get_u32(buf)".to_string(),
        "float64" => "get_f64(buf)".to_string(),
        "bool" => "get_bool(buf)".to_string(),
        "uuid" => "get_uuid(buf)".to_string(),
        // Array elements are never nullable in Kafka schemas: a wire null here
        // is a protocol violation, not an empty value.
        "string" => format!(
            "({}).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))",
            flex_cond(flex_min, "get_compact_string(buf)", "get_string(buf)")
        ),
        "bytes" | "records" => format!(
            "({}).and_then(|o| o.ok_or(DecodeError::NullForNonNullable))",
            flex_cond(flex_min, "get_compact_bytes(buf)", "get_bytes(buf)")
        ),
        other => format!("{}::decode(version, buf)", other),
    }
}

fn field_version_guard(field: &FieldSpec) -> (String, i16, i16) {
    let (min, max) = parse_versions(&field.versions);
    (field.versions.clone(), min, max)
}

fn version_guard_expr(min: i16, max: i16) -> String {
    match (min, max) {
        (0, i16::MAX) => "true".to_string(),
        (0, max) => format!("version <= {}", max),
        (min, i16::MAX) => format!("version >= {}", min),
        (min, max) => format!("version >= {} && version <= {}", min, max),
    }
}

/// Emit the opening line of a field block. For always-present fields (guard
/// "true") emit a bare block instead of `if true {` to avoid clippy noise.
fn guard_open(guard: &str) -> String {
    if guard == "true" {
        "        {\n".to_string()
    } else {
        format!("        if {} {{\n", guard)
    }
}

fn collect_inline_structs(fields: &[FieldSpec], out: &mut Vec<StructSpec>) {
    for field in fields {
        if !field.fields.is_empty() {
            let name = if field.field_type.starts_with("[]") {
                field.field_type[2..].trim().to_string()
            } else {
                field.field_type.clone()
            };
            out.push(StructSpec {
                name,
                fields: field.fields.clone(),
            });
            collect_inline_structs(&field.fields, out);
        }
    }
}
