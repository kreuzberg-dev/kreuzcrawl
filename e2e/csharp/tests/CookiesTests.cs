using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: cookies.</summary>
public class CookiesTests
{
    [Fact]
    public async Task Test_CookiesPerDomain()
    {
        // Isolates cookies per domain during crawl
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'cookies.length' not available on result type
        // skipped: field 'cookies' not available on result type
    }

    [Fact]
    public async Task Test_CookiesPersistence()
    {
        // Maintains cookies across multiple crawl requests
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'cookies' not available on result type
    }

    [Fact]
    public async Task Test_CookiesSetCookieResponse()
    {
        // Respects Set-Cookie header from server responses
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'cookies' not available on result type
    }
}
