//! Header inspection helpers for the WAF detection path.
//!
//! The hardcoded `headers_only_waf_match` function previously in this module
//! has been folded into `Rules::classify` (Pass 1 — header-first short-circuit).
//! The TOML corpus in `rules/waf_fingerprints.toml` is now the single source
//! of truth for all WAF signals, including header-stamp early-exit.
