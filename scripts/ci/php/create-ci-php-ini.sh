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

# Detect the active PHP's default extension_dir so we can preload bundled
# extensions (dom, json, mbstring, tokenizer, xml, xmlwriter, ctype, libxml)
# that PHPUnit requires. `php -c <ini>` REPLACES the default php.ini, so we
# must re-declare extension_dir + every needed extension here.
DEFAULT_EXT_DIR="$(php -r 'echo ini_get("extension_dir");' 2>/dev/null || true)"
if [ -z "$DEFAULT_EXT_DIR" ]; then
  DEFAULT_EXT_DIR="$(php-config --extension-dir 2>/dev/null || true)"
fi
if [ -z "$DEFAULT_EXT_DIR" ]; then
  echo "ERROR: could not determine PHP extension_dir"
  exit 1
fi

echo "Detected PHP extension_dir: $DEFAULT_EXT_DIR"

# Create the ini file with absolute path
# We load the Kreuzcrawl extension with its full path and set extension_dir to
# the active PHP's own dir so PHPUnit's required extensions resolve.
if cat >"$INI_FILE" <<EOF; then
; Kreuzcrawl PHP Extension Configuration for CI Testing
; This file is generated automatically by create-ci-php-ini.sh
; It allows loading the locally-built extension without system-wide installation

; Load the Kreuzcrawl PHP extension using full path
extension="$DISPLAY_DIR/$EXT_FILE"

; Mirror the active PHP's extension_dir so PHPUnit-required extensions resolve
extension_dir = $DEFAULT_EXT_DIR

; PHPUnit requires: dom, json, libxml, mbstring, tokenizer, xml, xmlwriter, ctype
extension = ctype
extension = dom
extension = libxml
extension = mbstring
extension = tokenizer
extension = xml
extension = xmlwriter
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
