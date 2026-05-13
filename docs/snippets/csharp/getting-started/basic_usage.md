```csharp title="C#"
using System;
using System.Threading.Tasks;

using Kreuzcrawl;

internal static class BasicUsage
{
    public static async Task Main()
    {
        // Simplest case: scrape a single page with default settings.
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "https://example.com/");
        Console.WriteLine($"Title: {result.Metadata.Title}");
        Console.WriteLine($"Status: {result.StatusCode}");
        Console.WriteLine($"Links found: {result.Links.Count}");

        // Crawl from a seed URL, limited to one hop and a handful of pages.
        var config = new CrawlConfig
        {
            MaxDepth = 1,
            MaxPages = 5,
        };
        var crawlEngine = KreuzcrawlLib.CreateEngine(config);
        var crawlResult = await KreuzcrawlLib.Crawl(
            crawlEngine,
            "https://en.wikipedia.org/wiki/Web_scraping"
        );
        Console.WriteLine($"Pages crawled: {crawlResult.Pages.Count}");
    }
}
```
