---
title: Installation
description: Install Crawlberg for Rust, Python, TypeScript, Go, Java, Kotlin, C#, Ruby, PHP, Elixir, Dart, Swift, Zig, WebAssembly, or Docker
---

## Installation

<div class="cli-hero" markdown>

### CLI

The fastest way to get started. No Rust toolchain needed if you use Homebrew.

=== "Homebrew (macOS / Linux)"

    ```bash
    brew trust xberg-io/tap
    brew install xberg-io/tap/crawlberg
    ```

=== "Cargo"

    ```bash
    cargo install crawlberg-cli
    ```

    With optional features:

    ```bash
    cargo install crawlberg-cli --features "api,mcp"
    ```

Verify:

```bash
crawlberg --version
```

</div>

---

### Language Bindings

=== "Rust"

    Add to your `Cargo.toml`:

    ```toml title="Cargo.toml"
    [dependencies]
    crawlberg = "0.3"
    tokio = { version = "1", features = ["full"] }
    ```

    **Optional feature flags:**

    | Feature | Description |
    |---------|-------------|
    | `native-runtime` | Native OS runtime marker; enabled by default outside wasm32 |
    | `browser` | Chromiumoxide browser backend |
    | `browser-native` | In-process native browser backend |
    | `ai` | LLM extraction via liter-llm |
    | `telemetry-init` | One-call OpenTelemetry/OTLP setup |
    | `interact` | Compatibility alias for browser-backed page interaction |
    | `mcp` | Model Context Protocol server |
    | `api` | REST API server via Axum |
    | `mcp-http` | MCP over HTTP transport |
    | `warc` | WARC archive output |

    Enable features as needed:

    ```toml title="Cargo.toml"
    [dependencies]
    crawlberg = { version = "0.3", features = ["browser", "ai", "mcp"] }
    ```

    !!! note "Rust version"
        Crawlberg requires Rust 1.91+ (edition 2024).

=== "Python"

    Requires Python 3.10+. Install from PyPI:

    ```bash
    pip install crawlberg
    ```

    Or with `uv`:

    ```bash
    uv add crawlberg
    ```

    The package ships pre-built wheels with the Rust core compiled via [maturin](https://www.maturin.rs/). No Rust toolchain needed.

=== "TypeScript"

    Install the `@kreuzberg/crawlberg` package:

    ```bash
    npm install @kreuzberg/crawlberg
    ```

    Or with pnpm:

    ```bash
    pnpm add @kreuzberg/crawlberg
    ```

    The package includes pre-built native binaries via [NAPI-RS](https://napi.rs/) and ships with TypeScript type definitions (`.d.ts`).

=== "Ruby"

    Requires Ruby 3.2+. Add to your `Gemfile`:

    ```ruby title="Gemfile"
    gem "crawlberg", "~> 0.3"
    ```

    Then:

    ```bash
    bundle install
    ```

    Or install directly:

    ```bash
    gem install crawlberg
    ```

    The gem includes a native extension built with [Magnus](https://github.com/matsadler/magnus) and [rb_sys](https://github.com/oxidize-rb/rb-sys).

=== "Go"

    Requires Go 1.21+. The Go bindings use cgo with the C FFI layer:

    ```bash
    go get github.com/xberg-io/crawlberg/packages/go
    ```

    !!! warning "Build requirement"
        A C compiler is required. The crawlberg shared library must be available at link time.

=== "Java"

    Requires Java 25+ (Panama FFM). Add the Maven dependency:

    ```xml title="pom.xml"
    <dependency>
        <groupId>dev.kreuzberg.crawlberg</groupId>
        <artifactId>crawlberg</artifactId>
        <version>0.3.0</version>
    </dependency>
    ```

    The Java bindings use the Panama Foreign Function & Memory API to call the C FFI layer.

=== "Kotlin (Android)"

    Add the Android AAR dependency:

    ```kotlin title="build.gradle.kts"
    dependencies {
        implementation("dev.kreuzberg.crawlberg:crawlberg-android:0.3.0")
    }
    ```

    The Kotlin package targets Android and bundles JNI shared libraries. Kotlin/JVM server applications should use the Java package.

=== "C#"

    Requires .NET 10+. Add the NuGet package:

    ```bash
    dotnet add package Crawlberg
    ```

    Or in your `.csproj`:

    ```xml title="Crawlberg.csproj"
    <PackageReference Include="Crawlberg" Version="0.3.0" />
    ```

    The C# bindings use P/Invoke to call the C FFI layer.

=== "PHP"

    Requires PHP 8.2+. Install the extension via Composer:

    ```bash
    composer require xberg-io/crawlberg
    ```

    The PHP bindings are built with [ext-php-rs](https://github.com/davidcole1340/ext-php-rs) and load as a native PHP extension.

    !!! tip "Verify the extension is loaded"
        ```bash
        php -m | grep crawlberg
        ```

=== "Elixir"

    Requires Elixir 1.14+ with OTP 25+. Add to your `mix.exs`:

    ```elixir title="mix.exs"
    defp deps do
      [
        {:crawlberg, "~> 0.3"}
      ]
    end
    ```

    Then:

    ```bash
    mix deps.get
    mix compile
    ```

    The Elixir bindings use [Rustler](https://github.com/rusterlium/rustler) NIFs.

=== "WebAssembly"

    For browser or Node.js environments via wasm-bindgen:

    ```bash
    npm install @kreuzberg/crawlberg-wasm
    ```

    ```typescript
    import init from "@kreuzberg/crawlberg-wasm";
    await init();
    ```

    !!! note "Limitations"
        The WASM build runs without native browser backends, REST/MCP server starters, WARC file output, native streaming crawl wrappers, and other native-only I/O surfaces.

=== "Dart"

    Requires Dart or Flutter with native FFI support:

    ```bash
    dart pub add crawlberg
    ```

=== "Swift"

    Add the Swift package from GitHub:

    ```swift
    .package(url: "https://github.com/xberg-io/crawlberg", exact: "0.3.0")
    ```

=== "Zig"

    Use the generated Zig package over the C FFI layer:

    ```bash
    zig fetch --save https://github.com/xberg-io/crawlberg/archive/refs/tags/v0.3.0.tar.gz
    ```

=== "C FFI"

    For languages not listed above, crawlberg provides a C-compatible FFI layer. Build the shared library:

    ```bash
    cargo build --release -p crawlberg-ffi
    ```

    The output is a shared library (`libcrawlberg.so` / `libcrawlberg.dylib` / `crawlberg.dll`) with C headers generated by `cbindgen`.

---

### Docker

Pull the official image:

```bash
docker pull ghcr.io/xberg-io/crawlberg:latest
```

Run the CLI:

```bash
docker run --rm ghcr.io/xberg-io/crawlberg:latest scrape https://example.com
```

Run with a volume for WARC output:

```bash
docker run --rm -v $(pwd)/output:/output \
  ghcr.io/xberg-io/crawlberg:latest \
  crawl https://example.com --depth 2 --warc-output /output/archive.warc
```

---

### Where to go next

- **[Quick start](quickstart.md)** — Scrape a page, run a crawl, and map a site in under five minutes. Covers the CLI and the Rust API side by side.
- **[Configuration guide](../guides/configuration.md)** — Every `CrawlConfig` field with its default and validation rules. Start here if you need to tune depth, concurrency, or content filtering.
- **[Features overview](../features.md)** — What the engine can do: browser rendering, LLM extraction, REST API, MCP, WARC output, and more. Useful for figuring out which feature flags you need.
