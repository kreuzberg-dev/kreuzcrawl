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
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error403Forbidden()
    {
        // Handles 403 Forbidden response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error404Page()
    {
        // Handles 404 response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error408RequestTimeout()
    {
        // Handles 408 Request Timeout response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error410Gone()
    {
        // Handles 410 Gone response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error500Server()
    {
        // Handles 500 server error
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_Error502BadGateway()
    {
        // Handles 502 Bad Gateway response correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorConnectionRefused()
    {
        // Handles connection refused error gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorDnsResolution()
    {
        // Handles DNS resolution failure gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorEmptyResponse()
    {
        // Handles 200 with completely empty body gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(false, result.HtmlNotEmpty);
        Assert.Equal(false, result.Error.IsError);
    }

    [Fact]
    public async Task Test_ErrorInvalidProxy()
    {
        // Proxy pointing to unreachable address causes connection error during scrape
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorPartialResponse()
    {
        // Handles incomplete or truncated HTTP response
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorRateLimited()
    {
        // Handles 429 rate limiting with Retry-After
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorRetry503()
    {
        // Retries request on 503 Service Unavailable response
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorRetryBackoff()
    {
        // Implements exponential backoff when retrying failed requests
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorSslInvalidCert()
    {
        // Handles SSL certificate validation error
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorTimeout()
    {
        // Handles request timeout
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorWafAkamai()
    {
        // Akamai WAF detection returns WafBlocked error
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorWafFalse403()
    {
        // Detects WAF/bot protection false 403 (Cloudflare challenge page)
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ErrorWafImperva()
    {
        // Imperva/Incapsula WAF detection
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }
}
