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
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_asset_dedup";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
        Assert.NotEmpty(result.Assets[0].ContentHash);
    }

    [Fact]
    public async Task Test_ScrapeAssetMaxSize()
    {
        // Skips assets exceeding max_asset_size limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_asset_max_size";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
    }

    [Fact]
    public async Task Test_ScrapeAssetTypeFilter()
    {
        // Only downloads image assets when asset_types filter is set
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_asset_type_filter";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(1, result.Assets.Count);
        Assert.Contains("image", result.Assets[0].AssetCategory.ToString());
    }

    [Fact]
    public async Task Test_ScrapeBasicHtmlPage()
    {
        // Scrapes a simple HTML page and extracts title, description, and links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_basic_html_page";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.NotEmpty(result.Html);
        Assert.Equal("Example Domain", result.Metadata.Title.Trim());
        Assert.Contains("illustrative examples", result.Metadata.Description.ToString());
        Assert.NotEmpty(result.Metadata.CanonicalUrl);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("external", result.Links[0].LinkType.ToString());
        Assert.Equal(0, result.Images.Count);
        Assert.Empty(result.Metadata.OgTitle);
    }

    [Fact]
    public async Task Test_ScrapeComplexLinks()
    {
        // Classifies links by type: internal, external, anchor, document, image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_complex_links";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > 9, "expected > 9");
        Assert.Contains("internal", result.Links[0].LinkType.ToString());
        Assert.Contains("external", result.Links[0].LinkType.ToString());
        Assert.Contains("anchor", result.Links[0].LinkType.ToString());
        Assert.Contains("document", result.Links[0].LinkType.ToString());
    }

    [Fact]
    public async Task Test_ScrapeDownloadAssets()
    {
        // Downloads CSS, JS, and image assets from page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_download_assets";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Assets.Count > 2, "expected > 2");
    }

    [Fact]
    public async Task Test_ScrapeDublinCore()
    {
        // Extracts Dublin Core metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_dublin_core";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Metadata.DcTitle);
        Assert.Equal("Effects of Climate Change on Marine Biodiversity", result.Metadata.DcTitle.Trim());
        Assert.Equal("Dr. Jane Smith", result.Metadata.DcCreator.Trim());
    }

    [Fact]
    public async Task Test_ScrapeEmptyPage()
    {
        // Handles an empty HTML document without errors
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_empty_page";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > -1, "expected > -1");
        Assert.Equal(0, result.Images.Count);
    }

    [Fact]
    public async Task Test_ScrapeFeedDiscovery()
    {
        // Discovers RSS, Atom, and JSON feed links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_feed_discovery";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Feeds.Count >= 3, "expected >= 3");
    }

    [Fact]
    public async Task Test_ScrapeImageSources()
    {
        // Extracts images from img, picture, og:image, twitter:image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_image_sources";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Images.Count > 4, "expected > 4");
        Assert.Equal("https://example.com/images/og-hero.jpg", result.Metadata.OgImage.Trim());
    }

    [Fact]
    public async Task Test_ScrapeJsHeavySpa()
    {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_js_heavy_spa";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public async Task Test_ScrapeJsonLd()
    {
        // Extracts JSON-LD structured data from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_json_ld";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.JsonLd);
        Assert.Equal("Recipe", result.JsonLd[0].SchemaType.Trim());
        Assert.Equal("Best Chocolate Cake", result.JsonLd[0].Name.Trim());
    }

    [Fact]
    public async Task Test_ScrapeMalformedHtml()
    {
        // Gracefully handles broken HTML without crashing
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_malformed_html";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Html);
        Assert.Contains("broken HTML", result.Metadata.Description.ToString());
    }

    [Fact]
    public async Task Test_ScrapeOgMetadata()
    {
        // Extracts full Open Graph metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_og_metadata";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Metadata.OgTitle);
        Assert.Equal("Article Title", result.Metadata.OgTitle.Trim());
        Assert.Equal("article", result.Metadata.OgType.Trim());
        Assert.Equal("https://example.com/images/article-hero.jpg", result.Metadata.OgImage.Trim());
        Assert.NotEmpty(result.Metadata.OgDescription);
        Assert.Equal("Article Title - Example Blog", result.Metadata.Title.Trim());
    }

    [Fact]
    public async Task Test_ScrapeTwitterCard()
    {
        // Extracts Twitter Card metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/scrape_twitter_card";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Metadata.TwitterCard);
        Assert.Equal("summary_large_image", result.Metadata.TwitterCard.Trim());
        Assert.Equal("New Product Launch", result.Metadata.TwitterTitle.Trim());
    }
}
