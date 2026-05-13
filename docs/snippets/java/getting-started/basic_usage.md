```java title="Java"
import java.util.Optional;

import dev.kreuzberg.kreuzcrawl.CrawlConfig;
import dev.kreuzberg.kreuzcrawl.CrawlEngineHandle;
import dev.kreuzberg.kreuzcrawl.CrawlResult;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;
import dev.kreuzberg.kreuzcrawl.ScrapeResult;

public final class BasicUsage {
    private BasicUsage() { }

    public static void main(final String[] args) throws Exception {
        // Simplest case: scrape a single page with default settings.
        CrawlEngineHandle engine = Kreuzcrawl.createEngine();
        ScrapeResult result = Kreuzcrawl.scrape(engine, "https://example.com/");
        System.out.println("Title: " + result.metadata().title());
        System.out.println("Status: " + result.statusCode());
        System.out.println("Links found: " + result.links().size());

        // Crawl from a seed URL, limited to one hop and a handful of pages.
        CrawlConfig config = CrawlConfig.builder()
            .withMaxDepth(Optional.of(1L))
            .withMaxPages(Optional.of(5L))
            .build();
        CrawlEngineHandle crawlEngine = Kreuzcrawl.createEngine(config);
        CrawlResult crawlResult = Kreuzcrawl.crawl(
            crawlEngine,
            "https://en.wikipedia.org/wiki/Web_scraping"
        );
        System.out.println("Pages crawled: " + crawlResult.pages().size());
    }
}
```
