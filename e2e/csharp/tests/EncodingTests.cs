using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: encoding.</summary>
public class EncodingTests
{
    [Fact]
    public async Task Test_EncodingDoubleEncoded()
    {
        // Handles double-encoded URL characters (%25C3%25B6)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
        Assert.True(result.Links.Count >= 1, "expected >= 1");
    }

    [Fact]
    public async Task Test_EncodingMixedCharsetPage()
    {
        // Handles charset mismatch between HTTP header and HTML meta tag
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public async Task Test_EncodingPercentEncodedPath()
    {
        // Handles percent-encoded spaces and characters in URL paths
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
        Assert.True(result.Links.Count >= 2, "expected >= 2");
    }

    [Fact]
    public async Task Test_EncodingUnicodeUrl()
    {
        // Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
    }
}
