//! Default trait implementations for kreuzcrawl.

mod cache;
mod emitter;
mod filter;
mod frontier;
mod llm_extractor;
mod rate_limiter;
mod store;
mod strategy;

#[cfg(not(target_arch = "wasm32"))]
pub use cache::DiskCache;
pub use cache::NoopCache;
pub use emitter::NoopEmitter;
pub use filter::{Bm25Filter, NoopFilter};
pub use frontier::InMemoryFrontier;
#[cfg(feature = "ai")]
pub use llm_extractor::LlmExtractor;
pub use rate_limiter::{NoopRateLimiter, PerDomainThrottle};
pub use store::NoopStore;
pub use strategy::{AdaptiveStrategy, BestFirstStrategy, BfsStrategy, DfsStrategy};
