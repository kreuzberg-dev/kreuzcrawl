using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: robots.</summary>
public class RobotsTests
{
    [Fact]
    public async Task Test_RobotsAllowAll()
    {
        // Permissive robots.txt allows all paths
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsAllowOverride()
    {
        // Allow directive overrides Disallow for specific paths
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsCommentsHandling()
    {
        // Correctly parses robots.txt with inline and line comments
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsCrawlDelay()
    {
        // Respects crawl-delay directive from robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(2, result.Robots.CrawlDelay);
    }

    [Fact]
    public async Task Test_RobotsDisallowPath()
    {
        // Robots.txt disallows specific paths
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(false, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsMetaNofollow()
    {
        // Detects nofollow meta robots tag and skips link extraction
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.NofollowDetected);
    }

    [Fact]
    public async Task Test_RobotsMetaNoindex()
    {
        // Detects noindex meta robots tag in HTML page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.NoindexDetected);
    }

    [Fact]
    public async Task Test_RobotsMissing404()
    {
        // Missing robots.txt (404) allows all crawling
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsMultipleUserAgents()
    {
        // Picks the most specific user-agent block from robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsRequestRate()
    {
        // Parses request-rate directive from robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(5, result.Robots.CrawlDelay);
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsSitemapDirective()
    {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsUserAgentSpecific()
    {
        // Matches user-agent specific rules in robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(false, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsWildcardPaths()
    {
        // Handles wildcard Disallow patterns in robots.txt
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(false, result.Robots.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsXRobotsTag()
    {
        // Respects X-Robots-Tag HTTP header directives
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal("noindex, nofollow", result.Robots.XRobotsTag.Trim());
        Assert.Equal(true, result.Robots.NoindexDetected);
        Assert.Equal(true, result.Robots.NofollowDetected);
    }
}
