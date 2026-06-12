//! Global metric registry skeleton.
//!
//! [`MetricRegistry`] is a `'static` singleton obtained via [`registry()`].
//! Cluster 2 will populate it with all `crawl_*` instrument handles.
//! This file intentionally stays thin — its only job today is to hand callers
//! a stable reference to the kreuzcrawl meter.

use std::sync::OnceLock;

use opentelemetry::global;
use opentelemetry::metrics::Meter;

/// Holds all OTel instrument handles for kreuzcrawl.
///
/// Obtain the global instance via [`registry()`].
/// Cluster 2 adds typed counter / histogram fields here.
pub struct MetricRegistry {
    /// The underlying meter — retained so Cluster 2 can call `build()` on
    /// new instrument builders without touching `global::meter()` again.
    // Allowed: Cluster 2 will use this field to build metric instruments.
    #[allow(dead_code)]
    pub(crate) meter: Meter,
}

static REGISTRY: OnceLock<MetricRegistry> = OnceLock::new();

impl MetricRegistry {
    fn new() -> Self {
        Self {
            meter: global::meter("kreuzcrawl"),
        }
    }
}

/// Return a reference to the process-wide [`MetricRegistry`].
///
/// Initialises the registry on first call (lazy, thread-safe).
pub fn registry() -> &'static MetricRegistry {
    REGISTRY.get_or_init(MetricRegistry::new)
}
