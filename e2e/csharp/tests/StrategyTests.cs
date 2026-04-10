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
        Assert.Equal(3, result.Crawl.PagesCrawled);
        Assert.Contains("/", result.Strategy.FirstPageUrlContains);
    }

    [Fact]
    public async Task Test_StrategyBfsDefaultOrder()
    {
        // BFS strategy visits pages in breadth-first order
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(5, result.Crawl.PagesCrawled);
        Assert.Equal(new[] { "/", "/a", "/b", "/a/1", "/b/1" }, result.Strategy.CrawlOrder);
    }

    [Fact]
    public async Task Test_StrategyDfsDepthFirst()
    {
        // DFS strategy visits pages in depth-first order
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(5, result.Crawl.PagesCrawled);
        Assert.Equal(new[] { "/", "/b", "/b/1", "/a", "/a/1" }, result.Strategy.CrawlOrder);
    }
}
