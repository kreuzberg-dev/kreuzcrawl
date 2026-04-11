using System.Text.Json;
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
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_allow_all";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsAllowOverride()
    {
        // Allow directive overrides Disallow for specific paths
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_allow_override";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsCommentsHandling()
    {
        // Correctly parses robots.txt with inline and line comments
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true,\"user_agent\":\"kreuzcrawl\"}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_comments_handling";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsMetaNofollow()
    {
        // Detects nofollow meta robots tag and skips link extraction
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_meta_nofollow";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.NofollowDetected);
    }

    [Fact]
    public async Task Test_RobotsMetaNoindex()
    {
        // Detects noindex meta robots tag in HTML page
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_meta_noindex";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.NoindexDetected);
    }

    [Fact]
    public async Task Test_RobotsMissing404()
    {
        // Missing robots.txt (404) allows all crawling
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_missing_404";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsMultipleUserAgents()
    {
        // Picks the most specific user-agent block from robots.txt
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true,\"user_agent\":\"SpecificBot\"}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_multiple_user_agents";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsSitemapDirective()
    {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_sitemap_directive";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.IsAllowed);
    }

    [Fact]
    public async Task Test_RobotsXRobotsTag()
    {
        // Respects X-Robots-Tag HTTP header directives
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":true}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/robots_x_robots_tag";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal("noindex, nofollow", result.XRobotsTag.Trim());
        Assert.Equal(true, result.NoindexDetected);
        Assert.Equal(true, result.NofollowDetected);
    }
}
