dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([kreuzcrawl],
  [whether to enable the kreuzcrawl extension],
  [AS_HELP_STRING([--enable-kreuzcrawl],
    [Enable kreuzcrawl extension support])],
  [yes])

if test "$PHP_KREUZCRAWL_ENABLED" = "yes"; then
  dnl Recognize the extension directory for phpize/make
  PHP_NEW_EXTENSION(kreuzcrawl, [], $ext_shared)
fi
