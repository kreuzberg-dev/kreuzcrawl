using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: scrape.</summary>
public class ScrapeTests
{
    [Fact]
    public async Task Test_ScrapeAssetDedup()
    {
        // Same asset linked twice results in one download with one unique hash
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
        Assert.Equal(2, result.Assets.UniqueHashes);
    }

    [Fact]
    public async Task Test_ScrapeAssetMaxSize()
    {
        // Skips assets exceeding max_asset_size limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
    }

    [Fact]
    public async Task Test_ScrapeAssetTypeFilter()
    {
        // Only downloads image assets when asset_types filter is set
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(1, result.Assets.Count);
        Assert.Contains("image", result.Assets[""].Category);
    }

    [Fact]
    public async Task Test_ScrapeBasicHtmlPage()
    {
        // Scrapes a simple HTML page and extracts title, description, and links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.NotEmpty(result.Html);
        Assert.Equal("Example Domain", result.Metadata.Title.Trim());
        Assert.Contains("illustrative examples", result.Metadata.Description);
        Assert.NotEmpty(result.Metadata.CanonicalUrl);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("external", result.Links[""].LinkType);
        Assert.Equal(0, result.Images.Count);
        Assert.Empty(result.Og.Title);
    }

    [Fact]
    public async Task Test_ScrapeComplexLinks()
    {
        // Classifies links by type: internal, external, anchor, document, image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > 9, "expected > 9");
        Assert.Contains("internal", result.Links[""].LinkType);
        Assert.Contains("external", result.Links[""].LinkType);
        Assert.Contains("anchor", result.Links[""].LinkType);
        Assert.Contains("document", result.Links[""].LinkType);
    }

    [Fact]
    public async Task Test_ScrapeDownloadAssets()
    {
        // Downloads CSS, JS, and image assets from page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Assets.Count > 2, "expected > 2");
    }

    [Fact]
    public async Task Test_ScrapeDublinCore()
    {
        // Extracts Dublin Core metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.DublinCore.Title);
        Assert.Equal("Effects of Climate Change on Marine Biodiversity", result.DublinCore.Title.Trim());
        Assert.Equal("Dr. Jane Smith", result.DublinCore.Creator.Trim());
    }

    [Fact]
    public async Task Test_ScrapeEmptyPage()
    {
        // Handles an empty HTML document without errors
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > -1, "expected > -1");
        Assert.Equal(0, result.Images.Count);
    }

    [Fact]
    public async Task Test_ScrapeFeedDiscovery()
    {
        // Discovers RSS, Atom, and JSON feed links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(1, result.Feeds.Rss.Count);
        Assert.Equal(1, result.Feeds.Atom.Count);
        Assert.Equal(1, result.Feeds.JsonFeed.Count);
    }

    [Fact]
    public async Task Test_ScrapeImageSources()
    {
        // Extracts images from img, picture, og:image, twitter:image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Images.Count > 4, "expected > 4");
        Assert.Equal("https://example.com/images/og-hero.jpg", result.Og.Image.Trim());
    }

    [Fact]
    public async Task Test_ScrapeJsHeavySpa()
    {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public async Task Test_ScrapeJsonLd()
    {
        // Extracts JSON-LD structured data from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.JsonLd);
        Assert.Equal("Recipe", result.JsonLd.Type.Trim());
        Assert.Equal("Best Chocolate Cake", result.JsonLd.Name.Trim());
    }

    [Fact]
    public async Task Test_ScrapeMalformedHtml()
    {
        // Gracefully handles broken HTML without crashing
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Html);
        Assert.Contains("broken HTML", result.Metadata.Description);
    }

    [Fact]
    public async Task Test_ScrapeOgMetadata()
    {
        // Extracts full Open Graph metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Og.Title);
        Assert.Equal("Article Title", result.Og.Title.Trim());
        Assert.Equal("article", result.Og.Type.Trim());
        Assert.Equal("https://example.com/images/article-hero.jpg", result.Og.Image.Trim());
        Assert.NotEmpty(result.Og.Description);
        Assert.Equal("Article Title - Example Blog", result.Metadata.Title.Trim());
    }

    [Fact]
    public async Task Test_ScrapeTwitterCard()
    {
        // Extracts Twitter Card metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Twitter.Card);
        Assert.Equal("summary_large_image", result.Twitter.CardType.Trim());
        Assert.Equal("New Product Launch", result.Twitter.Title.Trim());
    }
}
