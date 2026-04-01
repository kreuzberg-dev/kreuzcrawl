//! Tower-based service stack for HTTP request processing.
//!
//! Provides composable middleware layers following the Tower Service pattern,
//! consistent with liter-llm and kreuzberg.

mod cache;
mod rate_limit;
mod service;
#[cfg(feature = "tracing")]
mod tracing_layer;
mod types;
mod ua_rotation;

pub use cache::CrawlCacheLayer;
pub use rate_limit::PerDomainRateLimitLayer;
pub use service::HttpFetchService;
#[cfg(feature = "tracing")]
pub use tracing_layer::CrawlTracingLayer;
pub use types::{CrawlRequest, CrawlResponse};
pub use ua_rotation::UaRotationLayer;
