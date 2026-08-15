#!/usr/bin/env bash
#
# Run the Java-vs-Rust compatibility test against the currently generated code.
#
#   scripts/run_compat_test.sh                 # build fixtures if stale, then verify
#   REBUILD_FIXTURES=1 scripts/run_compat_test.sh   # force-rebuild the fixture file
#
# The test builds the Java fixture generator with the SAME kafka-clients version
# the Rust code was generated from (kafka_wire_codec::KAFKA_VERSION) and asserts
# every message/version round-trips byte-for-byte.
#
# Requires JDK 11+ and Maven on PATH. Set SKIP_COMPAT_TESTS=1 to skip the body.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Generated code must exist (the dispatch table + message modules).
if ! ls kafka-wire-codec/src/generated/dispatch.rs >/dev/null 2>&1; then
  echo "ERROR: no generated code found. Run scripts/regen.sh <TAG> first." >&2
  exit 1
fi

if [[ "${REBUILD_FIXTURES:-0}" == "1" ]]; then
  echo "==> Removing existing fixtures to force a rebuild"
  rm -f compat-tests/fixtures/compat.bin compat-tests/fixtures/compat.version
fi

echo "==> Running compatibility test"
cargo test --test compat -- --nocapture
