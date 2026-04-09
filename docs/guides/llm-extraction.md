# LLM Extraction

Kreuzcrawl can extract structured data from crawled pages using LLM providers. The `LlmExtractor` integrates as a `ContentFilter`, processing each page through an LLM to produce typed JSON output with full cost tracking.

!!! note "Feature flag required"
    LLM extraction requires the `ai` feature flag: `kreuzcrawl = { version = "...", features = ["ai"] }`

## LlmExtractor setup

```rust
use kreuzcrawl::{CrawlEngine, CrawlConfig, LlmExtractor};
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "title": { "type": "string" },
        "summary": { "type": "string" },
        "topics": {
            "type": "array",
            "items": { "type": "string" }
        }
    },
    "required": ["title", "summary"]
});

let extractor = LlmExtractor::new(
    "your-api-key",                 // API key
    "openai/gpt-4o-mini",          // Model identifier
    Some(schema),                   // JSON schema (optional)
    Some("Extract the article title, a brief summary, and main topics.".into()), // Instruction (optional)
    None,                           // Custom prompt template (optional)
)?;

let engine = CrawlEngine::builder()
    .config(CrawlConfig {
        max_depth: Some(1),
        max_pages: Some(10),
        ..Default::default()
    })
    .content_filter(extractor)
    .build()?;

let result = engine.crawl("https://example.com/blog").await?;

for page in &result.pages {
    if let Some(ref data) = page.extracted_data {
        println!("{}: {}", page.url, serde_json::to_string_pretty(data)?);
    }
}
```

## Constructor parameters

| Parameter | Type | Required | Description |
|---|---|---|---|
| `api_key` | `&str` | Yes | API key for the LLM provider. |
| `model` | `&str` | Yes | Model identifier in `provider/model` format. |
| `schema` | `Option<Value>` | No | JSON Schema for structured extraction. When provided, the LLM is constrained to output conforming JSON. |
| `instruction` | `Option<String>` | No | Natural language instruction describing what to extract. |
| `prompt_template` | `Option<String>` | No | Custom Jinja2 template for the prompt. Overrides the default template when provided. |

## Multi-provider support

The `LlmExtractor` uses [liter-llm](https://github.com/kreuzberg-dev/liter-llm) for provider routing. Model identifiers follow the `provider/model` format:

| Provider | Example model | Notes |
|---|---|---|
| OpenAI | `openai/gpt-4o-mini` | Supports JSON schema response format with strict mode. |
| Anthropic | `anthropic/claude-sonnet-4-20250514` | |
| Google | `google/gemini-2.0-flash` | |
| Mistral | `mistral/mistral-large-latest` | |

The provider is inferred from the model string prefix. The API key must be valid for the specified provider.

## JSON schema extraction

When a JSON schema is provided, the extractor enables structured output mode:

```rust
let schema = json!({
    "type": "object",
    "properties": {
        "product_name": { "type": "string" },
        "price": { "type": "number" },
        "currency": { "type": "string" },
        "in_stock": { "type": "boolean" }
    },
    "required": ["product_name", "price"]
});

let extractor = LlmExtractor::new(
    api_key,
    "openai/gpt-4o-mini",
    Some(schema),
    Some("Extract product information from this page.".into()),
    None,
)?;
```

The schema is passed to the LLM as a `response_format` constraint with `strict: true`, ensuring the output conforms to the specified structure. If the LLM response cannot be parsed as JSON, it is wrapped in a `Value::String`.

## Custom prompt templates

The default prompt template is a Jinja2 template that includes the extraction instruction, JSON schema, and page content. You can override it with a custom template:

```rust
let custom_template = r#"You are analyzing a web page.

URL: {{ url }}
{% if title %}Page title: {{ title }}{% endif %}

{% if instruction %}
Task: {{ instruction }}
{% endif %}

{% if schema %}
Output JSON schema:
```json
{{ schema }}
```

{% endif %}

Page content:
{{ content }}"#;

let extractor = LlmExtractor::new(
    api_key,
    "openai/gpt-4o-mini",
    Some(schema),
    Some("Extract key data points.".into()),
    Some(custom_template.to_string()),
)?;

```text

### Template variables

| Variable | Type | Description |
|---|---|---|
| `content` | `String` | Page content (Markdown if available, otherwise HTML). Truncated to 100,000 characters. |
| `schema` | `Option<String>` | Pretty-printed JSON schema, if provided. |
| `instruction` | `Option<&str>` | The extraction instruction, if provided. |
| `url` | `&str` | The page URL. |
| `title` | `Option<&str>` | The page title from metadata, if available. |

## Cost tracking with ExtractionMeta

Every page processed by the `LlmExtractor` includes cost and usage metadata:

```rust
for page in &result.pages {
    if let Some(ref meta) = page.extraction_meta {
        println!("Model: {:?}", meta.model);
        println!("Cost: ${:.6}", meta.cost.unwrap_or(0.0));
        println!("Prompt tokens: {:?}", meta.prompt_tokens);
        println!("Completion tokens: {:?}", meta.completion_tokens);
        println!("Chunks processed: {}", meta.chunks_processed);
    }
}
```

### ExtractionMeta fields

| Field | Type | Description |
|---|---|---|
| `cost` | `Option<f64>` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `Option<u64>` | Number of input tokens consumed. |
| `completion_tokens` | `Option<u64>` | Number of output tokens generated. |
| `model` | `Option<String>` | The model identifier used for extraction. |
| `chunks_processed` | `usize` | Number of content chunks sent to the LLM (currently always 1). |

### Aggregating costs across a crawl

```rust
let total_cost: f64 = result.pages.iter()
    .filter_map(|p| p.extraction_meta.as_ref())
    .filter_map(|m| m.cost)
    .sum();

let total_tokens: u64 = result.pages.iter()
    .filter_map(|p| p.extraction_meta.as_ref())
    .filter_map(|m| m.prompt_tokens.zip(m.completion_tokens).map(|(p, c)| p + c))
    .sum();

println!("Total cost: ${:.4}", total_cost);
println!("Total tokens: {}", total_tokens);
```

## Content handling

The extractor uses the best available content representation:

1. **Markdown** (preferred) -- if the page has a `MarkdownResult`, the `content` field is used
2. **HTML** (fallback) -- if Markdown is not available, the raw HTML is used

Content is truncated to 100,000 characters to avoid exceeding LLM context windows. Truncation respects UTF-8 character boundaries.

## Integration as ContentFilter

The `LlmExtractor` implements the `ContentFilter` trait, which means it runs during the crawl pipeline after page extraction. This has several implications:

- Every crawled page passes through the LLM (subject to the filter returning `Some`)
- The extractor never drops pages -- it always returns `Some(page)` with the `extracted_data` and `extraction_meta` fields populated
- Pages filtered out by other means (path exclusion, robots.txt) never reach the extractor
- LLM calls are made concurrently, bounded by `max_concurrent`

!!! warning "Cost awareness"
    Each page in a crawl triggers an LLM API call. A crawl with `max_pages: Some(100)` using GPT-4o-mini will make 100 API calls. Monitor costs using the `ExtractionMeta` fields and set appropriate `max_pages` limits.

## Error handling

LLM errors (network failures, rate limits, invalid API keys) are propagated as `CrawlError::Other`. The error message includes context from the LLM client:

```rust
match engine.crawl("https://example.com").await {
    Ok(result) => { /* process pages */ }
    Err(e) => eprintln!("Crawl failed: {}", e),
}
```

Template rendering errors (invalid Jinja2 syntax in custom templates) also produce `CrawlError::Other` with a descriptive message.
