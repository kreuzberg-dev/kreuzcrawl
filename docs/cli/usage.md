# CLI Usage

The `kreuzcrawl` CLI provides commands for scraping, crawling, site mapping, and running the API and MCP servers.

```text
kreuzcrawl <COMMAND> [OPTIONS]
```

## Commands

### scrape

Scrape a single URL and extract metadata.

```text
kreuzcrawl scrape <URL> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `URL` | Yes | URL to scrape |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--format <FORMAT>` | `json` | Output format: `json` or `markdown` |
| `--proxy <URL>` | -- | Proxy URL |
| `--user-agent <STRING>` | -- | Custom user agent |
| `--timeout <MS>` | `30000` | Request timeout in milliseconds |
| `--respect-robots-txt` | `false` | Respect robots.txt |

**Examples:**

```bash
# Scrape a page as JSON (default)
kreuzcrawl scrape https://example.com

# Scrape as markdown
kreuzcrawl scrape https://example.com --format markdown

# Scrape through a proxy with custom timeout
kreuzcrawl scrape https://example.com --proxy http://proxy:8080 --timeout 60000
```

**Output:**

- `json` format: Prints the full `ScrapeResult` as pretty-printed JSON to stdout.
- `markdown` format: Prints the markdown content to stdout. If no markdown content is available, prints an error to stderr.

---

### crawl

Crawl a website following links.

```text
kreuzcrawl crawl <URL>... [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `URL` | Yes (one or more) | Seed URL(s) to crawl. Multiple URLs triggers batch mode. |

**Options:**

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--depth <N>` | `-d` | `2` | Maximum crawl depth |
| `--max-pages <N>` | `-n` | -- | Maximum pages to crawl |
| `--concurrent <N>` | `-c` | `10` | Maximum concurrent requests |
| `--rate-limit <MS>` | -- | `200` | Rate limit delay between requests per domain (ms) |
| `--format <FORMAT>` | -- | `json` | Output format: `json` or `markdown` |
| `--proxy <URL>` | -- | -- | Proxy URL |
| `--user-agent <STRING>` | -- | -- | Custom user agent |
| `--timeout <MS>` | -- | `30000` | Request timeout in milliseconds |
| `--respect-robots-txt` | -- | `false` | Respect robots.txt |
| `--stay-on-domain` | -- | `false` | Stay on the same domain |

**Examples:**

```bash
# Crawl with default settings (depth 2, 10 concurrent)
kreuzcrawl crawl https://example.com

# Crawl deeper with more concurrency
kreuzcrawl crawl https://example.com -d 5 -c 20 --max-pages 500

# Crawl and output as markdown
kreuzcrawl crawl https://example.com --format markdown --stay-on-domain

# Batch crawl multiple seed URLs
kreuzcrawl crawl https://example.com https://example.org -d 1
```

**Output:**

- Single URL, `json` format: Prints the `CrawlResult` as pretty-printed JSON.
- Single URL, `markdown` format: Prints each page separated by `---` with URL header.
- Multiple URLs, `json` format: Prints an array of `{ seed_url, result }` objects.
- Multiple URLs, `markdown` format: Prints each page with seed URL and page URL headers.

---

### map

Discover all URLs on a website via sitemaps and link extraction.

```text
kreuzcrawl map <URL> [OPTIONS]
```

**Arguments:**

| Argument | Required | Description |
|----------|----------|-------------|
| `URL` | Yes | URL to map |

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--limit <N>` | -- | Maximum URLs to return |
| `--search <STRING>` | -- | Filter URLs by substring |
| `--respect-robots-txt` | `false` | Respect robots.txt |

**Examples:**

```bash
# Discover all URLs
kreuzcrawl map https://example.com

# Limit results and filter
kreuzcrawl map https://example.com --limit 50 --search "/docs/"
```

**Output:** Prints one URL per line to stdout.

---

### serve

Start the REST API server. Requires the `api` feature.

```text
kreuzcrawl serve [OPTIONS]
```

**Options:**

| Flag | Default | Description |
|------|---------|-------------|
| `--host <ADDRESS>` | `0.0.0.0` | Host address to bind to |
| `--port <PORT>` | `3000` | Port to listen on |

**Examples:**

```bash
# Start on default port
kreuzcrawl serve

# Start on custom host and port
kreuzcrawl serve --host 127.0.0.1 --port 8080
```

The server prints a startup message to stderr and runs until interrupted.

---

### mcp

Start the MCP server using stdio transport. Requires the `mcp` feature.

```text
kreuzcrawl mcp
```

No options. The server communicates via stdin/stdout using the MCP protocol. Startup messages are printed to stderr.

**Usage with Claude Desktop:**

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "kreuzcrawl": {
      "command": "kreuzcrawl",
      "args": ["mcp"]
    }
  }
}
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Error (scrape/crawl/map failure, server startup failure) |

Errors are printed to stderr. Successful output goes to stdout.
