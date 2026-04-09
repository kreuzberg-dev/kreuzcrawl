# Errors Reference

All errors in kreuzcrawl are represented by the `CrawlError` enum, defined using `thiserror`. Each variant carries a `String` message with context about the failure.

## CrawlError Variants

| Variant | Description | Example Message |
|---------|-------------|-----------------|
| `NotFound(String)` | The requested page was not found (HTTP 404) | `"not_found: https://example.com/missing"` |
| `Unauthorized(String)` | The request was unauthorized (HTTP 401) | `"unauthorized: https://example.com/admin"` |
| `Forbidden(String)` | The request was forbidden (HTTP 403) | `"forbidden: https://example.com/private"` |
| `WafBlocked(String)` | Blocked by a WAF or bot protection (HTTP 403 with WAF indicators) | `"forbidden: waf/blocked: Cloudflare challenge"` |
| `Timeout(String)` | The request timed out | `"timeout: timeout: operation timed out"` |
| `RateLimited(String)` | Rate-limited by the target server (HTTP 429) | `"rate_limited: https://example.com"` |
| `ServerError(String)` | A server error occurred (HTTP 5xx) | `"server_error: 500 Internal Server Error"` |
| `BadGateway(String)` | A bad gateway error occurred (HTTP 502) | `"bad_gateway: 502 Bad Gateway"` |
| `Gone(String)` | The resource is permanently gone (HTTP 410) | `"gone: https://example.com/old-page"` |
| `Connection(String)` | A connection error occurred | `"connection: connection refused"` |
| `Dns(String)` | A DNS resolution error occurred | `"dns: dns: could not resolve host"` |
| `Ssl(String)` | An SSL/TLS error occurred | `"ssl: ssl: certificate expired"` |
| `DataLoss(String)` | Data was lost or truncated during transfer | `"data_loss: content-length mismatch"` |
| `BrowserError(String)` | The browser failed to launch, connect, or navigate | `"browser: failed to launch Chrome"` |
| `BrowserTimeout(String)` | The browser page load or rendering timed out | `"browser_timeout: page load exceeded 30s"` |
| `InvalidConfig(String)` | The provided configuration is invalid | `"invalid_config: max_concurrent must be > 0"` |
| `Other(String)` | An unclassified error | `"other: unexpected failure"` |

## HTTP Status Code Mapping

When `CrawlError` is returned through the REST API, it is mapped to an HTTP status code and machine-readable error code:

| CrawlError Variant | HTTP Status | Error Code |
|---------------------|-------------|------------|
| `NotFound` | `404 Not Found` | `NOT_FOUND` |
| `Unauthorized` | `401 Unauthorized` | `UNAUTHORIZED` |
| `Forbidden` | `403 Forbidden` | `FORBIDDEN` |
| `WafBlocked` | `403 Forbidden` | `WAF_BLOCKED` |
| `Timeout` | `504 Gateway Timeout` | `TIMEOUT` |
| `BrowserTimeout` | `504 Gateway Timeout` | `TIMEOUT` |
| `RateLimited` | `429 Too Many Requests` | `RATE_LIMITED` |
| `ServerError` | `502 Bad Gateway` | `SERVER_ERROR` |
| `BadGateway` | `502 Bad Gateway` | `BAD_GATEWAY` |
| `Gone` | `410 Gone` | `GONE` |
| `InvalidConfig` | `422 Unprocessable Entity` | `INVALID_CONFIG` |
| `Connection` | `500 Internal Server Error` | `CONNECTION_ERROR` |
| `Dns` | `500 Internal Server Error` | `DNS_ERROR` |
| `Ssl` | `500 Internal Server Error` | `SSL_ERROR` |
| `DataLoss` | `500 Internal Server Error` | `DATA_LOSS` |
| `BrowserError` | `500 Internal Server Error` | `BROWSER_ERROR` |
| `Other` | `500 Internal Server Error` | `SERVER_ERROR` |

## MCP Error Code Mapping

When errors occur in MCP tool calls, they are mapped to JSON-RPC error codes:

| CrawlError Variant | MCP Error Code | Meaning |
|---------------------|----------------|---------|
| `InvalidConfig` | `-32602` | Invalid params |
| All other variants | `-32603` | Internal error |

Error messages are preserved in the MCP error response to aid debugging.

## Request Error Classification

Network errors from `reqwest` are automatically classified by inspecting the full error source chain:

| Detection Pattern | Classified As |
|-------------------|---------------|
| `is_timeout()`, `"timed out"`, `"timeout"` | `Timeout` |
| `"dns"`, `"resolve"`, `"lookup"` | `Dns` |
| `"ssl"`, `"tls"`, `"certificate"`, `"handshake"` | `Ssl` |
| `is_connect()`, `"connection"`, `"connect"` | `Connection` |
| `is_body()`, `"content-length"`, `"truncat"` | `DataLoss` |
| Everything else | `Other` |

## Error Handling Patterns

### Rust

```rust
use kreuzcrawl::{CrawlEngine, CrawlError};

let engine = CrawlEngine::builder().build()?;

match engine.scrape("https://example.com").await {
    Ok(result) => println!("Title: {:?}", result.metadata.title),
    Err(CrawlError::NotFound(msg)) => eprintln!("Page not found: {msg}"),
    Err(CrawlError::Timeout(msg)) => eprintln!("Request timed out: {msg}"),
    Err(CrawlError::RateLimited(msg)) => {
        eprintln!("Rate limited: {msg}");
        // Implement backoff logic
    }
    Err(e) => eprintln!("Unexpected error: {e}"),
}
```

### Python

```python
import kreuzcrawl

engine = kreuzcrawl.create_engine()

try:
    result = await kreuzcrawl.scrape(engine, "https://example.com")
except RuntimeError as e:
    error_msg = str(e)
    if "not_found" in error_msg:
        print(f"Page not found: {error_msg}")
    elif "timeout" in error_msg:
        print(f"Timed out: {error_msg}")
    else:
        print(f"Error: {error_msg}")
```

### TypeScript

```typescript
import { createEngine, scrape } from "kreuzcrawl";

const engine = createEngine();

try {
  const result = await scrape(engine, "https://example.com");
} catch (error) {
  if (error.message.includes("[NotFound]")) {
    console.error("Page not found");
  } else if (error.message.includes("[Timeout]")) {
    console.error("Request timed out");
  } else {
    console.error("Error:", error.message);
  }
}
```

## REST API Error Response Format

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "NOT_FOUND",
    "message": "not_found: https://example.com/missing"
  }
}
```

The `code` field is a machine-readable string suitable for programmatic error handling. The `message` field is human-readable and includes context about the failure.
