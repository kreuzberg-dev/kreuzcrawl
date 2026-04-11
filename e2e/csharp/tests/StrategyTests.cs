using System.Text.Json;
using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: strategy.</summary>
public class StrategyTests
{
    [Fact]
    public async Task Test_StrategyBestFirstSeed()
    {
        // BestFirst strategy always processes the seed URL first
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":1,\"max_depth\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/strategy_best_first_seed";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.first_page_url_contains' not available on result type
    }

    [Fact]
    public async Task Test_StrategyBfsDefaultOrder()
    {
        // BFS strategy visits pages in breadth-first order
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":1,\"max_depth\":2}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/strategy_bfs_default_order";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }

    [Fact]
    public async Task Test_StrategyDfsDepthFirst()
    {
        // DFS strategy visits pages in depth-first order
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"max_concurrent\":1,\"max_depth\":2}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/strategy_dfs_depth_first";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }
}
