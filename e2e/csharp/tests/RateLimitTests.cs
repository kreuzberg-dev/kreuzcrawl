using System.Text.Json;
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
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_depth\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/rate_limit_basic_delay";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'rate_limit.min_duration_ms' not available on result type
    }

    [Fact]
    public async Task Test_RateLimitZeroNoDelay()
    {
        // Rate limiter with zero delay does not slow crawling
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_depth\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/rate_limit_zero_no_delay";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
    }
}
