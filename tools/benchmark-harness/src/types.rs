//! Core data types shared across the benchmark harness.

use serde::{Deserialize, Serialize};

/// A single entry from a scrape-evals dataset or custom fixture file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeFixture {
    /// Unique identifier for this fixture.
    pub id: String,
    /// The URL to scrape.
    pub url: String,
    /// The ground-truth text that should appear in the extracted content.
    pub truth_text: Option<String>,
    /// Text that should be absent from the extracted content ("lies").
    pub lie_text: Option<String>,
    /// Expected error message, if the fixture represents a failure case.
    pub error: Option<String>,
    /// Dataset split: `"train"`, `"test"`, etc.
    pub split: Option<String>,
    /// Arbitrary tags for filtering.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Expected HTTP status code.
    pub expected_status: Option<u16>,
}

/// Quality metrics computed for a single scrape result against a fixture.
///
/// Uses kreuzberg's TF1 scoring: multiset token-level F1 with separate
/// numeric scoring and lie_text noise penalty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeQualityMetrics {
    /// Text F1 score (all tokens). 0.0–1.0.
    pub f1_text: f64,
    /// Numeric-only F1 score (tokens containing digits). 0.0–1.0.
    pub f1_numeric: f64,
    /// Combined quality score: 0.6*f1_text + 0.4*f1_numeric. 0.0–1.0.
    pub quality_score: f64,
    /// Precision: fraction of extracted tokens that appear in ground truth.
    pub precision: f64,
    /// Recall: fraction of ground truth tokens found in extraction.
    pub recall: f64,
    /// Noise penalty: fraction of lie_text tokens found in extraction.
    /// 0.0 = clean (no lie tokens present), 1.0 = all lie tokens present.
    pub noise_penalty: f64,
    /// Tokens in ground truth but missing from extraction (recall misses), sorted by count.
    pub missing_tokens: Vec<(String, usize)>,
    /// Tokens in extraction but not in ground truth (precision misses), sorted by count.
    pub extra_tokens: Vec<(String, usize)>,
    /// Whether quality_score >= 0.95.
    pub correct: bool,
}

/// System-level performance metrics captured during a benchmark run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Peak RSS memory observed during the run, in bytes.
    pub peak_memory_bytes: u64,
    /// Average CPU utilization as a percentage (0–100).
    pub avg_cpu_percent: f64,
    /// Pages scraped per second.
    pub throughput_pages_per_sec: f64,
    /// 50th-percentile memory footprint, in bytes.
    pub p50_memory_bytes: u64,
    /// 95th-percentile memory footprint, in bytes.
    pub p95_memory_bytes: u64,
    /// 99th-percentile memory footprint, in bytes.
    pub p99_memory_bytes: u64,
}

/// Outcome of a single warmup or benchmark iteration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationResult {
    /// Zero-based iteration index.
    pub iteration: usize,
    /// Wall-clock duration of this iteration, in milliseconds.
    pub duration_ms: f64,
    /// Whether the iteration completed without error.
    pub success: bool,
    /// Error message, if the iteration failed.
    pub error: Option<String>,
    /// RSS memory of the process tree observed immediately after this scrape, in bytes.
    ///
    /// Captured via a point-in-time snapshot taken right after the scrape call returns,
    /// so it reflects the peak footprint for this single iteration rather than the
    /// global monitor's aggregate across all concurrent tasks.
    pub memory_bytes: u64,
}

/// Descriptive statistics computed over a collection of durations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationStatistics {
    /// Arithmetic mean duration, in milliseconds.
    pub mean_ms: f64,
    /// Median (50th percentile) duration, in milliseconds.
    pub median_ms: f64,
    /// Sample standard deviation, in milliseconds.
    pub std_dev_ms: f64,
    /// Minimum observed duration, in milliseconds.
    pub min_ms: f64,
    /// Maximum observed duration, in milliseconds.
    pub max_ms: f64,
    /// 95th-percentile duration, in milliseconds.
    pub p95_ms: f64,
    /// 99th-percentile duration, in milliseconds.
    pub p99_ms: f64,
    /// Number of samples included in the statistics.
    pub sample_count: usize,
}

/// High-level categorization of a scrape failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// An HTTP-level error (4xx/5xx).
    HttpError,
    /// A network connectivity error.
    NetworkError,
    /// The request or page load timed out.
    Timeout,
    /// An internal framework or adapter error.
    FrameworkError,
    /// The request was blocked by bot-protection or a WAF.
    Blocked,
    /// The scrape succeeded but returned empty content.
    EmptyContent,
    /// No error occurred.
    #[default]
    None,
}

/// Whether the harness fetched pages from the live web or a local cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    /// Pages are fetched live from the network.
    Live,
    /// Pages are served from a pre-populated local HTML cache.
    Cached,
}

/// Complete benchmark result for one URL/fixture pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeBenchmarkResult {
    /// Name of the scraping framework under test.
    pub framework: String,
    /// URL that was scraped.
    pub url: String,
    /// Identifier of the originating [`ScrapeFixture`].
    pub fixture_id: String,
    /// Whether the scrape ultimately succeeded.
    pub success: bool,
    /// Human-readable error description, if the scrape failed.
    pub error_message: Option<String>,
    /// Structured error category.
    pub error_kind: ErrorKind,
    /// Total wall-clock duration of the benchmark (not warmup), in milliseconds.
    pub duration_ms: f64,
    /// System-level performance metrics.
    pub metrics: PerformanceMetrics,
    /// Quality metrics against the fixture ground truth, if measured.
    pub quality: Option<ScrapeQualityMetrics>,
    /// HTTP status code returned by the target server.
    pub status_code: Option<u16>,
    /// Whether a headless browser was used for this request.
    pub browser_used: bool,
    /// Whether the adapter detected that JavaScript rendering was required.
    pub js_render_hint: bool,
    /// Size of the extracted content in bytes.
    pub content_size: usize,
    /// Per-iteration timing records.
    pub iterations: Vec<IterationResult>,
    /// Aggregate statistics computed over all benchmark iterations.
    pub statistics: Option<DurationStatistics>,
    /// How pages were obtained during this run.
    pub execution_mode: ExecutionMode,
}

/// Aggregate quality summary over a full dataset run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetQualityReport {
    /// Fraction of fixtures for which quality was successfully measured.
    pub coverage: f64,
    /// Mean text F1 across all scored fixtures.
    pub mean_f1_text: f64,
    /// Mean numeric F1 across all scored fixtures.
    pub mean_f1_numeric: f64,
    /// Mean combined quality score across all scored fixtures.
    pub mean_quality_score: f64,
    /// Mean precision across all scored fixtures.
    pub mean_precision: f64,
    /// Mean recall across all scored fixtures.
    pub mean_recall: f64,
    /// Mean noise penalty across all scored fixtures.
    pub mean_noise_penalty: f64,
    /// Total number of fixture URLs in the dataset.
    pub total_urls: usize,
    /// Number of URLs that were scraped without error.
    pub successful_urls: usize,
    /// Number of URLs for which quality metrics were computed.
    pub scored_urls: usize,
}

/// Aggregate performance summary over a full dataset run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetPerformanceReport {
    /// Median end-to-end latency, in milliseconds.
    pub latency_p50_ms: f64,
    /// 95th-percentile latency, in milliseconds.
    pub latency_p95_ms: f64,
    /// 99th-percentile latency, in milliseconds.
    pub latency_p99_ms: f64,
    /// Aggregate throughput over the entire run, in pages per second.
    pub throughput_pages_per_sec: f64,
    /// Peak memory observed across all workers, in bytes.
    pub peak_memory_bytes: u64,
}

/// Top-level output document written at the end of a benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkOutput {
    /// Metadata describing the run configuration.
    pub metadata: BenchmarkMetadata,
    /// Quality summary, present only when `measure_quality` was enabled.
    pub quality_report: Option<DatasetQualityReport>,
    /// Performance summary for the full run.
    pub performance_report: DatasetPerformanceReport,
    /// Per-URL results.
    pub results: Vec<ScrapeBenchmarkResult>,
}

/// Comparison between two benchmark runs showing deltas.
///
/// All percentage deltas follow the sign convention: negative means the
/// candidate is *better* for latency and memory (less is better), while
/// positive means the candidate is *better* for throughput (more is better).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    /// Framework name of the baseline run.
    pub baseline: String,
    /// Framework name of the candidate run.
    pub candidate: String,
    /// Median latency delta: `(candidate_p50 - baseline_p50) / baseline_p50 * 100`.
    /// Negative values indicate the candidate is faster.
    pub latency_delta_pct: f64,
    /// Throughput delta: `(candidate - baseline) / baseline * 100`.
    /// Positive values indicate the candidate has higher throughput.
    pub throughput_delta_pct: f64,
    /// Mean quality-score delta (candidate minus baseline). `None` if neither
    /// run has quality metrics.
    pub quality_delta: Option<f64>,
    /// Peak-memory delta: `(candidate - baseline) / baseline * 100`.
    /// Negative values indicate the candidate uses less memory.
    pub memory_delta_pct: f64,
    /// Per-fixture comparison entries for fixtures present in both runs.
    pub fixture_comparisons: Vec<FixtureComparison>,
}

/// Comparison for a single fixture between baseline and candidate runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureComparison {
    /// Fixture identifier shared by both results.
    pub fixture_id: String,
    /// URL that was scraped.
    pub url: String,
    /// Median (or sole) duration of the baseline run for this fixture, in milliseconds.
    pub baseline_duration_ms: f64,
    /// Median (or sole) duration of the candidate run for this fixture, in milliseconds.
    pub candidate_duration_ms: f64,
    /// Per-fixture latency delta percentage.
    /// Negative values indicate the candidate is faster for this fixture.
    pub latency_delta_pct: f64,
    /// Quality score from the baseline run for this fixture, if available.
    pub baseline_quality: Option<f64>,
    /// Quality score from the candidate run for this fixture, if available.
    pub candidate_quality: Option<f64>,
    /// Quality delta (candidate minus baseline) for this fixture.
    /// `None` if either run lacks quality metrics for this fixture.
    pub quality_delta: Option<f64>,
}

/// Metadata recorded at the start of a benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetadata {
    /// ISO-8601 timestamp of when the run started.
    pub timestamp: String,
    /// Semver string of the benchmark harness binary.
    pub harness_version: String,
    /// Whether pages were fetched live or from cache.
    pub execution_mode: ExecutionMode,
    /// Dataset name or path used as input.
    pub dataset: String,
    /// Number of fixture entries loaded.
    pub fixture_count: usize,
    /// Name of the framework under test.
    pub framework: String,
    /// Number of benchmark iterations per URL.
    pub iterations: usize,
    /// Maximum number of concurrent scrape workers.
    pub max_concurrent: usize,
}
