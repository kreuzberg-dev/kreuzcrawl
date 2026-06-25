#!/usr/bin/env bash
set -euo pipefail

echo "=========================================="
echo "FFI Library Verification"
echo "=========================================="

if [[ "${RUNNER_OS:-}" == "Windows" ]]; then
  echo "Looking for Windows library files (.dll, .a, .lib)..."
  if ls target/x86_64-pc-windows-gnu/release/libcrawlberg_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in GNU target"
    ls -lh target/x86_64-pc-windows-gnu/release/libcrawlberg_ffi.*
  elif ls target/release/libcrawlberg_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in release target"
    ls -lh target/release/libcrawlberg_ffi.*
  else
    echo "✗ Error: FFI library not found in expected locations"
    find . -name "libcrawlberg_ffi.*" -o -name "crawlberg_ffi.*" 2>/dev/null || echo "No FFI library files found"
    exit 1
  fi
else
  echo "Looking for Unix library files (.so, .dylib, .a)..."
  if ls target/release/libcrawlberg_ffi.* 2>/dev/null; then
    echo "✓ Found FFI library in target/release"
    ls -lh target/release/libcrawlberg_ffi.*
  else
    echo "✗ Error: FFI library not found in target/release"
    exit 1
  fi
fi
