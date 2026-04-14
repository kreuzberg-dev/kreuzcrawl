# Kreuzcrawl

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <!-- Language Bindings -->
  <a href="https://crates.io/crates/kreuzcrawl">
    <img src="https://img.shields.io/crates/v/kreuzcrawl?label=Rust&color=007ec6" alt="Rust">
  </a>
  <a href="https://pypi.org/project/kreuzcrawl/">
    <img src="https://img.shields.io/pypi/v/kreuzcrawl?label=Python&color=007ec6" alt="Python">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/kreuzcrawl">
    <img src="https://img.shields.io/npm/v/@kreuzberg/kreuzcrawl?label=Node.js&color=007ec6" alt="Node.js">
  </a>
  <a href="https://www.npmjs.com/package/@kreuzberg/kreuzcrawl-wasm">
    <img src="https://img.shields.io/npm/v/@kreuzberg/kreuzcrawl-wasm?label=WASM&color=007ec6" alt="WASM">
  </a>
  <a href="https://central.sonatype.com/artifact/dev.kreuzberg.kreuzcrawl/kreuzcrawl">
    <img src="https://img.shields.io/maven-central/v/dev.kreuzberg.kreuzcrawl/kreuzcrawl?label=Java&color=007ec6" alt="Java">
  </a>
  <a href="https://pkg.go.dev/github.com/kreuzberg-dev/kreuzcrawl/packages/go">
    <img src="https://img.shields.io/github/v/tag/kreuzberg-dev/kreuzcrawl?label=Go&color=007ec6" alt="Go">
  </a>
  <a href="https://www.nuget.org/packages/Kreuzcrawl/">
    <img src="https://img.shields.io/nuget/v/Kreuzcrawl?label=C%23&color=007ec6" alt="C#">
  </a>
  <a href="https://packagist.org/packages/kreuzberg-dev/kreuzcrawl">
    <img src="https://img.shields.io/packagist/v/kreuzberg-dev/kreuzcrawl?label=PHP&color=007ec6" alt="PHP">
  </a>
  <a href="https://rubygems.org/gems/kreuzcrawl">
    <img src="https://img.shields.io/gem/v/kreuzcrawl?label=Ruby&color=007ec6" alt="Ruby">
  </a>
  <a href="https://hex.pm/packages/kreuzcrawl">
    <img src="https://img.shields.io/hexpm/v/kreuzcrawl?label=Elixir&color=007ec6" alt="Elixir">
  </a>
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/releases">
    <img src="https://img.shields.io/badge/C-FFI-007ec6" alt="C">
  </a>

  <!-- Project Info -->
  <a href="https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Elastic--2.0-blue.svg" alt="License">
  </a>
  <a href="https://docs.kreuzcrawl.dev">
    <img src="https://img.shields.io/badge/docs-kreuzcrawl.dev-007ec6" alt="Documentation">
  </a>
</div>

<div align="center" style="margin-top: 20px;">
  <a href="https://discord.gg/xt9WY3GnKR">
      <img height="22" src="https://img.shields.io/badge/Discord-Join%20our%20community-7289da?logo=discord&logoColor=white" alt="Discord">
  </a>
</div>

High-performance Rust web crawling engine for structured data extraction. Scrape, crawl, and map websites with native bindings for 10 languages — same engine, identical results across every runtime.

## Key Features

- **Structured extraction** — Text, metadata, links, images, assets, JSON-LD, Open Graph, hreflang, favicons, headings, and response headers
- **Markdown conversion** — Clean Markdown output with citations, document structure, and fit-content mode
- **Concurrent crawling** — Depth-first, breadth-first, or best-first traversal with configurable depth, page limits, and concurrency
- **10 language bindings** — Rust, Python, Node.js, Ruby, Go, Java, C#, PHP, Elixir, and WebAssembly
- **Smart filtering** — BM25 relevance scoring, URL include/exclude patterns, robots.txt compliance, and sitemap discovery
- **Browser rendering** — Optional headless browser for JavaScript-heavy SPAs with WAF detection and bypass
- **Batch operations** — Scrape or crawl hundreds of URLs concurrently with partial failure handling
- **Streaming** — Real-time crawl events via async streams for progress tracking
- **Authentication** — HTTP Basic, Bearer token, and custom header auth with persistent cookie jars
- **Rate limiting** — Per-domain request throttling with configurable delays
- **Asset download** — Download, deduplicate, and filter images, documents, and other linked assets
- **MCP server** — Model Context Protocol integration for AI agents
- **REST API** — HTTP server with OpenAPI spec

**[Documentation](https://docs.kreuzcrawl.dev)** | **[API Reference](https://docs.kreuzcrawl.dev/reference/)**

## Installation

### Scripting Languages

| Language | Package | Install |
|----------|---------|---------|
| **[Python](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/python)** | [kreuzcrawl](https://pypi.org/project/kreuzcrawl/) | `pip install kreuzcrawl` |
| **[Ruby](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/ruby)** | [kreuzcrawl](https://rubygems.org/gems/kreuzcrawl) | `gem install kreuzcrawl` |
| **[PHP](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/php)** | [kreuzberg-dev/kreuzcrawl](https://packagist.org/packages/kreuzberg-dev/kreuzcrawl) | `composer require kreuzberg-dev/kreuzcrawl` |
| **[Elixir](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/elixir)** | [kreuzcrawl](https://hex.pm/packages/kreuzcrawl) | `{:kreuzcrawl, "~> 0.1"}` |

### JavaScript / TypeScript

| Package | Registry | Install |
|---------|----------|---------|
| **[Node.js](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/typescript)** — NAPI-RS native bindings | [npm](https://www.npmjs.com/package/@kreuzberg/kreuzcrawl) | `npm install @kreuzberg/kreuzcrawl` |
| **[WASM](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/wasm)** — browsers, Deno, Workers | [npm](https://www.npmjs.com/package/@kreuzberg/kreuzcrawl-wasm) | `npm install @kreuzberg/kreuzcrawl-wasm` |

### Compiled Languages

| Language | Package | Install |
|----------|---------|---------|
| **[Go](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/go)** | [pkg.go.dev](https://pkg.go.dev/github.com/kreuzberg-dev/kreuzcrawl/packages/go) | `go get github.com/kreuzberg-dev/kreuzcrawl/packages/go` |
| **[Java](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java)** | [Maven Central](https://central.sonatype.com/artifact/dev.kreuzberg.kreuzcrawl/kreuzcrawl) | See [package README](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/java) |
| **[C#](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/packages/csharp)** | [NuGet](https://www.nuget.org/packages/Kreuzcrawl/) | `dotnet add package Kreuzcrawl` |

### Native / System

| Package | Description |
|---------|-------------|
| **[Rust](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl)** | Core library — `cargo add kreuzcrawl` |
| **[C FFI](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-ffi)** | C header + shared library for any FFI-capable language |
| **[CLI](https://github.com/kreuzberg-dev/kreuzcrawl/tree/main/crates/kreuzcrawl-cli)** | Cross-platform binary — `cargo install kreuzcrawl-cli` |

## Quick Start

### Python

```python
from kreuzcrawl import create_engine, scrape

engine = create_engine()
result = scrape(engine, "https://example.com")

print(result.metadata.title)
print(result.markdown.content)
print(len(result.links))
```

### Node.js

```typescript
import { createEngine, scrape } from "@kreuzberg/kreuzcrawl";

const engine = createEngine();
const result = await scrape(engine, "https://example.com");

console.log(result.metadata.title);
console.log(result.markdown.content);
console.log(result.links.length);
```

### Rust

```rust
let engine = kreuzcrawl::create_engine(None)?;
let result = kreuzcrawl::scrape(&engine, "https://example.com").await?;

println!("{}", result.metadata.title);
println!("{}", result.markdown.content);
println!("{}", result.links.len());
```

### Go

```go
engine, _ := kcrawl.CreateEngine()
result, _ := kcrawl.Scrape(engine, "https://example.com")

fmt.Println(result.Metadata.Title)
fmt.Println(result.Markdown.Content)
fmt.Println(len(result.Links))
```

See each language's README for complete documentation, configuration options, and advanced examples.

## Platform Support

| Language | Linux x86_64 | Linux aarch64 | macOS ARM64 | Windows x64 |
|----------|:------------:|:-------------:|:-----------:|:-----------:|
| Python | ✅ | ✅ | ✅ | ✅ |
| Node.js | ✅ | ✅ | ✅ | ✅ |
| WASM | ✅ | ✅ | ✅ | ✅ |
| Ruby | ✅ | ✅ | ✅ | — |
| Elixir | ✅ | ✅ | ✅ | ✅ |
| Go | ✅ | ✅ | ✅ | ✅ |
| Java | ✅ | ✅ | ✅ | ✅ |
| C# | ✅ | ✅ | ✅ | ✅ |
| PHP | ✅ | ✅ | ✅ | ✅ |
| Rust | ✅ | ✅ | ✅ | ✅ |
| C (FFI) | ✅ | ✅ | ✅ | ✅ |
| CLI | ✅ | ✅ | ✅ | ✅ |

## Architecture

```text
Your Application (Python, Node.js, Ruby, Java, Go, C#, PHP, Elixir, ...)
    │
Language Bindings (PyO3, NAPI-RS, Magnus, ext-php-rs, Rustler, cgo, Panama, P/Invoke)
    │
Rust Core Engine (async, concurrent, SIMD-optimized)
    │
    ├── HTTP Client (reqwest + tower middleware stack)
    ├── HTML Parser (html5ever + lol_html)
    ├── Markdown Converter (html-to-markdown-rs)
    ├── Content Extraction (metadata, JSON-LD, Open Graph, readability)
    ├── Link Discovery (robots.txt, sitemaps, anchor analysis)
    └── Browser Rendering (optional headless Chrome/Firefox)
```

## Contributing

Contributions are welcome! See our [Contributing Guide](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/CONTRIBUTING.md).

## License

[Elastic License 2.0](https://github.com/kreuzberg-dev/kreuzcrawl/blob/main/LICENSE)

## Links

- [Documentation](https://docs.kreuzcrawl.dev)
- [API Reference](https://docs.kreuzcrawl.dev/reference/)
- [GitHub](https://github.com/kreuzberg-dev/kreuzcrawl)
- [Issues](https://github.com/kreuzberg-dev/kreuzcrawl/issues)
- [Discussions](https://github.com/kreuzberg-dev/kreuzcrawl/discussions)
- [Discord](https://discord.gg/xt9WY3GnKR)
