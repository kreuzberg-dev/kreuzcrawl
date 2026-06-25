# WAF Fixture Corpus

Each `.http` file in this directory is a synthetic response capture that must
be matched by exactly one fingerprint in `rules/waf_fingerprints.toml`.

## File naming convention

```text
<vendor>_<scenario>_<n>.http
```

- `vendor` — lowercase vendor name (cloudflare, datadome, perimeterx, imperva, aws_waf, incapsula)
- `scenario` — brief description (challenge, block, rate_limit, captcha, interstitial)
- `n` — disambiguation integer when multiple fixtures cover the same scenario

## HTTP file format

```text
HTTP/1.1 <status> <reason>
<header-name>: <header-value>
...
<blank line>
<body>
```

Body continues to end of file. No trailing newline is required.

## Expected fingerprint

The test (`tests/test_waf_fixtures.rs`) derives the expected vendor from the
filename prefix. The fixture must be matched by the classifier.

A fixture named `cloudflare_challenge_1.http` must produce a `WafSignal`
whose `vendor == "cloudflare"`.

## Adding a fixture

1. Capture or synthesise a response that matches a real WAF challenge page.
2. Anonymise: remove cookies, auth headers, real hostnames, and API keys.
3. Place it here following the naming convention.
4. Run `cargo test -p crawlberg --test test_waf_fixtures` to verify.

Fixtures must be synthetic or anonymised captures — never commit real session
cookies or tokens.
