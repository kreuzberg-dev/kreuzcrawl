#!/usr/bin/env bash

set -euo pipefail

npm_dir="${1:-crates/kreuzcrawl-node/npm}"

if [ ! -d "$npm_dir" ]; then
  echo "Error: npm directory not found: $npm_dir" >&2
  exit 1
fi

echo "=========================================="
echo "Packing Node platform packages"
echo "=========================================="
echo "npm directory: $npm_dir"
echo ""
echo "Directory structure:"
find "$npm_dir" -type f | head -30
echo ""

cd "$npm_dir"

platform_count=$(find . -maxdepth 1 -type d ! -name '.' | wc -l)
echo "Found $platform_count platform directories:"
find . -maxdepth 1 -type d ! -name '.' | sort
echo ""

success_count=0
for dir in */; do
  echo "=========================================="
  echo "Processing: $dir"
  echo "=========================================="

  if [ ! -f "${dir}package.json" ]; then
    echo "⚠ Skipping $dir (no package.json)"
    continue
  fi

  echo "✓ Found package.json"
  echo "Files in $dir:"
  find "$dir" -type f -print0 | xargs -0 ls -lah | tail -20

  shopt -s nullglob
  node_bins=("${dir}"*.node)

  # NAPI-RS pre-stages a directory for every platform listed in
  # `napi.targets` even when the build matrix only covers a subset. Skip
  # platforms that were not built rather than hard-failing the publish:
  # those platforms simply don't get published this release, and the
  # parent package's `optionalDependencies` entry resolves to a 404 at
  # install time, which is the documented graceful-fallback behaviour
  # for napi optional platform packages. Hard-failing here aborts the
  # entire publish — including platforms that DID build — and forces a
  # whole-matrix re-run for a missing platform that we explicitly chose
  # not to build (e.g. musl variants not in the publish matrix).
  if [ "${#node_bins[@]}" -eq 0 ]; then
    echo "⚠ Skipping ${dir} — no .node binary (platform not built this release)"
    echo ""
    continue
  fi

  echo "✓ Found .node binary: ${node_bins[0]}"
  echo "  File size: $(stat -f%z "${dir}${node_bins[0]}" 2>/dev/null || stat -c%s "${dir}${node_bins[0]}")"

  echo "Running npm pack..."
  if (cd "$dir" && npm pack && mv ./*.tgz ..); then
    echo "✓ Successfully packed $dir"
    success_count=$((success_count + 1))
  else
    echo "✗ Failed to pack $dir"
    exit 1
  fi
  echo ""
done

echo "=========================================="
echo "Summary: Successfully packed $success_count platform package(s)"
echo "========================================"
