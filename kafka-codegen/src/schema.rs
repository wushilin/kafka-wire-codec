use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageSpec {
    pub api_key: Option<i16>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub name: String,
    pub valid_versions: String,
    #[serde(default = "default_none_versions")]
    pub flexible_versions: String,
    #[serde(default)]
    pub fields: Vec<FieldSpec>,
    #[serde(default)]
    pub common_structs: Vec<StructSpec>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub versions: String,
    #[serde(default)]
    pub nullable_versions: String,
    /// Semantic entity annotation (e.g. "topicName", "brokerId") — mapped to a
    /// typed newtype in the generated code.
    pub entity_type: Option<String>,
    /// Tag number for flexible-version tagged fields. When present, the field
    /// lives in the trailing tagged-fields section, not inline. Kafka encodes
    /// this as a JSON string (e.g. "0"), so accept any JSON value — only its
    /// presence matters to the generator.
    pub tag: Option<serde_json::Value>,
    #[serde(default)]
    pub fields: Vec<FieldSpec>,
    #[serde(default)]
    pub about: String,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructSpec {
    pub name: String,
    #[serde(default)]
    pub fields: Vec<FieldSpec>,
}

fn default_none_versions() -> String {
    "none".to_string()
}

/// Parse a version range string like "0+", "0-5", "none" into (min, max).
/// max = i16::MAX means unbounded.
///
/// Malformed input is a codegen-time schema error: fail loudly instead of
/// guessing a range and silently generating wrong version gating.
pub fn parse_versions(s: &str) -> (i16, i16) {
    let s = s.trim();
    if s == "none" {
        return (i16::MAX, i16::MIN); // empty range
    }
    let parse = |part: &str| -> i16 {
        part.trim()
            .parse::<i16>()
            .unwrap_or_else(|_| panic!("malformed version range in schema: {:?}", s))
    };
    if let Some(base) = s.strip_suffix('+') {
        return (parse(base), i16::MAX);
    }
    if let Some((lo, hi)) = s.split_once('-') {
        return (parse(lo), parse(hi));
    }
    let v = parse(s);
    (v, v)
}
