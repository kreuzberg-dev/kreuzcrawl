dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([kreuzcrawl],
  [whether to enable the kreuzcrawl extension],
  [AS_HELP_STRING([--enable-kreuzcrawl],
    [Enable kreuzcrawl extension support])],
  [yes])

if test "$PHP_KREUZCRAWL_ENABLED" = "yes"; then
  dnl Register the extension directory so phpize creates modules/ and sets up build rules.
  PHP_NEW_EXTENSION(kreuzcrawl, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library and copy it to modules/.
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/kreuzcrawl-php/Cargo.toml"; then
      (cd crates/kreuzcrawl-php && cargo build --release) || exit 1

      dnl Detect output filename based on platform
      if test -f "crates/kreuzcrawl-php/target/release/libkreuzcrawl_php.dylib"; then
        cargo_lib="crates/kreuzcrawl-php/target/release/libkreuzcrawl_php.dylib"
      elif test -f "crates/kreuzcrawl-php/target/release/libkreuzcrawl_php.so"; then
        cargo_lib="crates/kreuzcrawl-php/target/release/libkreuzcrawl_php.so"
      else
        echo "ERROR: cargo build succeeded but .so/.dylib not found in crates/kreuzcrawl-php/target/release" >&2
        exit 1
      fi

      mkdir -p modules
      cp "$cargo_lib" "modules/kreuzcrawl.so" || exit 1
    else
      echo "ERROR: crates/kreuzcrawl-php/Cargo.toml not found" >&2
      exit 1
    fi
  ], [])
fi
