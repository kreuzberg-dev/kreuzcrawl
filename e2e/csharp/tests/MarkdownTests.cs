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
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_basic_conversion";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("Test", result.Metadata.Title.Trim());
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown!.Content);
        Assert.Contains("Hello World", result.Markdown!.Content.ToString());
    }

    [Fact]
    public async Task Test_MarkdownCrawlAllPages()
    {
        // All crawled pages have markdown field populated
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_crawl_all_pages";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    [Fact]
    public async Task Test_MarkdownFitContent()
    {
        // Fit markdown removes navigation and boilerplate content
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_fit_content";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Markdown!.Content);
    }

    [Fact]
    public async Task Test_MarkdownHeadingsAndParagraphs()
    {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_headings_and_paragraphs";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.NotEmpty(result.Markdown!.Content);
        Assert.Contains("Main Title", result.Markdown!.Content.ToString());
    }

    [Fact]
    public async Task Test_MarkdownLinksConverted()
    {
        // HTML links are converted to markdown link syntax
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_links_converted";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown!.Content);
        Assert.Contains("Example", result.Markdown!.Content.ToString());
    }

    [Fact]
    public async Task Test_MarkdownWithCitations()
    {
        // Markdown includes citation conversion with numbered references
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/markdown_with_citations";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Markdown!.Content);
    }
}
