//! Page interaction module for action-based browser automation.
//!
//! The public action types and validation helpers are available regardless of
//! which browser backend is compiled. Runtime execution is selected from the
//! configured [`BrowserBackend`].

/// Page action types and validation limits.
pub mod actions;
#[cfg(feature = "browser-chromiumoxide")]
mod chromiumoxide;
/// Page action validation helpers.
pub mod validation;

pub use actions::{
    MAX_ACTIONS, MAX_SCRIPT_LEN, MAX_SCROLL_AMOUNT, MAX_SELECTOR_LEN, MAX_SINGLE_WAIT_MS, MAX_TEXT_LEN,
    MAX_TOTAL_WAIT_SECS, PageAction, ScrollDirection,
};
pub use validation::validate_actions;

use crate::engine::CrawlEngine;
use crate::error::CrawlError;
use crate::types::{BrowserBackend, InteractionResult};

/// Execute browser actions on a single page.
pub(crate) async fn run(
    engine: &CrawlEngine,
    url: &str,
    actions: &[PageAction],
) -> Result<InteractionResult, CrawlError> {
    validate_actions(actions)?;
    engine.config.validate()?;

    match engine.config.browser.backend {
        BrowserBackend::Chromiumoxide => run_chromiumoxide(url, actions, &engine.config).await,
        BrowserBackend::Native => Err(native_unsupported()),
    }
}

#[cfg(feature = "browser-chromiumoxide")]
async fn run_chromiumoxide(
    url: &str,
    actions: &[PageAction],
    config: &crate::types::CrawlConfig,
) -> Result<InteractionResult, CrawlError> {
    chromiumoxide::run(url, actions, config).await
}

#[cfg(not(feature = "browser-chromiumoxide"))]
async fn run_chromiumoxide(
    _url: &str,
    _actions: &[PageAction],
    _config: &crate::types::CrawlConfig,
) -> Result<InteractionResult, CrawlError> {
    Err(CrawlError::Unsupported(
        "interact() with BrowserBackend::Chromiumoxide requires the browser-chromiumoxide feature".into(),
    ))
}

fn native_unsupported() -> CrawlError {
    CrawlError::Unsupported(
        "interact() is not supported by BrowserBackend::Native yet; use BrowserBackend::Chromiumoxide".into(),
    )
}
