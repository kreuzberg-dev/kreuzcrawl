using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: redirect.</summary>
public class RedirectTests
{
    [Fact]
    public async Task Test_Redirect301Permanent()
    {
        // Follows 301 permanent redirect and returns final page content
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_Redirect302Found()
    {
        // Follows 302 Found redirect correctly
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/found-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_Redirect303SeeOther()
    {
        // Follows 303 See Other redirect (method changes to GET)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/see-other", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_Redirect307Temporary()
    {
        // Follows 307 Temporary Redirect (preserves method)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/temp-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_Redirect308Permanent()
    {
        // Follows 308 Permanent Redirect (preserves method)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/perm-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_RedirectChain()
    {
        // Follows a chain of redirects (301 -> 302 -> 200)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/step2", result.FinalUrl);
        Assert.Equal(2, result.RedirectCount);
    }

    [Fact]
    public async Task Test_RedirectCrossDomain()
    {
        // Reports cross-domain redirect target without following to external domain
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/external-redirect", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_RedirectLoop()
    {
        // Detects redirect loop (A -> B -> A) and returns error
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.IsError);
    }

    [Fact]
    public async Task Test_RedirectMaxExceeded()
    {
        // Aborts when redirect count exceeds max_redirects limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(true, result.IsError);
    }

    [Fact]
    public async Task Test_RedirectMetaRefresh()
    {
        // Follows HTML meta-refresh redirect to target page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_RedirectRefreshHeader()
    {
        // Handles HTTP Refresh header redirect
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/refreshed", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
    }

    [Fact]
    public async Task Test_RedirectTo404()
    {
        // Redirect target returns 404 Not Found
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Contains("/gone", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount);
        Assert.Equal(true, result.IsError);
    }
}
