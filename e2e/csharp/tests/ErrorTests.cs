using System.Text.Json;
using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: error.</summary>
public class ErrorTests
{
    [Fact]
    public async Task Test_Error401Unauthorized()
    {
        // Handles 401 Unauthorized response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_401_unauthorized";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error403Forbidden()
    {
        // Handles 403 Forbidden response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_403_forbidden";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error404Page()
    {
        // Handles 404 response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_404_page";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error408RequestTimeout()
    {
        // Handles 408 Request Timeout response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_408_request_timeout";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error410Gone()
    {
        // Handles 410 Gone response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_410_gone";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error500Server()
    {
        // Handles 500 server error
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_500_server";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_Error502BadGateway()
    {
        // Handles 502 Bad Gateway response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_502_bad_gateway";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorEmptyResponse()
    {
        // Handles 200 with completely empty body gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_empty_response";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'html_not_empty' not available on result type
        // skipped: field 'error.is_error' not available on result type
    }

    [Fact]
    public async Task Test_ErrorInvalidProxy()
    {
        // Proxy pointing to unreachable address causes connection error during scrape
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"proxy\":{\"url\":\"http://127.0.0.1:1\"},\"request_timeout\":2000}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_invalid_proxy";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorPartialResponse()
    {
        // Handles incomplete or truncated HTTP response
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":false}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_partial_response";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorRateLimited()
    {
        // Handles 429 rate limiting with Retry-After
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_rate_limited";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorRetry503()
    {
        // Retries request on 503 Service Unavailable response
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":false,\"retry_codes\":[503],\"retry_count\":2}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_retry_503";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorRetryBackoff()
    {
        // Implements exponential backoff when retrying failed requests
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"respect_robots_txt\":false,\"retry_codes\":[429],\"retry_count\":3}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_retry_backoff";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorTimeout()
    {
        // Handles request timeout
        var engineConfig = JsonSerializer.Deserialize<CrawlConfig>("{\"request_timeout\":1}")!;
        var engine = KreuzcrawlLib.CreateEngine(engineConfig);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_timeout";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorWafAkamai()
    {
        // Akamai WAF detection returns WafBlocked error
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_waf_akamai";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorWafFalse403()
    {
        // Detects WAF/bot protection false 403 (Cloudflare challenge page)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_waf_false_403";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ErrorWafImperva()
    {
        // Imperva/Incapsula WAF detection
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/error_waf_imperva";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }
}
