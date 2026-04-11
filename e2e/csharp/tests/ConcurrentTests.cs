using System.Text.Json;
using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: concurrent.</summary>
public class ConcurrentTests
{
    [Fact]
    public async Task Test_ConcurrentBasic()
    {
        // Concurrent crawling fetches all pages with max_concurrent workers
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":3,\"max_depth\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/concurrent_basic";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentDepthTwoFanOut()
    {
        // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":3,\"max_depth\":2}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/concurrent_depth_two_fan_out";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentMaxPagesExact()
    {
        // Concurrent crawling does not exceed max_pages limit even with high concurrency
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":5,\"max_depth\":1,\"max_pages\":3}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/concurrent_max_pages_exact";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentPartialErrors()
    {
        // Concurrent crawl handles partial failures gracefully
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":3,\"max_depth\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/concurrent_partial_errors";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentRespectsMaxPages()
    {
        // Concurrent crawling respects max_pages limit
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":2,\"max_depth\":1,\"max_pages\":3}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/concurrent_respects_max_pages";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }
}
