# WARC Output

Kreuzcrawl can write crawled content to [WARC 1.1](https://iipc.github.io/warc-specifications/specifications/warc-format/warc-1.1/)
(Web ARChive) files for long-term archival and reproducibility. The WARC feature is gated
behind the `warc` Cargo feature.

## WARC 1.1 format overview

WARC is the standard format used by the Internet Archive and web preservation institutions.
Each WARC file contains a sequence of records, where each record has:

1. A version line (`WARC/1.1`)
2. Named-field headers (record type, date, record ID, content type, content length)
3. A payload block (the actual HTTP response)
4. A double-CRLF terminator

Kreuzcrawl writes two record types:

- **warcinfo** -- A single metadata record at the start of the file, identifying the
  software and hostname that produced the archive.
- **response** -- One record per crawled page, containing the full HTTP response
  (status line, headers, and body) as the payload.

Record IDs use the `<urn:uuid:...>` format. Timestamps follow ISO 8601 with second
precision and a `Z` suffix.

## WarcWriter API

The `WarcWriter` struct provides a low-level API for producing WARC files:

```rust
use std::path::Path;
use chrono::Utc;
use kreuzcrawl::WarcWriter;

let mut writer = WarcWriter::new(Path::new("output.warc"))?;

// Write the warcinfo record (call once, before any response records).
writer.write_warcinfo("kreuzcrawl/0.1.0", "my-host.example.com")?;

// Write a response record for each crawled page.
let record_id = writer.write_response(
    "https://example.com/page",
    200,
    &[("Content-Type", "text/html")],
    b"<html>Hello</html>",
    Utc::now(),
)?;

// Flush and finalize.
writer.finish()?;
```

The `write_response` method returns the WARC-Record-ID assigned to the record, which
callers can use for cross-referencing.

!!! note "Header validation"
    Header names and values are validated to reject CR/LF characters, preventing
    header injection attacks. Invalid headers cause `CrawlError::InvalidConfig`.

## Integration with crawl

The simplest way to enable WARC output is through the `warc_output` field on `CrawlConfig`:

```rust
use std::path::PathBuf;
use kreuzcrawl::CrawlConfig;

let config = CrawlConfig {
    warc_output: Some(PathBuf::from("crawl-2026-04-09.warc")),
    ..Default::default()
};
```

When `warc_output` is set, the crawl engine writes a warcinfo record at the start and
a response record for every successfully fetched page. The WARC file is flushed and
closed when the crawl completes.

## CLI flag

From the command line, pass `--warc-output` to any crawl command:

```bash
kreuzcrawl crawl https://example.com --depth 2 --warc-output archive.warc
```

The resulting file is a valid WARC 1.1 archive that can be processed by standard tools
such as [warcio](https://github.com/webrecorder/warcio), [pywb](https://github.com/webrecorder/pywb),
and the [Wayback Machine](https://web.archive.org/).

## HTTP response encoding

Each WARC response record wraps the full HTTP response block:

```text
HTTP/1.1 200 OK\r\n
Content-Type: text/html\r\n
\r\n
<html>...</html>
```

Kreuzcrawl maps common status codes to their standard reason phrases (200 OK, 404 Not Found,
etc.) and uses `"Unknown"` for unrecognized codes. The response block is stored as raw bytes
in the WARC payload, preserving the original content encoding.

!!! tip "File size"
    WARC files can grow large for deep crawls. Consider post-processing with gzip
    compression (`.warc.gz`) using external tools. Kreuzcrawl does not compress WARC
    output natively.
