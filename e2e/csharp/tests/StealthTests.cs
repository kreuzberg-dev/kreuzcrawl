using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: stealth.</summary>
public class StealthTests
{
    [Fact]
    public async Task Test_StealthUaRotationConfig()
    {
        // User-agent rotation config is accepted and crawl succeeds
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/stealth_ua_rotation_config";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        Assert.Equal(200, result.StatusCode);
    }
}
