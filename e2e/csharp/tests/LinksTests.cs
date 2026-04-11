using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: links.</summary>
public class LinksTests
{
    [Fact]
    public async Task Test_LinksAnchorFragment()
    {
        // Identifies fragment-only links as anchor type
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_anchor_fragment";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Contains("anchor", result.Links[0].LinkType.ToString());
    }

    [Fact]
    public async Task Test_LinksBaseTag()
    {
        // Resolves relative URLs using base tag href
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_base_tag";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 2, "expected > 2");
        Assert.Contains("example.com", result.Links[0].Url.ToString());
    }

    [Fact]
    public async Task Test_LinksDocumentTypes()
    {
        // Detects PDF, DOCX, XLSX links as document type
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_document_types";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Contains("document", result.Links[0].LinkType.ToString());
    }

    [Fact]
    public async Task Test_LinksEmptyHref()
    {
        // Handles empty href attributes without errors
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_empty_href";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("/valid", result.Links[0].Url.ToString());
    }

    [Fact]
    public async Task Test_LinksInternalExternalClassification()
    {
        // Correctly classifies internal vs external links by domain
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_internal_external_classification";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 4, "expected > 4");
        Assert.NotEmpty(result.Links[0].Url);
    }

    [Fact]
    public async Task Test_LinksMailtoJavascriptSkip()
    {
        // Skips mailto:, javascript:, and tel: scheme links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_mailto_javascript_skip";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.DoesNotContain("mailto:", result.Links[0].Url.ToString());
    }

    [Fact]
    public async Task Test_LinksProtocolRelative()
    {
        // Handles protocol-relative URLs (//example.com) correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_protocol_relative";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 1, "expected > 1");
        Assert.Contains("//", result.Links[0].Url.ToString());
    }

    [Fact]
    public async Task Test_LinksRelAttributes()
    {
        // Preserves rel=nofollow and rel=canonical attributes
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_rel_attributes";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 0, "expected > 0");
    }

    [Fact]
    public async Task Test_LinksRelativeParent()
    {
        // Resolves ../ and ./ relative parent path links correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/links_relative_parent";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.True(result.Links.Count > 3, "expected > 3");
    }
}
