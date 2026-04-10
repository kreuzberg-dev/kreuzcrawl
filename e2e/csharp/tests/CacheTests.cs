using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: cache.</summary>
public class CacheTests
{
    [Fact]
    public async Task Test_CacheBasic()
    {
        // Crawling with disk cache enabled succeeds without errors
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
    }
}
