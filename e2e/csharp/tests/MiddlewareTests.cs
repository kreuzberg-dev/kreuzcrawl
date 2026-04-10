using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: middleware.</summary>
public class MiddlewareTests
{
    [Fact]
    public async Task Test_MiddlewareEngineCrawlWithDefaults()
    {
        // Engine crawl with default middleware chain produces correct multi-page results
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    [Fact]
    public async Task Test_MiddlewareNoopNoEffect()
    {
        // Default middleware chain does not affect normal scraping
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("Middleware Test", result.Metadata.Title.Trim());
    }
}
