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
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_MapExcludePatterns()
    {
        // Excludes URLs matching patterns from URL map
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_MapIncludeSubdomains()
    {
        // Includes subdomain URLs in URL map discovery
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'urls' not available on result type
    }

    [Fact]
    public async Task Test_MapLargeSitemap()
    {
        // Handles large sitemap with 100+ URLs
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_MapLimitPagination()
    {
        // Limits map result count to specified maximum
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_MapSearchFilter()
    {
        // Filters map results by search keyword
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'urls' not available on result type
    }
}
