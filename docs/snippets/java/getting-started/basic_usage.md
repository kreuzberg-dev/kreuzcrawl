```java
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;
import dev.kreuzberg.kreuzcrawl.ScrapeResult;

// Create engine with default settings
var engine = Kreuzcrawl.createEngine(null);

// Scrape a single page
ScrapeResult result = Kreuzcrawl.scrape(engine, "https://example.com");
System.out.println("Title: " + result.metadata().title());
System.out.println("Status: " + result.statusCode());
System.out.println("Links: " + result.links().size());
```
