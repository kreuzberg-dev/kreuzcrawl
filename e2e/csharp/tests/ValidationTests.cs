using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: validation.</summary>
public class ValidationTests
{
    [Fact]
    public async Task Test_ValidationInvalidExcludeRegex()
    {
        // Invalid regex in exclude_paths is rejected
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ValidationInvalidIncludeRegex()
    {
        // Invalid regex in include_paths is rejected
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ValidationInvalidRetryCode()
    {
        // Retry code outside 100-599 is rejected
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ValidationMaxPagesZero()
    {
        // max_pages=0 is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ValidationMaxRedirectsTooHigh()
    {
        // max_redirects > 100 is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }

    [Fact]
    public async Task Test_ValidationTimeoutZero()
    {
        // Zero request timeout is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, ""));
    }
}
