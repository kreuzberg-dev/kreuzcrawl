using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: batch.</summary>
public class BatchTests
{
    [Fact]
    public async Task Test_ScrapeBatchBasic()
    {
        // Batch scrape of multiple URLs all succeeding
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.failed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    [Fact]
    public async Task Test_ScrapeBatchPartialFailure()
    {
        // Batch scrape with one URL failing returns partial results
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.failed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    [Fact]
    public async Task Test_ScrapeBatchProgress()
    {
        // Batch scrape results include specific URL
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'batch.total_count' not available on result type
        // skipped: field 'batch.results' not available on result type
    }
}
