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
        Assert.Equal(3, result.Crawl.PagesCrawled);
        Assert.True(result.Crawl.MinPages >= 3, "expected >= 3");
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
