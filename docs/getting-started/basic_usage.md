---
title: Basic Usage
description: Minimal scrape + crawl example for every crawlberg binding
---

## Basic Usage

A single end-to-end example showing the two core operations — **scrape** a page,
then **crawl** a small site — in every supported binding. Each tab is the canonical
"hello world" for that language: `create_engine`, `scrape(engine, url)`,
`CrawlConfig(max_depth=1, max_pages=5)`, `crawl(engine, url)`.

=== "Python"

    --8<-- "snippets/python/getting-started/basic_usage.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/basic_usage.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/basic_usage.md"

=== "Go"

    --8<-- "snippets/go/getting-started/basic_usage.md"

=== "Java"

    --8<-- "snippets/java/getting-started/basic_usage.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/basic_usage.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/basic_usage.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/basic_usage.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/basic_usage.md"

=== "Dart"

    --8<-- "snippets/dart/getting-started/basic_usage.md"

=== "Kotlin (Android)"

    --8<-- "snippets/kotlin_android/getting-started/basic_usage.md"

=== "Swift"

    --8<-- "snippets/swift/getting-started/basic_usage.md"

=== "Zig"

    --8<-- "snippets/zig/getting-started/basic_usage.md"

For deeper walkthroughs of scrape, crawl, and map operations — including
configuration options, link filtering, and result-shape details — see the
[Quick Start](quickstart.md) and the per-operation guides under **Guides → Core**.
