using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: map.</summary>
public class MapTests
{
    [Fact]
    public async Task Test_MapDiscoverUrls()
    {
        // Discovers all URLs on a site without fetching full content
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Urls.Count >= 3, "expected >= 3");
    }

    [Fact]
    public async Task Test_MapExcludePatterns()
    {
        // Excludes URLs matching patterns from URL map
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(1, result.Urls.Count);
    }

    [Fact]
    public async Task Test_MapIncludeSubdomains()
    {
        // Includes subdomain URLs in URL map discovery
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Urls.Count >= 2, "expected >= 2");
        Assert.Contains("blog.example.com", result.Urls);
    }

    [Fact]
    public async Task Test_MapLargeSitemap()
    {
        // Handles large sitemap with 100+ URLs
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Urls.Count >= 100, "expected >= 100");
    }

    [Fact]
    public async Task Test_MapLimitPagination()
    {
        // Limits map result count to specified maximum
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Urls.Count <= 5, "expected <= 5");
    }

    [Fact]
    public async Task Test_MapSearchFilter()
    {
        // Filters map results by search keyword
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Urls.Count >= 2, "expected >= 2");
        Assert.Contains("blog", result.Urls);
    }
}
