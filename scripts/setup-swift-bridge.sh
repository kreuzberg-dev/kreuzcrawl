#!/bin/bash
# Setup Swift bridge files after cargo build
# Usage: setup-swift-bridge.sh [debug|release]  (default: release)

set -e

PROFILE="${1:-release}"
BUILD_DIR="target/${PROFILE}/build"

# Find the most recently built output directory
OUT=$(find "$BUILD_DIR" -maxdepth 2 -type d -name out -path '*kreuzcrawl-swift-*' \
  -exec stat -f '%m %N' {} + 2>/dev/null | sort -rn | head -1 | cut -d' ' -f2-)
if [ -z "$OUT" ]; then
  echo "ERROR: Could not find swift-bridge build output in ${BUILD_DIR}/"
  exit 1
fi

echo "Using swift-bridge output from: $OUT"

# Ensure target directories exist
mkdir -p packages/swift/Sources/RustBridgeC
mkdir -p packages/swift/Sources/RustBridge

# Copy C headers
cat "$OUT/SwiftBridgeCore.h" "$OUT/kreuzcrawl-swift/kreuzcrawl-swift.h" \
  >packages/swift/Sources/RustBridgeC/RustBridgeC.h

# Copy Swift bridge files with import statement prepended
{
  printf 'import RustBridgeC\n'
  cat "$OUT/SwiftBridgeCore.swift"
} >packages/swift/Sources/RustBridge/SwiftBridgeCore.swift
{
  printf 'import RustBridgeC\n'
  cat "$OUT/kreuzcrawl-swift/kreuzcrawl-swift.swift"
} >packages/swift/Sources/RustBridge/kreuzcrawl-swift.swift

echo "Swift-bridge files setup complete"
