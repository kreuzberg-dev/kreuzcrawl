using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: rate_limit.</summary>
public class RateLimitTests
{
    [Fact]
    public async Task Test_RateLimitBasicDelay()
    {
        // Rate limiter adds delay between requests to the same domain
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(3, result.Crawl.PagesCrawled);
        Assert.True(result.RateLimit.MinDurationMs >= 150, "expected >= 150");
    }

    [Fact]
    public async Task Test_RateLimitZeroNoDelay()
    {
        // Rate limiter with zero delay does not slow crawling
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(2, result.Crawl.PagesCrawled);
    }
}
