#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${REPO_ROOT:-$(cd "$SCRIPT_DIR/../../.." && pwd)}"

source "$REPO_ROOT/scripts/lib/common.sh"

validate_repo_root "$REPO_ROOT" || exit 1

cd "$REPO_ROOT"

echo "=== Running Rust unit tests ==="
echo "  Repository: $REPO_ROOT"
echo "  RUST_BACKTRACE: ${RUST_BACKTRACE:-not set}"
echo "  CARGO_TERM_COLOR: ${CARGO_TERM_COLOR:-not set}"

TEST_LOG="/tmp/cargo-test-$$.log"

if ! {
  echo "=== cargo test -p crawlberg --all-features ==="
  RUST_BACKTRACE=full cargo test -p crawlberg --all-features --verbose

  echo "=== cargo test --workspace (excluding bindings) ==="
  RUST_BACKTRACE=full cargo test \
    --workspace \
    --exclude crawlberg \
    --exclude crawlberg-py \
    --exclude crawlberg-node \
    --exclude crawlberg-php \
    --exclude crawlberg-wasm \
    --all-features \
    --verbose
} 2>&1 | tee "$TEST_LOG"; then
  echo "=== Test execution failed ==="
  echo "Last 50 lines of test output:"
  tail -n 50 "$TEST_LOG"
  rm -f "$TEST_LOG"
  exit 1
fi

rm -f "$TEST_LOG"

echo "=== Tests complete ==="
