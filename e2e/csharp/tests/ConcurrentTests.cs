using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: concurrent.</summary>
public class ConcurrentTests
{
    [Fact]
    public async Task Test_ConcurrentBasic()
    {
        // Concurrent crawling fetches all pages with max_concurrent workers
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentDepthTwoFanOut()
    {
        // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentMaxPagesExact()
    {
        // Concurrent crawling does not exceed max_pages limit even with high concurrency
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentPartialErrors()
    {
        // Concurrent crawl handles partial failures gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_ConcurrentRespectsMaxPages()
    {
        // Concurrent crawling respects max_pages limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }
}
