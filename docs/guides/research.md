# Deep Research

Kreuzcrawl includes an autonomous research agent that crawls the web and synthesizes
findings into a Markdown report. The research module is feature-gated behind `ai`.

## How it works

The research agent follows a **plan -- crawl -- evaluate -- synthesize** loop:

1. **Plan** -- The `ResearchPlanner` decides the next action: crawl a seed URL or
   synthesize the collected findings. Currently it iterates through seed URLs in order,
   then triggers synthesis. Future versions will use an LLM to plan dynamically based
   on what has been learned so far.

2. **Crawl** -- A `CrawlEngine` fetches the target URL and follows links up to the
   configured depth. Each page's markdown content is extracted and scored for relevance.

3. **Evaluate** -- Every extracted snippet is assigned a relevance score in `[0.0, 1.0]`
   using keyword matching against the research query. Words from the query are matched
   case-insensitively against the page content; the score is the fraction of query words
   found.

4. **Synthesize** -- The `ResearchSynthesizer` sorts all findings by relevance score
   (highest first) and assembles them into a Markdown report with a "Key Findings" section
   and a "Sources" section. Up to 20 top findings are included.

The loop runs for at most `max_steps` iterations, exiting early when synthesis is triggered.

## ResearchAgent setup

```rust
use kreuzcrawl::research::{ResearchAgent, ResearchConfig};
use kreuzcrawl::CrawlConfig;

let config = ResearchConfig {
    query: "Rust async runtime comparison".into(),
    max_steps: 10,
    max_pages_per_step: 5,
    max_depth: 3,
    seed_urls: vec![
        "https://tokio.rs".into(),
        "https://async-std.rs".into(),
    ],
};

let agent = ResearchAgent::new(config)
    .with_crawl_config(CrawlConfig {
        stay_on_domain: true,
        ..Default::default()
    });

let result = agent.research().await?;
println!("{}", result.synthesis);
```

## ResearchConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `String` | (required) | The research question or topic. |
| `max_steps` | `usize` | 10 | Maximum number of plan-crawl cycles before forcing synthesis. |
| `max_pages_per_step` | `usize` | 5 | Maximum pages the crawler visits per step. |
| `max_depth` | `usize` | 3 | Maximum link-hop depth for each crawl step. |
| `seed_urls` | `Vec<String>` | `[]` | Starting URLs. The planner crawls these in order. |

!!! tip "Choosing seed URLs"
    Provide 3--5 high-quality seed URLs that cover different facets of the topic.
    The agent currently does not discover new seed URLs on its own, so the seed list
    directly determines which sites are visited.

## ResearchResult structure

```rust
pub struct ResearchResult {
    /// The original research query.
    pub query: String,
    /// The synthesized Markdown report.
    pub synthesis: String,
    /// Individual findings extracted from crawled pages.
    pub findings: Vec<Finding>,
    /// Sources visited during the research.
    pub sources: Vec<SourceInfo>,
    /// The sequence of steps the agent took.
    pub steps: Vec<ResearchStep>,
    /// Total number of pages crawled across all steps.
    pub pages_crawled: usize,
    /// Optional LLM extraction metadata (cost, tokens, model).
    pub cost: Option<ExtractionMeta>,
}
```

### Finding

Each finding is a snippet of content extracted from a crawled page:

```rust
pub struct Finding {
    /// The extracted text (up to 500 characters).
    pub content: String,
    /// The URL the content was extracted from.
    pub source_url: String,
    /// Relevance score in [0.0, 1.0].
    pub relevance_score: f64,
}
```

### SourceInfo

```rust
pub struct SourceInfo {
    /// The page URL.
    pub url: String,
    /// The page title, if available.
    pub title: Option<String>,
    /// A short text snippet (up to 200 characters).
    pub snippet: Option<String>,
}
```

### ResearchStep

Each step records what the agent did:

```rust
pub struct ResearchStep {
    /// Zero-based step index.
    pub step_number: usize,
    /// The action taken (Crawl or Synthesize).
    pub action: StepAction,
    /// URLs visited during this step.
    pub urls_visited: Vec<String>,
    /// Number of findings extracted.
    pub findings_count: usize,
    /// Error message if the step failed.
    pub error: Option<String>,
}
```

Steps with errors are recorded but do not halt the research loop; the agent continues
to the next step.

## Relevance scoring

The current scoring algorithm is a simple keyword-match heuristic:

1. Split the query into whitespace-delimited words.
2. For each word, check if it appears (case-insensitive) anywhere in the page content.
3. The score is `matching_words / total_words`.

For example, the query `"rust async patterns"` against a page containing "Rust has many
useful patterns" would score `2/3 = 0.667` (matching "rust" and "patterns", missing "async").

!!! note "Future improvements"
    The keyword-match scorer is a placeholder. Future iterations will use LLM-based
    relevance evaluation and semantic similarity for more accurate ranking.

## Crawl configuration

The agent accepts a `CrawlConfig` via `with_crawl_config()` that is used as the base
configuration for every crawl step. Per-step overrides for `max_depth` and `max_pages`
are applied automatically from the `ResearchConfig`.

```rust
let agent = ResearchAgent::new(research_config)
    .with_crawl_config(CrawlConfig {
        user_agent: Some("ResearchBot/1.0".into()),
        request_timeout: std::time::Duration::from_secs(15),
        respect_robots_txt: true,
        ..Default::default()
    });
```
