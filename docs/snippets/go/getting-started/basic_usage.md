```go title="Go"
package main

import (
    "fmt"
    "log"

    kreuzcrawl "github.com/xberg-io/kreuzcrawl/packages/go"
)

func main() {
    // Simplest case: scrape a single page with default settings.
    engine, err := kreuzcrawl.CreateEngine(nil)
    if err != nil {
        log.Fatalf("create engine: %v", err)
    }

    result, err := kreuzcrawl.Scrape(engine, "https://example.com/")
    if err != nil {
        log.Fatalf("scrape: %v", err)
    }
    title := ""
    if result.Metadata.Title != nil {
        title = *result.Metadata.Title
    }
    fmt.Printf("Title: %s\n", title)
    fmt.Printf("Status: %d\n", result.StatusCode)
    fmt.Printf("Links found: %d\n", len(result.Links))

    // Crawl from a seed URL, limited to one hop and a handful of pages.
    config := kreuzcrawl.NewCrawlConfig(
        kreuzcrawl.WithCrawlConfigMaxDepth(1),
        kreuzcrawl.WithCrawlConfigMaxPages(5),
    )
    crawlEngine, err := kreuzcrawl.CreateEngine(config)
    if err != nil {
        log.Fatalf("create crawl engine: %v", err)
    }
    crawlResult, err := kreuzcrawl.Crawl(crawlEngine, "https://en.wikipedia.org/wiki/Web_scraping")
    if err != nil {
        log.Fatalf("crawl: %v", err)
    }
    fmt.Printf("Pages crawled: %d\n", len(crawlResult.Pages))
}
```
