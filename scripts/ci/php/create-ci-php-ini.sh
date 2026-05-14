#!/bin/bash

set -e

# This script creates a php.ini file for CI testing that loads the built PHP extension.
# This allows PHPUnit to find and load the locally-built extension without requiring
# system-wide installation or sudo access.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../" && pwd)"
OUTPUT_DIR="${OUTPUT_DIR:-.}"
INI_FILE="$OUTPUT_DIR/php-kreuzcrawl.ini"

echo "=== Creating CI PHP ini file ==="
echo "Repo root: $REPO_ROOT"
echo "Output file: $INI_FILE"
echo ""

# Determine the extension file based on OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  EXT_FILE="libkreuzcrawl_php.so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  EXT_FILE="libkreuzcrawl_php.dylib"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
  EXT_FILE="kreuzcrawl_php.dll"
else
  echo "Warning: Unknown OS type: $OSTYPE - assuming Linux"
  EXT_FILE="libkreuzcrawl_php.so"
fi

# Prefer release build, fall back to debug. task php:build:dev produces
# target/debug/, task php:build:release produces target/release/. The generic
# CI "Build language binding" step calls build:dev for all langs.
BUILT_EXT=""
TARGET_DIR=""
for candidate_dir in "$REPO_ROOT/target/release" "$REPO_ROOT/target/debug"; do
  if [ -f "$candidate_dir/$EXT_FILE" ]; then
    BUILT_EXT="$candidate_dir/$EXT_FILE"
    TARGET_DIR="$candidate_dir"
    break
  fi
done

if [ -z "$BUILT_EXT" ]; then
  echo "ERROR: Built extension $EXT_FILE not found in target/release or target/debug"
  for candidate_dir in "$REPO_ROOT/target/release" "$REPO_ROOT/target/debug"; do
    echo ""
    echo "Available files in $candidate_dir:"
    find "$candidate_dir" -maxdepth 1 -iname "*kreuzcrawl*" -type f 2>/dev/null || echo "  (directory missing or empty)"
  done
  exit 1
fi

echo "Target dir: $TARGET_DIR"

echo "Found built extension: $BUILT_EXT"
echo "Extension file size: $(du -h "$BUILT_EXT" | cut -f1)"
echo ""

# Convert paths to format acceptable by PHP on Windows (forward slashes)
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
  # On Windows with MSYS, convert backslashes to forward slashes
  DISPLAY_DIR="${TARGET_DIR//\\/\/}"
else
  DISPLAY_DIR="$TARGET_DIR"
fi

# Create the ini file with absolute path
# We load the Kreuzcrawl extension with its full path to avoid overriding extension_dir
# This allows core PHP extensions to be loaded from their default location
if cat >"$INI_FILE" <<EOF; then
; Kreuzcrawl PHP Extension Configuration for CI Testing
; This file is generated automatically by create-ci-php-ini.sh
; It allows loading the locally-built extension without system-wide installation

; Load the Kreuzcrawl PHP extension using full path
; This avoids overriding extension_dir which would prevent core extensions from loading
extension="$DISPLAY_DIR/$EXT_FILE"

; Load additional extensions that the main PHP installation provides
; (necessary because -c option overrides php.ini, so we must explicitly load all needed extensions)
extension_dir = /opt/homebrew/lib/php/pecl/20240924
extension="ts_pack_core_php.so"
EOF
  echo "✓ INI file created: $INI_FILE"
  echo ""
  echo "INI file contents:"
  cat "$INI_FILE"
  echo ""
  echo "To use this file with PHPUnit:"
  echo "  php -c $INI_FILE vendor/bin/phpunit"
  echo ""
  echo "Or pass it to task:"
  echo "  task php:test:ci"
else
  echo "✗ Failed to create INI file"
  exit 1
fi
