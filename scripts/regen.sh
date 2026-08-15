#!/usr/bin/env bash
#
# Clean, regenerate the Rust protocol code from a Kafka release, and re-run the
# compatibility test — all in one shot.
#
#   scripts/regen.sh                  # regen at the CURRENT pinned tag
#   scripts/regen.sh 4.3.1            # regen at an explicit tag
#   scripts/regen.sh latest           # resolve + regen at the newest stable release
#   FORCE_DOWNLOAD=1 scripts/regen.sh 4.3.1   # ignore cached schemas
#   SKIP_COMPAT=1   scripts/regen.sh 4.3.1     # regenerate only, skip Java compat
#
# Schemas are pulled from the GitHub release tarball (codeload) — NOT the API —
# so there are no rate limits and no GITHUB_TOKEN needed. "latest" resolution
# uses `git ls-remote` (also unauthenticated and not rate-limited).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Newest stable release tag (X.Y.Z only — no -rc/-beta, no ^{} peel entries).
latest_stable_tag() {
  git ls-remote --tags https://github.com/apache/kafka.git 2>/dev/null \
    | sed -n 's|.*refs/tags/\([0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*\)$|\1|p' \
    | sort -uV | tail -1
}

# Default tag: whatever the current generated code was built from, so a bare
# `scripts/regen.sh` can never silently change versions. Falls back to 4.3.1
# on a fresh tree. Pass "latest" to explicitly track the newest release.
CURRENT_TAG="$(sed -n 's/^pub const KAFKA_VERSION: &str = "\(.*\)";$/\1/p' \
  "${ROOT}/kafka-wire-codec/src/generated/mod.rs" 2>/dev/null || true)"
TAG="${1:-${CURRENT_TAG:-4.3.1}}"

if [[ "${TAG}" == "latest" ]]; then
  echo "==> Resolving newest stable Kafka release from GitHub tags"
  TAG="$(latest_stable_tag)"
  [[ -n "${TAG}" ]] || { echo "ERROR: could not resolve latest Kafka tag (network?)" >&2; exit 1; }
  echo "==> Latest stable release: ${TAG}"
else
  # Pinned run: point out (non-fatally) when a newer release exists.
  NEWEST="$(latest_stable_tag || true)"
  if [[ -n "${NEWEST}" && "${NEWEST}" != "${TAG}" ]] \
     && [[ "$(printf '%s\n%s\n' "${TAG}" "${NEWEST}" | sort -V | tail -1)" == "${NEWEST}" ]]; then
    echo "NOTE: Kafka ${NEWEST} is available (regenerating at pinned ${TAG})." >&2
    echo "      Run 'scripts/regen.sh latest' to upgrade with full compat verification." >&2
  fi
fi
CACHE="${TMPDIR:-/tmp}/kafka-schema-${TAG}"
TARBALL="${TMPDIR:-/tmp}/kafka-${TAG}.tar.gz"
SCHEMA_SUBPATH="kafka-${TAG}/clients/src/main/resources/common/message"

echo "==> Kafka tag: ${TAG}"
echo "==> Repo root: ${ROOT}"

# 1. Clean generated code + fixtures -------------------------------------------
echo "==> Cleaning generated code and fixtures"
rm -f  "${ROOT}/kafka-wire-codec/src/generated/"*.rs
rm -f  "${ROOT}/compat-tests/fixtures/compat.bin" \
       "${ROOT}/compat-tests/fixtures/compat.version"

# 2. Fetch schemas from the release tarball (no API rate limit) ----------------
if [[ "${FORCE_DOWNLOAD:-0}" == "1" ]]; then
  rm -rf "${CACHE}"
fi
if [[ ! -d "${CACHE}/message" ]]; then
  echo "==> Downloading ${TAG} source tarball (codeload)"
  curl -fsSL "https://codeload.github.com/apache/kafka/tar.gz/refs/tags/${TAG}" -o "${TARBALL}"
  mkdir -p "${CACHE}"
  echo "==> Extracting message schemas"
  tar -xzf "${TARBALL}" -C "${CACHE}" --strip-components=6 "${SCHEMA_SUBPATH}"
else
  echo "==> Using cached schemas: ${CACHE}/message"
fi
echo "==> $(ls "${CACHE}/message"/*.json | wc -l | tr -d ' ') schema files available"

# 3. Regenerate Rust code ------------------------------------------------------
echo "==> Regenerating Rust protocol code"
KAFKA_SCHEMA_DIR="${CACHE}/message" cargo run -q -p kafka-codegen -- "${TAG}"

# 4. Build + test --------------------------------------------------------------
echo "==> Building workspace"
cargo build

if [[ "${SKIP_COMPAT:-0}" == "1" ]]; then
  echo "==> SKIP_COMPAT=1 — skipping Java compat test"
  echo "==> Done. Generated from Kafka ${TAG}."
  exit 0
fi

echo "==> Running compatibility test against kafka-clients ${TAG}"
cargo test --test compat -- --nocapture

echo "==> Done. Generated and verified against Kafka ${TAG}."
