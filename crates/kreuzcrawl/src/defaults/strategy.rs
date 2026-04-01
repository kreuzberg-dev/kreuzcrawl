//! Crawl strategies: BFS, DFS, best-first, and adaptive URL selection.

use std::collections::VecDeque;

use crate::traits::{CrawlStats, CrawlStrategy, FrontierEntry};
use crate::types::CrawlPageResult;

/// A breadth-first crawl strategy.
///
/// Always selects the first (oldest) entry from the frontier, giving BFS behavior.
#[derive(Debug, Clone, Default, Copy)]
pub struct BfsStrategy;

impl CrawlStrategy for BfsStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() { None } else { Some(0) }
    }
}

/// A depth-first crawl strategy.
///
/// Always selects the last (newest) entry from the working set, giving LIFO / DFS behavior.
#[derive(Debug, Clone, Default, Copy)]
pub struct DfsStrategy;

impl CrawlStrategy for DfsStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() {
            None
        } else {
            Some(candidates.len() - 1)
        }
    }
}

/// A best-first crawl strategy.
///
/// Selects the candidate with the highest `priority` value. Consumers can override
/// [`CrawlStrategy::score_url`] to provide custom scoring; the default scores by
/// inverse depth: `1.0 / (depth + 1.0)`.
#[derive(Debug, Clone, Default, Copy)]
pub struct BestFirstStrategy;

impl CrawlStrategy for BestFirstStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }
        let mut best_idx = 0;
        let mut best_priority = candidates[0].priority;
        for (i, entry) in candidates.iter().enumerate().skip(1) {
            if entry.priority > best_priority {
                best_priority = entry.priority;
                best_idx = i;
            }
        }
        Some(best_idx)
    }

    fn score_url(&self, _url: &str, depth: usize) -> f64 {
        1.0 / (depth as f64 + 1.0)
    }

    fn should_continue(&self, _stats: &CrawlStats) -> bool {
        true
    }
}

/// Adaptive crawling strategy that stops when content saturation is detected.
///
/// Tracks the number of new unique terms discovered per page. When the rate of
/// new term discovery drops below a configurable threshold (term saturation),
/// the strategy signals the engine to stop crawling.
#[derive(Debug)]
pub struct AdaptiveStrategy {
    /// Number of recent pages to consider for saturation detection.
    window_size: usize,
    /// Stop when new_terms_per_page drops below this fraction (0.0 to 1.0).
    saturation_threshold: f64,
    /// History of unique new terms per page.
    term_history: std::sync::Mutex<AdaptiveState>,
}

#[derive(Debug, Default)]
struct AdaptiveState {
    window: VecDeque<usize>,
    all_terms: ahash::AHashSet<String>,
}

impl AdaptiveStrategy {
    /// Create a new adaptive strategy.
    ///
    /// - `window_size`: Number of recent pages for saturation window (default: 10)
    /// - `saturation_threshold`: Stop when new terms per page ratio drops below this (default: 0.05)
    pub fn new(window_size: usize, saturation_threshold: f64) -> Self {
        Self {
            window_size,
            saturation_threshold,
            term_history: std::sync::Mutex::new(AdaptiveState::default()),
        }
    }

    /// Record a page's content for saturation tracking.
    pub fn record_page(&self, html: &str) {
        let mut state = self.term_history.lock().unwrap();
        let mut new_terms = 0usize;

        for token in html
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 2)
        {
            let lower = token.to_lowercase();
            if state.all_terms.insert(lower) {
                new_terms += 1;
            }
        }

        state.window.push_back(new_terms);
        if state.window.len() > self.window_size {
            state.window.pop_front();
        }
    }
}

impl Default for AdaptiveStrategy {
    fn default() -> Self {
        Self::new(10, 0.05)
    }
}

impl CrawlStrategy for AdaptiveStrategy {
    fn select_next(&self, candidates: &[FrontierEntry]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }
        // BFS-like: pick first candidate (can be enhanced with scoring)
        Some(0)
    }

    fn score_url(&self, _url: &str, depth: usize) -> f64 {
        1.0 / (depth as f64 + 1.0)
    }

    fn should_continue(&self, stats: &CrawlStats) -> bool {
        if stats.pages_crawled < self.window_size {
            return true; // Need enough data
        }

        let state = self.term_history.lock().unwrap();
        if state.window.len() < self.window_size {
            return true;
        }

        let avg_new_terms: f64 =
            state.window.iter().sum::<usize>() as f64 / state.window.len() as f64;
        let avg_total_per_page = state.all_terms.len() as f64 / stats.pages_crawled.max(1) as f64;
        let saturation_ratio = if avg_total_per_page > 0.0 {
            avg_new_terms / avg_total_per_page
        } else {
            1.0 // No data yet, continue
        };

        saturation_ratio > self.saturation_threshold
    }

    fn on_page_processed(&self, page: &CrawlPageResult) {
        self.record_page(&page.html);
    }
}
