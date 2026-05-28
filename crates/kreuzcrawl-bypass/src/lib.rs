//! Configurable HTTP-based `BypassProvider` implementations for kreuzcrawl.
//!
//! This crate lets you drive any HTTP-based bypass vendor through a YAML config
//! file without writing Rust code. Drop a config in `configs/`, set the env
//! vars it references, and wire up a [`SimpleHttpProvider`].
//!
//! # Quick start
//!
//! ```no_run
//! use std::path::Path;
//! use kreuzcrawl_bypass::{SimpleHttpProvider, load_with_process_env};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = load_with_process_env(Path::new("configs/bright_data.yaml"))?;
//! let provider = SimpleHttpProvider::new(config)?;
//! # let _ = provider;
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod error;
pub mod extract;
pub mod loader;
pub mod provider;

pub use config::ProviderConfig;
pub use error::{ConfigError, ProviderError};
pub use loader::{load_with_env, load_with_process_env};
pub use provider::SimpleHttpProvider;
