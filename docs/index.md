---
title: Crawlberg
description: "Crawlberg - High-performance Rust web crawling engine with always-on HTML->Markdown, browser fallbacks, REST/MCP servers, and generated language packages."
---

## crawlberg

High-performance web crawling and scraping with a Rust core and generated language packages. Always-on HTML→Markdown conversion, structured metadata, Chromiumoxide and native browser backends, REST/MCP servers, a CLI, and a Docker image.

<div class="hero-badges" markdown>

[:material-lightning-bolt: Quick Start](getting-started/quickstart.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:material-feature-search-outline: Features](features.md){ .md-button }
[:fontawesome-brands-discord: Join our Community](https://discord.gg/xt9WY3GnKR){ .md-button }

</div>

---

### Why Crawlberg

<div class="grid cards" markdown>

- :material-spider-web:{ .lg .middle } **Flexible Crawling**

  BFS, DFS, BestFirst, and Adaptive traversal with concurrent fetching, streaming events, and batch crawl/scrape.

- :material-text-box-outline:{ .lg .middle } **Always-On Markdown** <span class="version-badge">v0.1</span>

  Every fetched page is converted to Markdown. `MarkdownResult.citations` is a boolean; call `generate_citations` when you need the reference list.

- :material-database-search:{ .lg .middle } **Rich Metadata**

  `PageMetadata` carries Open Graph, Twitter Card, Dublin Core, article fields, JSON-LD, links, images, feeds, favicons, and hreflang.

- :material-web:{ .lg .middle } **Browser Fallback** <span class="version-badge">v0.3</span>

  Use the Chromiumoxide CDP backend or the native browser backend. WAF signals can trigger browser escalation; the vendor is surfaced when the classifier can identify it.

- :material-server:{ .lg .middle } **MCP & REST Servers**

  Built-in MCP server for AI agents, REST API for service deployments, both gated behind cargo features.

- :material-translate:{ .lg .middle } **Rust + Generated Packages** <span class="version-badge">v0.3</span>

  Rust uses the core crate. Generated packages cover Python, TypeScript, WebAssembly, Go, Java, Kotlin Android, C#, Ruby, PHP, Elixir, Dart, Swift, Zig, and C FFI.

</div>

→ **[See all features](features.md)**

---

### Language Support

| Language              | Package                                                     | Docs                                             |
| :-------------------- | :---------------------------------------------------------- | :----------------------------------------------- |
| **Rust**              | `cargo add crawlberg`                                      | [API Reference](reference/api-rust.md)           |
| **Python**            | `pip install crawlberg`                                    | [API Reference](reference/api-python.md)         |
| **TypeScript / Node** | `npm install @kreuzberg/crawlberg`                         | [API Reference](reference/api-typescript.md)     |
| **WebAssembly**       | `npm install @kreuzberg/crawlberg-wasm`                    | [API Reference](reference/api-wasm.md)           |
| **Go**                | `go get github.com/xberg-io/crawlberg/packages/go`    | [API Reference](reference/api-go.md)             |
| **Java**              | Maven Central `dev.kreuzberg.crawlberg:crawlberg`         | [API Reference](reference/api-java.md)           |
| **Kotlin (Android)**  | Maven Central `dev.kreuzberg.crawlberg:crawlberg-android` | [API Reference](reference/api-kotlin-android.md) |
| **C#**                | `dotnet add package Crawlberg`                             | [API Reference](reference/api-csharp.md)         |
| **Ruby**              | `gem install crawlberg`                                    | [API Reference](reference/api-ruby.md)           |
| **PHP**               | `composer require xberg-io/crawlberg`                 | [API Reference](reference/api-php.md)            |
| **Elixir**            | `{:crawlberg, "~> 0.3.0"}`                                 | [API Reference](reference/api-elixir.md)         |
| **Dart / Flutter**    | `dart pub add crawlberg`                                   | [API Reference](reference/api-dart.md)           |
| **Swift**             | Swift Package Manager                                       | [API Reference](reference/api-swift.md)          |
| **Zig**               | `zig fetch --save` from GitHub                              | [API Reference](reference/api-zig.md)            |
| **C (FFI)**           | Shared library + header                                     | [API Reference](reference/api-c.md)              |
| **CLI**               | `cargo install crawlberg-cli`                              | [CLI Guide](cli/usage.md)                        |
| **Docker**            | `ghcr.io/xberg-io/crawlberg`                          | [Docker Guide](guides/docker.md)                 |

!!! tip "Choosing between TypeScript packages"

    **`@kreuzberg/crawlberg`** — Native NAPI-RS bindings. Use for Node.js servers and CLI tools. Full feature set including the browser fallback.

    **`@kreuzberg/crawlberg-wasm`** — Pure WebAssembly. Use for browsers, Cloudflare Workers, Deno, Bun, and serverless. No native browser backend, REST server, MCP server, or native streaming crawl wrappers.

---

### Quick Example

=== "Rust"

    ```rust title="src/main.rs"
    use crawlberg::{CrawlConfig, ContentConfig, create_engine, crawl};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let config = CrawlConfig {
            max_depth: Some(2),
            max_pages: Some(50),
            content: ContentConfig::default(),
            ..Default::default()
        };
        let engine = create_engine(Some(config))?;

        let result = crawl(&engine, "https://example.com").await?;
        for page in &result.pages {
            let title = page.metadata.title.as_deref().unwrap_or("(no title)");
            println!("{} — {}", page.url, title);
        }
        Ok(())
    }
    ```

=== "Python"

    ```python title="main.py"
    import asyncio
    from crawlberg import CrawlConfig, create_engine, crawl

    async def main():
        engine = create_engine(CrawlConfig(max_depth=2, max_pages=50))
        result = await crawl(engine, "https://example.com")
        for page in result.pages:
            print(f"{page.url} — {page.metadata.title or '(no title)'}")

    asyncio.run(main())
    ```

=== "TypeScript"

    ```typescript title="index.ts"
    import { createEngine, crawl } from "@kreuzberg/crawlberg";

    const engine = createEngine({ maxDepth: 2, maxPages: 50 });
    const result = await crawl(engine, "https://example.com");

    for (const page of result.pages) {
      console.log(`${page.url} — ${page.metadata.title ?? "(no title)"}`);
    }
    ```

---

### Part of Kreuzberg.dev

<div class="grid cards" markdown>

- :material-file-document-multiple:{ .lg .middle } **[Kreuzberg](https://github.com/xberg-io/kreuzberg)**

  Document intelligence: text, tables, metadata from 91+ formats with optional OCR.

- :material-cloud:{ .lg .middle } **[Xberg Enterprise](https://github.com/xberg-io/xberg-enterprise)**

  Managed extraction API with SDKs, dashboards, and observability.

- :material-spider-web:{ .lg .middle } **[crawlberg](https://github.com/xberg-io/crawlberg)**

  Web crawling and scraping with HTML→Markdown and headless-Chrome fallback.

- :material-language-html5:{ .lg .middle } **[html-to-markdown](https://github.com/xberg-io/html-to-markdown)**

  Fast, lossless HTML→Markdown engine.

- :material-robot-outline:{ .lg .middle } **[liter-llm](https://github.com/xberg-io/liter-llm)**

  Universal LLM API client with native bindings for 14 languages and 143 providers.

- :material-code-tags:{ .lg .middle } **[tree-sitter-language-pack](https://github.com/xberg-io/tree-sitter-language-pack)**

  Tree-sitter grammars and code-intelligence primitives.

- :material-alpha-a-circle:{ .lg .middle } **[alef](https://github.com/xberg-io/alef)**

  The polyglot binding generator that produces every per-language binding across the 5 polyglot repos.

</div>

---

### Explore the Docs

<div class="grid cards" markdown>

- :material-rocket-launch:{ .lg .middle } **Get Started**

  Install Crawlberg and run your first crawl in under five minutes.

  [:octicons-arrow-right-24: Quick Start](getting-started/quickstart.md)

- :material-book-open-variant:{ .lg .middle } **Guides**

  Crawling, scraping, URL discovery, browser automation, WARC output, and deployment.

  [:octicons-arrow-right-24: All Guides](guides/crawling.md)

- :material-puzzle-outline:{ .lg .middle } **Concepts**

  Public surface, data flow, the binding matrix, feature gates, and the content-extraction pipeline.

  [:octicons-arrow-right-24: Architecture](concepts/architecture.md)

- :material-api:{ .lg .middle } **Reference**

  Per-language API docs, the configuration schema, type catalogue, and error matrix.

  [:octicons-arrow-right-24: References](reference/api-rust.md)

- :material-console:{ .lg .middle } **CLI & Servers**

  The `crawlberg` CLI, REST API server, and MCP server for AI agents.

  [:octicons-arrow-right-24: CLI Usage](cli/usage.md)

- :material-feature-search-outline:{ .lg .middle } **Features**

  Complete feature breakdown: crawl strategies, metadata extraction, browser backends, WARC, MCP, REST.

  [:octicons-arrow-right-24: Features](features.md)

</div>

---

### Getting Help

- **Bugs & feature requests** — [Open an issue on GitHub](https://github.com/xberg-io/crawlberg/issues)
- **Community chat** — [Join the Discord](https://discord.gg/xt9WY3GnKR)
- **Contributing** — [Read the contributor guide](contributing.md)
