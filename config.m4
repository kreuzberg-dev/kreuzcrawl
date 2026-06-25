dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([crawlberg],
  [whether to enable the crawlberg extension],
  [AS_HELP_STRING([--enable-crawlberg],
    [Enable crawlberg extension support])],
  [yes])

if test "$PHP_CRAWLBERG_ENABLED" = "yes"; then
  dnl Register the extension directory so phpize creates modules/ and sets up build rules.
  PHP_NEW_EXTENSION(crawlberg, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library and copy it to modules/.
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/crawlberg-php/Cargo.toml"; then
      (cd crates/crawlberg-php && cargo build --release) || exit 1

      dnl Detect output filename based on platform
      if test -f "crates/crawlberg-php/target/release/libcrawlberg_php.dylib"; then
        cargo_lib="crates/crawlberg-php/target/release/libcrawlberg_php.dylib"
      elif test -f "crates/crawlberg-php/target/release/libcrawlberg_php.so"; then
        cargo_lib="crates/crawlberg-php/target/release/libcrawlberg_php.so"
      else
        echo "ERROR: cargo build succeeded but .so/.dylib not found in crates/crawlberg-php/target/release" >&2
        exit 1
      fi

      mkdir -p modules
      cp "$cargo_lib" "modules/crawlberg.so" || exit 1
    else
      echo "ERROR: crates/crawlberg-php/Cargo.toml not found" >&2
      exit 1
    fi
  ], [])
fi
