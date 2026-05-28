//! Header inspection helpers for the WAF early-exit path.
//!
//! The core classification algorithm (Aho-Corasick body scan + signal
//! evaluation) lives in `Rules::classify` in `rules.rs`.
//! This module exposes `headers_only_waf_match`, used by `http.rs` for the
//! header-only 2xx early-exit check before the body is read.

use reqwest::header::HeaderMap;

/// Returns true if `headers` contain an unambiguous WAF-stamp header.
///
/// Called before the body is read (2xx early-exit path). Header signals are
/// sufficient on their own — when a WAF stamps its own response header there
/// is no false-positive risk.
pub(crate) fn headers_only_waf_match(headers: &HeaderMap) -> bool {
    headers.contains_key("x-sucuri-id")
        || headers.contains_key("x-datadome")
        || headers.contains_key("x-amzn-waf-action")
        || headers.keys().any(|k| k.as_str().starts_with("x-px-"))
}
