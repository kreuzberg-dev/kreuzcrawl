using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: stream.</summary>
public class StreamTests
{
    [Fact]
    public async Task Test_CrawlStreamEvents()
    {
        // Crawl stream produces page and complete events
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Stream.EventCountMin >= 4, "expected >= 4");
        Assert.Equal(true, result.Stream.HasPageEvent);
        Assert.Equal(true, result.Stream.HasCompleteEvent);
    }

    [Fact]
    public async Task Test_StreamDepthCrawl()
    {
        // Stream produces events for multi-depth crawl with link following
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.True(result.Stream.EventCountMin >= 5, "expected >= 5");
        Assert.Equal(true, result.Stream.HasPageEvent);
        Assert.Equal(true, result.Stream.HasCompleteEvent);
    }

    [Fact]
    public async Task Test_StreamWithErrorEvent()
    {
        // Stream emits page and complete events even when some pages fail
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.Stream.HasPageEvent);
        Assert.Equal(true, result.Stream.HasCompleteEvent);
        Assert.True(result.Stream.EventCountMin >= 2, "expected >= 2");
    }
}
