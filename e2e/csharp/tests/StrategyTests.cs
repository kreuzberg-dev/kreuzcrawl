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
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.first_page_url_contains' not available on result type
    }

    [Fact]
    public async Task Test_StrategyBfsDefaultOrder()
    {
        // BFS strategy visits pages in breadth-first order
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }

    [Fact]
    public async Task Test_StrategyDfsDepthFirst()
    {
        // DFS strategy visits pages in depth-first order
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }
}
