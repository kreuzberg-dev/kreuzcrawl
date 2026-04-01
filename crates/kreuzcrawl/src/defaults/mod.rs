//! Default trait implementations for kreuzcrawl.

mod emitter;
mod filter;
mod frontier;
mod llm_extractor;
mod middleware;
mod rate_limiter;
mod store;
mod strategy;

pub use emitter::NoopEmitter;
pub use filter::{Bm25Filter, NoopFilter};
pub use frontier::InMemoryFrontier;
#[cfg(feature = "ai")]
pub use llm_extractor::LlmExtractor;
pub use middleware::{CachingMiddleware, NoopMiddleware, UaRotationMiddleware};
pub use rate_limiter::{NoopRateLimiter, PerDomainThrottle};
pub use store::NoopStore;
pub use strategy::{AdaptiveStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy};
