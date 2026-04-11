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
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_invalid_exclude_regex";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ValidationInvalidIncludeRegex()
    {
        // Invalid regex in include_paths is rejected
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_invalid_include_regex";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ValidationInvalidRetryCode()
    {
        // Retry code outside 100-599 is rejected
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_invalid_retry_code";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ValidationMaxPagesZero()
    {
        // max_pages=0 is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_max_pages_zero";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ValidationMaxRedirectsTooHigh()
    {
        // max_redirects > 100 is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_max_redirects_too_high";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }

    [Fact]
    public async Task Test_ValidationTimeoutZero()
    {
        // Zero request timeout is rejected as invalid config
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/validation_timeout_zero";
        await Assert.ThrowsAsync<Exception>(() => KreuzcrawlLib.Scrape(engine, url));
    }
}
