//! In-process domain-state backend, EWMA utility, and the learning
//! retry policy that consults a [`DomainStatePort`] for prior block
//! rates.
#![allow(dead_code)]

use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::types::{AttemptOutcome, DomainOutcome, DomainState, DomainStatePort, RetryDirective, RetryPolicy, Tier};

use super::dispatch::SimpleRetryPolicy;

/// Pure-math EWMA with promote/demote thresholds. Stateless — caller
/// supplies the prior and the observation.
#[derive(Debug, Clone, Copy)]
pub struct EwmaTracker {
    /// Smoothing factor 0.0 < alpha < 1.0. Higher = react faster.
    alpha: f32,
    /// Block rate above which the dispatcher should promote starting_tier.
    promote_threshold: f32,
    /// Block rate below which the dispatcher should demote starting_tier.
    demote_threshold: f32,
    /// Minimum sample count before promotion takes effect.
    min_samples_promote: u64,
    /// Minimum sample count before demotion takes effect.
    min_samples_demote: u64,
}

impl EwmaTracker {
    /// Recommended defaults: alpha=0.1 (~72h half-life at typical rates),
    /// promote at 0.4 / 10 samples, demote at 0.1 / 50 samples.
    pub const fn new() -> Self {
        Self {
            alpha: 0.1,
            promote_threshold: 0.4,
            demote_threshold: 0.1,
            min_samples_promote: 10,
            min_samples_demote: 50,
        }
    }

    /// Update the EWMA given whether the current observation was a block.
    pub fn update(&self, prev: f32, blocked: bool) -> f32 {
        let observation = if blocked { 1.0 } else { 0.0 };
        self.alpha.mul_add(observation, (1.0 - self.alpha) * prev)
    }

    /// Return `true` if the EWMA and sample count warrant promoting to a
    /// higher tier.
    pub fn should_promote(&self, ewma: f32, sample_count: u64) -> bool {
        sample_count >= self.min_samples_promote && ewma >= self.promote_threshold
    }

    /// Return `true` if the EWMA and sample count warrant demoting to a
    /// lower tier.
    pub fn should_demote(&self, ewma: f32, sample_count: u64) -> bool {
        sample_count >= self.min_samples_demote && ewma <= self.demote_threshold
    }
}

impl Default for EwmaTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Process-local domain state. `DashMap`-backed, ephemeral — no
/// persistence across restarts. For multi-process / multi-tenant
/// learning, use kreuzberg-cloud's PostgresDomainState.
#[derive(Debug, Default)]
pub struct InMemoryDomainState {
    inner: DashMap<String, DomainState>,
    ewma: EwmaTracker,
}

impl InMemoryDomainState {
    /// Create a new in-memory domain state with default EWMA settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the EWMA tracker configuration.
    pub fn with_ewma(mut self, ewma: EwmaTracker) -> Self {
        self.ewma = ewma;
        self
    }
}

#[async_trait]
impl DomainStatePort for InMemoryDomainState {
    async fn get(&self, domain: &str) -> Option<DomainState> {
        self.inner.get(domain).map(|s| s.clone())
    }

    async fn record_outcome(&self, domain: &str, outcome: &DomainOutcome) {
        let mut entry = self.inner.entry(domain.to_string()).or_insert_with(|| DomainState {
            block_ewma: 0.0,
            sample_count: 0,
            classifier: None,
            starting_tier: Tier::Http,
        });
        let next_ewma = self.ewma.update(entry.block_ewma, outcome.blocked);
        entry.block_ewma = next_ewma;
        entry.sample_count += 1;
        if let Some(sig) = &outcome.waf_signal {
            entry.classifier = Some(sig.vendor.clone());
        }
        if self.ewma.should_promote(entry.block_ewma, entry.sample_count) {
            entry.starting_tier = Tier::Bypass;
        } else if self.ewma.should_demote(entry.block_ewma, entry.sample_count) {
            entry.starting_tier = Tier::Http;
        }
    }
}

/// Retry policy that consults a [`DomainStatePort`] for the per-domain
/// prior on each decision. Falls back to [`SimpleRetryPolicy`] semantics
/// when no state is available for the domain.
#[derive(Debug)]
pub struct LearningRetryPolicy {
    state: Arc<dyn DomainStatePort>,
    fallback: SimpleRetryPolicy,
}

impl LearningRetryPolicy {
    /// Create a new learning policy backed by the given state port.
    pub fn new(state: Arc<dyn DomainStatePort>) -> Self {
        Self {
            state,
            fallback: SimpleRetryPolicy::new(),
        }
    }

    /// Override the fallback policy used for the immediate retry decision.
    pub fn with_fallback(mut self, fallback: SimpleRetryPolicy) -> Self {
        self.fallback = fallback;
        self
    }
}

#[async_trait]
impl RetryPolicy for LearningRetryPolicy {
    async fn decide(&self, outcome: &AttemptOutcome<'_>) -> RetryDirective {
        let directive = self.fallback.decide(outcome).await;

        // Record outcome for future learning, then return decision.
        if let Ok(parsed) = url::Url::parse(outcome.url)
            && let Some(domain) = parsed.host_str()
        {
            let blocked = matches!(
                outcome.error,
                Some(crate::error::CrawlError::WafBlocked { .. } | crate::error::CrawlError::Forbidden(_))
            ) || outcome.waf_signal.is_some();
            self.state
                .record_outcome(
                    domain,
                    &DomainOutcome {
                        tier: outcome.previous_tier,
                        blocked,
                        waf_signal: outcome.waf_signal.cloned(),
                    },
                )
                .await;
        }

        directive
    }

    fn name(&self) -> &'static str {
        "learning"
    }
}

/// Convenience constructor: `Arc<dyn DomainStatePort>` backed by an in-memory map.
#[must_use]
pub fn in_memory_domain_state() -> Arc<dyn DomainStatePort> {
    Arc::new(InMemoryDomainState::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::WafSignal;

    #[test]
    fn ewma_starts_at_zero_and_climbs() {
        let tracker = EwmaTracker::new();
        let mut ewma = 0.0;
        for _ in 0..20 {
            ewma = tracker.update(ewma, true);
        }
        assert!(
            ewma > 0.5,
            "ewma should climb past 0.5 after 20 blocked observations, got {ewma}"
        );
    }

    #[test]
    fn ewma_decays_to_zero() {
        let tracker = EwmaTracker::new();
        let mut ewma = 0.9;
        for _ in 0..50 {
            ewma = tracker.update(ewma, false);
        }
        assert!(
            ewma < 0.05,
            "ewma should decay below 0.05 after 50 clean observations, got {ewma}"
        );
    }

    #[test]
    fn promote_only_with_enough_samples() {
        let tracker = EwmaTracker::new();
        assert!(!tracker.should_promote(0.9, 5), "below min_samples_promote");
        assert!(tracker.should_promote(0.5, 10));
        assert!(!tracker.should_promote(0.3, 100), "ewma below threshold");
    }

    #[test]
    fn demote_requires_low_ewma_and_high_samples() {
        let tracker = EwmaTracker::new();
        assert!(!tracker.should_demote(0.05, 10), "below min_samples_demote");
        assert!(tracker.should_demote(0.05, 100));
        assert!(!tracker.should_demote(0.2, 100), "ewma above demote threshold");
    }

    #[tokio::test]
    async fn in_memory_state_records_and_returns() {
        let state = InMemoryDomainState::new();
        let outcome = DomainOutcome {
            tier: Tier::Http,
            blocked: true,
            waf_signal: Some(WafSignal {
                vendor: "cloudflare".into(),
                fingerprint_id: "cloudflare_challenge_v1".into(),
                weight: 1.0,
            }),
        };
        state.record_outcome("example.com", &outcome).await;
        let snapshot = state.get("example.com").await.expect("recorded");
        assert!(snapshot.block_ewma > 0.0);
        assert_eq!(snapshot.sample_count, 1);
        assert_eq!(snapshot.classifier.as_deref(), Some("cloudflare"));
    }

    #[tokio::test]
    async fn in_memory_state_promotes_starting_tier_after_streak() {
        let state = InMemoryDomainState::new();
        let outcome = DomainOutcome {
            tier: Tier::Http,
            blocked: true,
            waf_signal: None,
        };
        for _ in 0..30 {
            state.record_outcome("blocked.example", &outcome).await;
        }
        let snapshot = state.get("blocked.example").await.expect("recorded");
        assert_eq!(
            snapshot.starting_tier,
            Tier::Bypass,
            "should promote after sustained blocks"
        );
    }

    #[tokio::test]
    async fn in_memory_state_returns_none_for_unseen_domain() {
        let state = InMemoryDomainState::new();
        assert!(state.get("never-seen.example").await.is_none());
    }

    #[tokio::test]
    async fn learning_policy_records_outcome_on_waf_blocked() {
        let state = Arc::new(InMemoryDomainState::new());
        let policy = LearningRetryPolicy::new(state.clone() as Arc<dyn DomainStatePort>);
        let err = crate::error::CrawlError::WafBlocked {
            vendor: "cloudflare".into(),
            message: "cloudflare".into(),
        };
        let outcome = AttemptOutcome {
            attempt: 0,
            url: "https://example.com/path",
            status: None,
            error: Some(&err),
            waf_signal: None,
            body_size: 0,
            content_density: 0.0,
            bytes_transferred: None,
            previous_tier: Tier::Http,
        };
        let directive = policy.decide(&outcome).await;
        assert!(matches!(directive, RetryDirective::Escalate { .. }));
        let snapshot = state.get("example.com").await.expect("state should record domain");
        assert!(snapshot.block_ewma > 0.0, "blocked outcome should increase ewma");
        assert_eq!(snapshot.sample_count, 1);
    }

    #[tokio::test]
    async fn learning_policy_name_is_learning() {
        let state = Arc::new(InMemoryDomainState::new()) as Arc<dyn DomainStatePort>;
        let policy = LearningRetryPolicy::new(state);
        assert_eq!(policy.name(), "learning");
    }
}
