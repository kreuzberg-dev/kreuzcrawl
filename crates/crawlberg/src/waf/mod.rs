//! TOML-driven WAF fingerprint classifier.
//!
//! [`TomlClassifier`] implements [`crate::types::WafClassifier`] by loading
//! `rules/waf_fingerprints.toml` at compile time, building a single
//! Aho-Corasick automaton over all body patterns, and checking response
//! headers for WAF-stamp signals.
//!
//! Hot-reload: the classifier wraps [`Rules`] in [`arc_swap::ArcSwap`].
//! Callers can call [`TomlClassifier::swap`] to atomically replace the rule
//! set without restarting the process, or use [`TomlClassifier::watch`] to
//! watch a TOML rules file on disk and reload automatically on change.
//! [`WatchHandle`] stops the watcher and debounce task on drop.

use std::fmt;
use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::http::HttpResponse;
use crate::types::{WafClassifier, WafClassifyError, WafSignal};

pub(crate) mod matcher;
pub(crate) mod rules;
pub(crate) mod watch;

#[cfg(test)]
mod tests;

pub use rules::{Rules, RulesError, load_from_str};
pub use watch::{WatchError, WatchHandle};

/// Default [`WafClassifier`] backed by a TOML fingerprint corpus.
///
/// Use [`Self::builtin`] for the crawlberg-canonical corpus embedded at
/// compile time. Use [`Self::from_rules`] to supply a custom corpus (useful
/// in tests or when the caller manages the rules file).
pub struct TomlClassifier {
    rules: ArcSwap<Rules>,
}

impl fmt::Debug for TomlClassifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loaded = self.rules.load();
        formatter
            .debug_struct("TomlClassifier")
            .field("fingerprint_count", &loaded.fingerprints.len())
            .finish()
    }
}

impl TomlClassifier {
    /// Create a classifier from the built-in corpus.
    pub fn builtin() -> Self {
        Self {
            rules: ArcSwap::from_pointee(Rules::builtin()),
        }
    }

    /// Create a classifier from a caller-supplied [`Rules`] set.
    pub fn from_rules(rules: Rules) -> Self {
        Self {
            rules: ArcSwap::from_pointee(rules),
        }
    }

    /// Atomically swap the active rule set.
    ///
    /// Concurrent classifiers that have already called `load()` on the
    /// previous generation finish with the old rules; new calls see the
    /// new rules immediately. No locks, no readers blocked.
    pub fn swap(&self, rules: Rules) {
        self.rules.store(Arc::new(rules));
    }

    /// Watch a TOML rules file on disk and atomically swap the classifier's
    /// active rule set when the file changes. Returns a [`WatchHandle`] that
    /// stops the watcher on drop.
    ///
    /// Events are debounced 500 ms so editors that write via tmpfile + rename
    /// (and Kubernetes ConfigMap atomic projection) deliver one reload, not
    /// several. Reload errors are logged at warn level; the previous rules
    /// stay in place until a subsequent reload succeeds.
    ///
    /// # Errors
    ///
    /// Returns [`WatchError::Setup`] if the OS file-system watcher cannot be
    /// created, or [`WatchError::NoParent`] if `path` has no parent directory.
    pub fn watch(self: &Arc<Self>, path: impl AsRef<std::path::Path>) -> Result<WatchHandle, WatchError> {
        watch::start_watch(Arc::clone(self), path.as_ref())
    }
}

impl WafClassifier for TomlClassifier {
    fn classify(&self, response: &HttpResponse) -> Result<Option<WafSignal>, WafClassifyError> {
        self.rules.load().classify(response)
    }
}
