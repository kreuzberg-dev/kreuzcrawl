using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: auth.</summary>
public class AuthTests
{
    [Fact]
    public async Task Test_AuthBasicHttp()
    {
        // Sends HTTP Basic authentication header
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/auth_basic_http";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.AuthHeaderSent);
        Assert.Equal(200, result.StatusCode);
    }

    [Fact]
    public async Task Test_AuthBearerToken()
    {
        // Sends Bearer token in Authorization header
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/auth_bearer_token";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.AuthHeaderSent);
        Assert.Equal(200, result.StatusCode);
    }

    [Fact]
    public async Task Test_AuthCustomHeader()
    {
        // Sends authentication via custom header (X-API-Key)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/auth_custom_header";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(true, result.AuthHeaderSent);
        Assert.Equal(200, result.StatusCode);
    }
}
