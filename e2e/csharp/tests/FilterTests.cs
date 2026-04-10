using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: filter.</summary>
public class FilterTests
{
    [Fact]
    public async Task Test_FilterBm25CrawlIntegration()
    {
        // BM25 filter works during multi-page crawl, keeping relevant pages
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    [Fact]
    public async Task Test_FilterBm25EmptyQuery()
    {
        // BM25 filter with empty query passes all pages through
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    [Fact]
    public async Task Test_FilterBm25HighThreshold()
    {
        // BM25 filter with very high threshold filters out all pages
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    [Fact]
    public async Task Test_FilterBm25RelevantPages()
    {
        // BM25 filter keeps only pages relevant to the query
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    [Fact]
    public async Task Test_FilterBm25ThresholdZero()
    {
        // BM25 filter with zero threshold passes all pages
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    [Fact]
    public async Task Test_FilterNoopCrawlAllKept()
    {
        // NoopFilter keeps all pages during a multi-page crawl
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    [Fact]
    public async Task Test_FilterNoopPassesAll()
    {
        // No content filter passes all crawled pages through
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }
}
