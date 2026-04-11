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
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/middleware_engine_crawl_with_defaults";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    [Fact]
    public async Task Test_MiddlewareNoopNoEffect()
    {
        // Default middleware chain does not affect normal scraping
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/middleware_noop_no_effect";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("Middleware Test", result.Metadata.Title.Trim());
    }
}
