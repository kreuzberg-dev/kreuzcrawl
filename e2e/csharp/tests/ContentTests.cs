using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: content.</summary>
public class ContentTests
{
    [Fact]
    public async Task Test_Content204NoContent()
    {
        // Handles 204 No Content response gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(204, result.StatusCode);
        Assert.Empty(result.Html);
    }

    [Fact]
    public async Task Test_ContentCharsetIso8859()
    {
        // Handles ISO-8859-1 encoded page correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal("iso-8859-1", result.DetectedCharset.Trim());
    }

    [Fact]
    public async Task Test_ContentEmptyBody()
    {
        // Handles 200 response with empty body gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
    }

    [Fact]
    public async Task Test_ContentGzipCompressed()
    {
        // Handles response with Accept-Encoding gzip negotiation
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
        Assert.Equal(200, result.StatusCode);
    }

    [Fact]
    public async Task Test_ContentLargePageLimit()
    {
        // Respects max body size limit and truncates or skips oversized pages
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.BodySize < 1025, "expected < 1025");
    }

    [Fact]
    public async Task Test_ContentMainOnly()
    {
        // Extracts only main content area, excluding nav, sidebar, footer
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.MainContentOnly);
    }

    [Fact]
    public async Task Test_ContentPdfNoExtension()
    {
        // Detects PDF content by Content-Type header when URL has no .pdf extension
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.IsPdf);
    }

    [Fact]
    public async Task Test_ContentRemoveTags()
    {
        // Removes specified HTML elements by CSS selector before processing
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public async Task Test_ContentUtf8Bom()
    {
        // Handles UTF-8 content with BOM marker correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal("utf-8", result.DetectedCharset.Trim());
        Assert.NotEmpty(result.Html);
    }
}
