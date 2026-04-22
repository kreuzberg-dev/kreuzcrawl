use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use kreuzcrawl::{
    BrowserConfig, BrowserMode, CrawlConfig, ProxyConfig, batch_crawl, crawl, create_engine, map_urls, scrape,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum CliBrowserMode {
    Auto,
    Always,
    Never,
}

impl From<CliBrowserMode> for BrowserMode {
    fn from(value: CliBrowserMode) -> Self {
        match value {
            CliBrowserMode::Auto => BrowserMode::Auto,
            CliBrowserMode::Always => BrowserMode::Always,
            CliBrowserMode::Never => BrowserMode::Never,
        }
    }
}

/// Validate that a `--browser-endpoint` value is a WebSocket URL (`ws://` or `wss://`).
fn parse_browser_endpoint(value: &str) -> Result<String, String> {
    if value.starts_with("ws://") || value.starts_with("wss://") {
        Ok(value.to_owned())
    } else {
        Err(format!(
            "browser endpoint must be a WebSocket URL starting with ws:// or wss://, got: {value:?}"
        ))
    }
}

fn build_browser_config(
    browser_mode: CliBrowserMode,
    browser_endpoint: Option<String>,
    timeout: Duration,
) -> BrowserConfig {
    BrowserConfig {
        mode: browser_mode.into(),
        endpoint: browser_endpoint,
        timeout,
        ..Default::default()
    }
}

#[derive(Parser)]
#[command(name = "kreuzcrawl", about = "High-performance web crawler and scraper", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scrape a single URL and extract metadata
    Scrape {
        /// URL to scrape
        url: String,
        /// Output format: json or markdown
        #[arg(long, default_value = "json")]
        format: String,
        /// Proxy URL
        #[arg(long)]
        proxy: Option<String>,
        /// Custom user agent
        #[arg(long)]
        user_agent: Option<String>,
        /// Request timeout in milliseconds
        #[arg(long, default_value = "30000")]
        timeout: u64,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
        /// When to use the browser: auto, always, or never
        #[arg(long, value_enum, default_value_t = CliBrowserMode::Auto)]
        browser_mode: CliBrowserMode,
        /// CDP WebSocket endpoint for an external browser (must start with ws:// or wss://)
        #[arg(long, value_parser = parse_browser_endpoint)]
        browser_endpoint: Option<String>,
    },
    /// Crawl a website following links
    Crawl {
        /// Seed URL(s) to crawl
        #[arg(required = true)]
        urls: Vec<String>,
        /// Maximum crawl depth
        #[arg(long, short = 'd', default_value = "2")]
        depth: usize,
        /// Maximum pages to crawl
        #[arg(long, short = 'n')]
        max_pages: Option<usize>,
        /// Maximum concurrent requests
        #[arg(long, short = 'c', default_value = "10")]
        concurrent: usize,
        /// Rate limit delay in milliseconds
        #[arg(long, default_value = "200")]
        rate_limit: u64,
        /// Output format: json or markdown
        #[arg(long, default_value = "json")]
        format: String,
        /// Proxy URL
        #[arg(long)]
        proxy: Option<String>,
        /// Custom user agent
        #[arg(long)]
        user_agent: Option<String>,
        /// Request timeout in milliseconds
        #[arg(long, default_value = "30000")]
        timeout: u64,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
        /// Stay on the same domain
        #[arg(long)]
        stay_on_domain: bool,
        /// When to use the browser: auto, always, or never
        #[arg(long, value_enum, default_value_t = CliBrowserMode::Auto)]
        browser_mode: CliBrowserMode,
        /// CDP WebSocket endpoint for an external browser (must start with ws:// or wss://)
        #[arg(long, value_parser = parse_browser_endpoint)]
        browser_endpoint: Option<String>,
    },
    /// Discover all URLs on a website via sitemaps and link extraction
    Map {
        /// URL to map
        url: String,
        /// Maximum URLs to return
        #[arg(long)]
        limit: Option<usize>,
        /// Filter URLs by substring
        #[arg(long)]
        search: Option<String>,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
    },
    /// Start the REST API server
    #[cfg(feature = "api")]
    Serve {
        /// Host address to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        /// Port to listen on
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    /// Start the MCP server (stdio transport)
    #[cfg(feature = "mcp")]
    Mcp {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scrape {
            url,
            format,
            proxy,
            user_agent,
            timeout,
            respect_robots_txt,
            browser_mode,
            browser_endpoint,
        } => {
            let timeout_duration = Duration::from_millis(timeout);
            let config = CrawlConfig {
                user_agent,
                request_timeout: timeout_duration,
                respect_robots_txt,
                proxy: proxy.map(|url| ProxyConfig {
                    url,
                    username: None,
                    password: None,
                }),
                browser: build_browser_config(browser_mode, browser_endpoint, timeout_duration),
                ..Default::default()
            };
            let handle = create_engine(Some(config)).expect("failed to create crawl engine");
            match scrape(&handle, &url).await {
                Ok(result) => {
                    if format == "markdown" {
                        if let Some(ref md) = result.markdown {
                            println!("{}", md.content);
                        } else {
                            eprintln!("No markdown content available");
                        }
                    } else {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&result).expect("result is serializable")
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Crawl {
            urls,
            depth,
            max_pages,
            concurrent,
            rate_limit,
            format,
            proxy,
            user_agent,
            timeout,
            respect_robots_txt,
            stay_on_domain,
            browser_mode,
            browser_endpoint,
        } => {
            let timeout_duration = Duration::from_millis(timeout);
            let config = CrawlConfig {
                max_depth: Some(depth),
                max_pages,
                max_concurrent: Some(concurrent),
                rate_limit_ms: Some(rate_limit),
                user_agent,
                request_timeout: timeout_duration,
                respect_robots_txt,
                stay_on_domain,
                proxy: proxy.map(|url| ProxyConfig {
                    url,
                    username: None,
                    password: None,
                }),
                browser: build_browser_config(browser_mode, browser_endpoint, timeout_duration),
                ..Default::default()
            };
            let handle = create_engine(Some(config)).expect("failed to create crawl engine");

            if urls.len() == 1 {
                match crawl(&handle, &urls[0]).await {
                    Ok(result) => {
                        if format == "markdown" {
                            for page in &result.pages {
                                if let Some(ref md) = page.markdown {
                                    println!("---\nURL: {}\n---\n{}\n", page.url, md.content);
                                }
                            }
                        } else {
                            println!(
                                "{}",
                                serde_json::to_string_pretty(&result).expect("result is serializable")
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                let results = batch_crawl(&handle, urls).await;
                if format == "markdown" {
                    for entry in &results {
                        if let Some(ref r) = entry.result {
                            for page in &r.pages {
                                if let Some(ref md) = page.markdown {
                                    println!("---\nSeed: {}\nURL: {}\n---\n{}\n", entry.url, page.url, md.content);
                                }
                            }
                        }
                        if let Some(ref e) = entry.error {
                            eprintln!("Error crawling {}: {e}", entry.url);
                        }
                    }
                } else {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(
                            &results
                                .iter()
                                .map(|entry| {
                                    serde_json::json!({
                                        "seed_url": entry.url,
                                        "result": match (&entry.result, &entry.error) {
                                            (Some(r), _) => serde_json::to_value(r).unwrap_or_default(),
                                            (_, Some(e)) => serde_json::json!({"error": e}),
                                            _ => serde_json::json!(null),
                                        }
                                    })
                                })
                                .collect::<Vec<_>>()
                        )
                        .expect("results are serializable")
                    );
                }
            }
        }
        Commands::Map {
            url,
            limit,
            search,
            respect_robots_txt,
        } => {
            let config = CrawlConfig {
                respect_robots_txt,
                map_limit: limit,
                map_search: search,
                ..Default::default()
            };
            let handle = create_engine(Some(config)).expect("failed to create crawl engine");
            match map_urls(&handle, &url).await {
                Ok(result) => {
                    for url_entry in &result.urls {
                        println!("{}", url_entry.url);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        #[cfg(feature = "api")]
        Commands::Serve { host, port } => {
            eprintln!("Starting REST API server on {host}:{port}");
            if let Err(e) = kreuzcrawl::serve_api(&host, port, CrawlConfig::default()).await {
                eprintln!("Server error: {e}");
                std::process::exit(1);
            }
        }
        #[cfg(feature = "mcp")]
        Commands::Mcp {} => {
            eprintln!("Starting MCP server (stdio transport)");
            if let Err(e) = kreuzcrawl::start_mcp_server().await {
                eprintln!("MCP server error: {e}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{CliBrowserMode, build_browser_config, parse_browser_endpoint};
    use kreuzcrawl::BrowserMode;

    const DEFAULT_TIMEOUT: Duration = Duration::from_millis(30_000);

    #[test]
    fn maps_cli_browser_mode_to_engine_mode() {
        assert_eq!(
            build_browser_config(CliBrowserMode::Auto, None, DEFAULT_TIMEOUT).mode,
            BrowserMode::Auto
        );
        assert_eq!(
            build_browser_config(CliBrowserMode::Always, None, DEFAULT_TIMEOUT).mode,
            BrowserMode::Always
        );
        assert_eq!(
            build_browser_config(CliBrowserMode::Never, None, DEFAULT_TIMEOUT).mode,
            BrowserMode::Never
        );
    }

    #[test]
    fn preserves_browser_endpoint() {
        let endpoint = Some("ws://127.0.0.1:9222/devtools/browser/test".to_string());
        let config = build_browser_config(CliBrowserMode::Auto, endpoint.clone(), DEFAULT_TIMEOUT);
        assert_eq!(config.endpoint, endpoint);
    }

    #[test]
    fn timeout_is_propagated_to_browser_config() {
        let timeout = Duration::from_millis(5_000);
        let config = build_browser_config(CliBrowserMode::Auto, None, timeout);
        assert_eq!(config.timeout, timeout);
    }

    #[test]
    fn parse_browser_endpoint_accepts_ws_urls() {
        assert!(parse_browser_endpoint("ws://127.0.0.1:9222/devtools/browser/abc").is_ok());
        assert!(parse_browser_endpoint("wss://remote.host/devtools/browser/abc").is_ok());
    }

    #[test]
    fn parse_browser_endpoint_rejects_non_ws_urls() {
        assert!(parse_browser_endpoint("http://127.0.0.1:9222").is_err());
        assert!(parse_browser_endpoint("https://remote.host").is_err());
        assert!(parse_browser_endpoint("127.0.0.1:9222").is_err());
    }
}
