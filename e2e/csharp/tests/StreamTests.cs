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
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/crawl_stream_events";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    [Fact]
    public async Task Test_StreamDepthCrawl()
    {
        // Stream produces events for multi-depth crawl with link following
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/stream_depth_crawl";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    [Fact]
    public async Task Test_StreamWithErrorEvent()
    {
        // Stream emits page and complete events even when some pages fail
        var engine = KreuzcrawlLib.CreateEngine(null);
        var url = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") + "/fixtures/stream_with_error_event";
        var result = await KreuzcrawlLib.Scrape(engine, url);
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }
}
