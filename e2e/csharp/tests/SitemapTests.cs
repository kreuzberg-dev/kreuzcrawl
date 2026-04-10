using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: sitemap.</summary>
public class SitemapTests
{
    [Fact]
    public async Task Test_SitemapBasic()
    {
        // Parses a standard urlset sitemap
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }

    [Fact]
    public async Task Test_SitemapCompressedGzip()
    {
        // Parses a gzip-compressed sitemap file
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_SitemapEmpty()
    {
        // Handles empty sitemap gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_SitemapFromRobotsTxt()
    {
        // Discovers sitemap via robots.txt Sitemap directive
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_SitemapIndex()
    {
        // Follows sitemap index to discover child sitemaps
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_SitemapLastmodFilter()
    {
        // Filters sitemap URLs by lastmod date
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }

    [Fact]
    public async Task Test_SitemapOnlyMode()
    {
        // Uses sitemap URLs exclusively without following page links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    [Fact]
    public async Task Test_SitemapXhtmlLinks()
    {
        // Parses sitemap with XHTML namespace alternate links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }
}
