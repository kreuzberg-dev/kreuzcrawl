#!/usr/bin/env bash
# Spawn the alef-generated mock-server, parse its env markers
# (MOCK_SERVER_URL + per-fixture MOCK_SERVERS={...}), export per-fixture
# MOCK_SERVER_<UPPER_ID> vars, then run the generated brew bash suite.
# Tears the server down on exit.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MOCK_SERVER_BIN="$REPO_ROOT/e2e/rust/target/release/mock-server"
FIXTURES_DIR="$REPO_ROOT/fixtures"
BREW_DIR="$REPO_ROOT/e2e/brew"

# Prepend the freshly-built CLI's release directory so tests resolve our
# binary, not any system-installed crawlberg (e.g. Homebrew Cellar).
export PATH="$REPO_ROOT/target/release:${PATH}"

if [ ! -x "$MOCK_SERVER_BIN" ]; then
  echo "mock-server binary not found at $MOCK_SERVER_BIN — run 'task mock-server:rebuild'" >&2
  exit 1
fi

TMP_OUT=$(mktemp)
TMP_FIFO=$(mktemp -u)
mkfifo "$TMP_FIFO"
# Open a fd to the fifo to hold its read side; mock-server treats stdin EOF as
# a shutdown signal so we keep the write end alive until cleanup.
exec 9<>"$TMP_FIFO"
rm -f "$TMP_FIFO"

"$MOCK_SERVER_BIN" "$FIXTURES_DIR" <&9 >"$TMP_OUT" 2>&1 &
SERVER_PID=$!

cleanup() {
  exec 9>&- || true
  kill "$SERVER_PID" 2>/dev/null || true
  wait "$SERVER_PID" 2>/dev/null || true
  rm -f "$TMP_OUT"
}
trap cleanup EXIT

# Wait up to 30s for the MOCK_SERVER_URL line to appear.
for _ in $(seq 1 300); do
  if grep -q '^MOCK_SERVER_URL=' "$TMP_OUT" 2>/dev/null; then break; fi
  sleep 0.1
done

MOCK_SERVER_URL_LINE=$(grep '^MOCK_SERVER_URL=' "$TMP_OUT" | head -n1 || true)
if [ -z "$MOCK_SERVER_URL_LINE" ]; then
  echo "mock-server failed to emit MOCK_SERVER_URL; output:" >&2
  cat "$TMP_OUT" >&2
  exit 1
fi
export MOCK_SERVER_URL="${MOCK_SERVER_URL_LINE#MOCK_SERVER_URL=}"

# Give the server up to 2s to print MOCK_SERVERS={...} (only emitted when
# host-root fixtures like robots.txt / sitemap.xml are configured).
for _ in $(seq 1 20); do
  if grep -q '^MOCK_SERVERS=' "$TMP_OUT" 2>/dev/null; then break; fi
  sleep 0.1
done
MOCK_SERVERS_LINE=$(grep '^MOCK_SERVERS=' "$TMP_OUT" | head -n1 || true)
if [ -n "$MOCK_SERVERS_LINE" ]; then
  export MOCK_SERVERS="${MOCK_SERVERS_LINE#MOCK_SERVERS=}"
  # Export MOCK_SERVER_<UPPER_ID>=<url> for each fixture in the JSON map.
  while IFS=$'\t' read -r fid furl; do
    [ -z "$fid" ] && continue
    upper_id=$(echo "$fid" | tr '[:lower:]-' '[:upper:]_')
    export "MOCK_SERVER_${upper_id}=${furl}"
  done < <(echo "$MOCK_SERVERS" | jq -r 'to_entries[] | "\(.key)\t\(.value)"')
fi

cd "$BREW_DIR"
bash run_tests.sh
