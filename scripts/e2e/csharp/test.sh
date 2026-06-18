#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

source "$REPO_ROOT/scripts/lib/common.sh"
source "$REPO_ROOT/scripts/lib/library-paths.sh"
source "$REPO_ROOT/scripts/lib/tessdata.sh"

validate_repo_root "$REPO_ROOT" || exit 1

setup_rust_ffi_paths "$REPO_ROOT"
setup_tessdata

case "${RUNNER_OS:-$(uname -s)}" in
Linux)
  PATH="/usr/bin:${PATH}"
  ;;
macOS)
  PATH="/opt/homebrew/bin:/usr/local/bin:${PATH}"
  ;;
Windows*)
  PATH="/c/Program Files/Tesseract-OCR:${PATH}"
  ;;
esac

cd "${REPO_ROOT}/e2e/csharp"
results_dir="${REPO_ROOT}/target/test-results/csharp-e2e"
mkdir -p "$results_dir"

# Source the mock server URL from the running mock server
if [ -f "${REPO_ROOT}/.mock-server.env" ]; then
  source "${REPO_ROOT}/.mock-server.env"
  echo "✓ Mock server URL: $MOCK_SERVER_URL"
fi

# KREUZCRAWL_ALLOW_PRIVATE_NETWORK is set at the task level (csharp:e2e:test)
# and inherited by this script. It must be set in the OS process environment
# (libc environ) BEFORE dotnet starts, not via managed Environment.SetEnvironmentVariable()
# in TestSetup.cs, because the P/Invoke'd Rust FFI reads via std::env::var().
export KREUZCRAWL_ALLOW_PRIVATE_NETWORK="${KREUZCRAWL_ALLOW_PRIVATE_NETWORK:-true}"

dotnet test Kreuzcrawl.E2eTests.csproj \
  -c Release \
  --logger "console;verbosity=diagnostic" \
  --logger "trx;LogFileName=csharp-e2e.trx" \
  --results-directory "$results_dir" \
  --diag "$results_dir/dotnet-test-diag.log" \
  --blame \
  --blame-crash \
  --blame-hang \
  --blame-hang-timeout 20m \
  --blame-hang-dump-type mini \
  --blame-crash-dump-type mini
