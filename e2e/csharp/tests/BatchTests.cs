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
        Assert.Equal(3, result.Batch.CompletedCount);
        Assert.Equal(0, result.Batch.FailedCount);
        Assert.Equal(3, result.Batch.TotalCount);
    }

    [Fact]
    public async Task Test_ScrapeBatchPartialFailure()
    {
        // Batch scrape with one URL failing returns partial results
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(2, result.Batch.CompletedCount);
        Assert.Equal(1, result.Batch.FailedCount);
        Assert.Equal(3, result.Batch.TotalCount);
    }

    [Fact]
    public async Task Test_ScrapeBatchProgress()
    {
        // Batch scrape results include specific URL
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(2, result.Batch.TotalCount);
        Assert.Contains("/target", result.Batch.Results);
    }
}
