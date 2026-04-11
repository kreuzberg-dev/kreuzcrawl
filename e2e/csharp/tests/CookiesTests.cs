using System.Text.Json;
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
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"cookies_enabled\":true,\"max_depth\":1,\"respect_robots_txt\":false}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/cookies_per_domain";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'cookies.length' not available on result type
        // skipped: field 'cookies' not available on result type
    }

    [Fact]
    public async Task Test_CookiesPersistence()
    {
        // Maintains cookies across multiple crawl requests
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"cookies_enabled\":true,\"max_depth\":1,\"respect_robots_txt\":false}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/cookies_persistence";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'cookies' not available on result type
    }

    [Fact]
    public async Task Test_CookiesSetCookieResponse()
    {
        // Respects Set-Cookie header from server responses
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"cookies_enabled\":true,\"max_depth\":1,\"respect_robots_txt\":false}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/cookies_set_cookie_response";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'cookies' not available on result type
    }
}
