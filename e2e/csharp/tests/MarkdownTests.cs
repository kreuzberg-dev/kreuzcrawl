using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: markdown.</summary>
public class MarkdownTests
{
    [Fact]
    public async Task Test_MarkdownBasicConversion()
    {
        // HTML is always converted to markdown alongside raw HTML
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("Test", result.Metadata.Title.Trim());
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Hello World", result.Markdown);
    }

    [Fact]
    public async Task Test_MarkdownCrawlAllPages()
    {
        // All crawled pages have markdown field populated
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    [Fact]
    public async Task Test_MarkdownFitContent()
    {
        // Fit markdown removes navigation and boilerplate content
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Markdown);
    }

    [Fact]
    public async Task Test_MarkdownHeadingsAndParagraphs()
    {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Main Title", result.Markdown);
    }

    [Fact]
    public async Task Test_MarkdownLinksConverted()
    {
        // HTML links are converted to markdown link syntax
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Example", result.Markdown);
    }

    [Fact]
    public async Task Test_MarkdownWithCitations()
    {
        // Markdown includes citation conversion with numbered references
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Markdown);
    }
}
