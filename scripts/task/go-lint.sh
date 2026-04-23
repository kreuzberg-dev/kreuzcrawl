#!/usr/bin/env bash
set -euo pipefail

mode="${1:-check}"

root="$(git rev-parse --show-toplevel)"

export PATH="$HOME/go/bin:/usr/lib/golang/bin:${PATH:-}"
export PKG_CONFIG_PATH="$root/crates/kreuzcrawl-ffi:${PKG_CONFIG_PATH:-}"
export DYLD_LIBRARY_PATH="$root/target/release:$root/target/debug:${DYLD_LIBRARY_PATH:-}"
export LD_LIBRARY_PATH="$root/target/release:$root/target/debug:${LD_LIBRARY_PATH:-}"

# Ensure FFI library is built (Go bindings link against it via cgo).
if [ ! -f "$root/target/release/libkreuzcrawl_ffi.dylib" ] && [ ! -f "$root/target/release/libkreuzcrawl_ffi.so" ] && [ ! -f "$root/target/debug/libkreuzcrawl_ffi.dylib" ] && [ ! -f "$root/target/debug/libkreuzcrawl_ffi.so" ]; then
  echo "==> Building kreuzcrawl-ffi (required by Go bindings)..."
  cargo build -p kreuzcrawl-ffi 2>/dev/null
fi

# Go module directories in go.work
workspace_dirs=(
  packages/go/v4
  e2e/go
  tools/benchmark-harness/scripts
)

# Standalone modules NOT in go.work (duplicate module paths, need GOWORK=off)
# Note: these have broken replace directives when run locally; they work in CI
standalone_dirs=()

failed=0

lint_dir() {
  local dir="$1"
  local full="$root/$dir"

  if [ ! -f "$full/go.mod" ]; then
    return
  fi

  echo "==> Linting $dir"
  cd "$full"

  case "$mode" in
  fix)
    go fmt ./...
    golangci-lint run --config "$root/.golangci.yml" --fix ./... || failed=1
    ;;
  check)
    if gofmt -l . | read -r; then
      echo "  gofmt issues in $dir:"
      gofmt -l .
      failed=1
    fi
    golangci-lint run --config "$root/.golangci.yml" ./... || failed=1
    ;;
  *)
    echo "Usage: $0 [fix|check]" >&2
    exit 2
    ;;
  esac
}

for dir in "${workspace_dirs[@]}"; do
  lint_dir "$dir"
done

if [ ${#standalone_dirs[@]} -gt 0 ]; then
  for dir in "${standalone_dirs[@]}"; do
    GOWORK=off lint_dir "$dir"
  done
fi

exit $failed
