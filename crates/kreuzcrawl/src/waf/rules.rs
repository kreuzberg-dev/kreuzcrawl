//! TOML loader, validation, and the compiled `Rules` struct.

use std::collections::HashMap;
use std::path::Path;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpResponse;
use crate::types::WafSignal;

/// Maximum body size (bytes) at which body fingerprints are checked on 2xx.
/// Real content pages overwhelmingly exceed this; challenge pages are tiny.
pub(crate) const CHALLENGE_BODY_LIMIT: usize = 100 * 1024;

// ---------------------------------------------------------------------------
// TOML schema types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct TomlRules {
    fingerprint: Vec<TomlFingerprint>,
}

#[derive(Debug, Deserialize, Clone)]
struct TomlFingerprint {
    id: String,
    vendor: String,
    weight: f32,
    signals: Vec<TomlSignal>,
}

#[derive(Debug, Deserialize, Clone)]
struct TomlSignal {
    kind: String,
    name: Option<String>,
    value_contains: Option<String>,
    pattern: Option<String>,
}

// ---------------------------------------------------------------------------
// Compiled types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Signal {
    /// Header must be present. `value_contains` is an optional substring.
    ResponseHeader {
        name: String,
        value_contains: Option<String>,
    },
    /// Body substring (case-insensitive). Matched via Aho-Corasick.
    /// The actual matching is done by the AC automaton; the fingerprint index
    /// is retrieved via `Rules::pattern_to_fp`. No per-signal fields needed at
    /// match time — this variant's presence indicates the fingerprint requires
    /// at least one body match (checked via `matched_fp_indices` in `classify`).
    BodySubstring,
}

#[derive(Debug, Clone)]
pub(crate) struct Fingerprint {
    pub(crate) id: String,
    pub(crate) vendor: String,
    pub(crate) weight: f32,
    pub(crate) signals: Vec<Signal>,
}

/// Compiled WAF rules: fingerprint list + single Aho-Corasick automaton.
///
/// `builtin()` loads from the compile-time TOML corpus. Hot-reload swaps
/// the `Rules` wrapped in [`arc_swap::ArcSwap`] (Commit 1.6).
#[derive(Debug)]
pub struct Rules {
    pub(crate) fingerprints: Vec<Fingerprint>,
    pub(crate) automaton: AhoCorasick,
    /// Maps an AC pattern index → fingerprint index in `fingerprints`.
    pub(crate) pattern_to_fp: Vec<usize>,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Error returned when loading or validating a rules file.
#[derive(Debug, Error)]
pub enum RulesError {
    /// TOML parse failure.
    #[error("parse error: {0}")]
    ParseError(#[from] toml::de::Error),
    /// A fingerprint failed validation.
    #[error("validation error for fingerprint '{fingerprint_id}': {reason}")]
    Validation {
        /// The `id` field of the fingerprint that failed validation.
        fingerprint_id: String,
        /// Human-readable description of the validation failure.
        reason: String,
    },
    /// Aho-Corasick build failure.
    #[error("failed to build Aho-Corasick automaton: {0}")]
    MatcherBuild(String),
}

// ---------------------------------------------------------------------------
// Loading
// ---------------------------------------------------------------------------

/// Load and compile rules from a TOML file on disk.
///
/// Useful for hot-reload (Commit 1.6) when the caller manages the rules file.
#[allow(dead_code)]
pub fn load_from_path(path: &Path) -> Result<Rules, RulesError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| RulesError::MatcherBuild(format!("cannot read {}: {e}", path.display())))?;
    load_from_str(&content)
}

/// Load and compile rules from a TOML string.
pub fn load_from_str(toml_src: &str) -> Result<Rules, RulesError> {
    let parsed: TomlRules = toml::from_str(toml_src)?;
    compile(parsed)
}

impl Rules {
    /// Load the canonical built-in corpus embedded at compile time.
    ///
    /// The corpus is validated by unit tests; a broken corpus is a
    /// programming error so `expect` is appropriate here.
    pub fn builtin() -> Self {
        let src = include_str!("../../rules/waf_fingerprints.toml");
        load_from_str(src).expect("builtin waf_fingerprints.toml must be valid")
    }
}

// ---------------------------------------------------------------------------
// Compilation
// ---------------------------------------------------------------------------

fn compile(raw: TomlRules) -> Result<Rules, RulesError> {
    let mut fingerprints: Vec<Fingerprint> = Vec::with_capacity(raw.fingerprint.len());
    let mut ac_patterns: Vec<String> = Vec::new();
    let mut pattern_to_fp: Vec<usize> = Vec::new();

    // Track ids for uniqueness validation.
    let mut seen_ids: HashMap<String, ()> = HashMap::new();

    for (fp_idx, raw_fp) in raw.fingerprint.iter().enumerate() {
        // Validate uniqueness.
        if seen_ids.contains_key(&raw_fp.id) {
            return Err(RulesError::Validation {
                fingerprint_id: raw_fp.id.clone(),
                reason: "duplicate fingerprint id".into(),
            });
        }
        // Validate id format: snake_case, no dots.
        if raw_fp.id.contains('.') {
            return Err(RulesError::Validation {
                fingerprint_id: raw_fp.id.clone(),
                reason: "fingerprint id must not contain dots".into(),
            });
        }
        seen_ids.insert(raw_fp.id.clone(), ());

        let mut signals: Vec<Signal> = Vec::with_capacity(raw_fp.signals.len());

        for raw_sig in &raw_fp.signals {
            match raw_sig.kind.as_str() {
                "response_header" => {
                    let name = raw_sig
                        .name
                        .clone()
                        .ok_or_else(|| RulesError::Validation {
                            fingerprint_id: raw_fp.id.clone(),
                            reason: "response_header signal requires 'name'".into(),
                        })?
                        .to_lowercase();
                    signals.push(Signal::ResponseHeader {
                        name,
                        value_contains: raw_sig.value_contains.as_ref().map(|s| s.to_lowercase()),
                    });
                }
                "body_substring" => {
                    let pattern = raw_sig
                        .pattern
                        .clone()
                        .ok_or_else(|| RulesError::Validation {
                            fingerprint_id: raw_fp.id.clone(),
                            reason: "body_substring signal requires 'pattern'".into(),
                        })?
                        .to_lowercase();
                    ac_patterns.push(pattern);
                    pattern_to_fp.push(fp_idx);
                    signals.push(Signal::BodySubstring);
                }
                other => {
                    return Err(RulesError::Validation {
                        fingerprint_id: raw_fp.id.clone(),
                        reason: format!("unknown signal kind '{other}'"),
                    });
                }
            }
        }

        fingerprints.push(Fingerprint {
            id: raw_fp.id.clone(),
            vendor: raw_fp.vendor.clone(),
            weight: raw_fp.weight,
            signals,
        });
    }

    // Build Aho-Corasick over all body_substring patterns in one pass.
    // Use leftmost-first match kind so the first pattern hit per position is returned.
    let automaton = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostFirst)
        .build(ac_patterns)
        .map_err(|e| RulesError::MatcherBuild(e.to_string()))?;

    Ok(Rules {
        fingerprints,
        automaton,
        pattern_to_fp,
    })
}

// ---------------------------------------------------------------------------
// Classification
// ---------------------------------------------------------------------------

impl Rules {
    /// Inspect `response` and return the first matching [`WafSignal`], if any.
    ///
    /// On a 2xx response the body fingerprint check is only applied when the
    /// body is ≤ `CHALLENGE_BODY_LIMIT` — real content pages are much larger.
    /// Header signals are always checked regardless of status code.
    pub fn classify(&self, response: &HttpResponse) -> Option<WafSignal> {
        let is_2xx = (200..300).contains(&response.status);
        let body_too_large = response.body_bytes.len() > CHALLENGE_BODY_LIMIT;

        // Skip body matching on large 2xx responses (would be legitimate content).
        let check_body = !is_2xx || !body_too_large;

        // Run Aho-Corasick once over the body to collect matched fingerprint indices.
        let mut matched_fp_indices: std::collections::HashSet<usize> = std::collections::HashSet::new();
        if check_body {
            for mat in self.automaton.find_iter(&response.body) {
                let fp_idx = self.pattern_to_fp[mat.pattern().as_usize()];
                matched_fp_indices.insert(fp_idx);
            }
        }

        // Evaluate each fingerprint; return the first whose signals all satisfy.
        for (fp_idx, fingerprint) in self.fingerprints.iter().enumerate() {
            if self.fingerprint_matches(fingerprint, fp_idx, &matched_fp_indices, response, is_2xx) {
                let signal = WafSignal {
                    vendor: fingerprint.vendor.clone(),
                    fingerprint_id: fingerprint.id.clone(),
                    weight: fingerprint.weight,
                };

                // Emit per-fingerprint match counter via tracing structured event.
                // The OTel collector turns this into
                // kreuzcrawl_waf_fingerprint_matches_total{fingerprint_id, vendor}.
                #[cfg(feature = "tracing")]
                {
                    tracing::info!(
                        kreuzcrawl.waf.fingerprint_id = %signal.fingerprint_id,
                        kreuzcrawl.waf.vendor = %signal.vendor,
                        "kreuzcrawl_waf_fingerprint_matches_total"
                    );
                }

                return Some(signal);
            }
        }
        None
    }

    fn fingerprint_matches(
        &self,
        fingerprint: &Fingerprint,
        fp_idx: usize,
        matched_body_fps: &std::collections::HashSet<usize>,
        response: &HttpResponse,
        is_2xx: bool,
    ) -> bool {
        let body_too_large = response.body_bytes.len() > CHALLENGE_BODY_LIMIT;
        let check_body = !is_2xx || !body_too_large;

        for signal in &fingerprint.signals {
            match signal {
                Signal::BodySubstring => {
                    if !check_body {
                        // Body is over the limit; body signals cannot fire.
                        return false;
                    }
                    if !matched_body_fps.contains(&fp_idx) {
                        return false;
                    }
                }
                Signal::ResponseHeader { name, value_contains } => {
                    if !header_matches(&response.headers, name, value_contains.as_deref()) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Header matching helpers
// ---------------------------------------------------------------------------

/// Returns true if the header `name` is present and (optionally) any of its
/// values contain `value_contains` (case-insensitive).
fn header_matches(headers: &HashMap<String, Vec<String>>, name: &str, value_contains: Option<&str>) -> bool {
    // Handle x-px-* prefix match: any header starting with "x-px-" signals PX.
    if name == "x-px-" {
        return headers.keys().any(|k| k.starts_with("x-px-"));
    }

    match headers.get(name) {
        None => false,
        Some(values) => match value_contains {
            None => true,
            Some(needle) => values.iter().any(|v| v.to_lowercase().contains(needle)),
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_rules_parse_without_error() {
        let rules = Rules::builtin();
        assert!(
            !rules.fingerprints.is_empty(),
            "builtin must have at least one fingerprint"
        );
    }

    #[test]
    fn load_from_str_rejects_duplicate_id() {
        let src = r#"
[[fingerprint]]
id = "duplicate_id"
vendor = "test"
weight = 1.0
[[fingerprint.signals]]
kind = "body_substring"
pattern = "foo"

[[fingerprint]]
id = "duplicate_id"
vendor = "test"
weight = 1.0
[[fingerprint.signals]]
kind = "body_substring"
pattern = "bar"
"#;
        assert!(matches!(load_from_str(src), Err(RulesError::Validation { .. })));
    }

    #[test]
    fn load_from_str_rejects_unknown_signal_kind() {
        let src = r#"
[[fingerprint]]
id = "bad_signal"
vendor = "test"
weight = 1.0
[[fingerprint.signals]]
kind = "magic_beam"
"#;
        assert!(matches!(load_from_str(src), Err(RulesError::Validation { .. })));
    }
}
