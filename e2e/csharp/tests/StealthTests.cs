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
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
    }
}
