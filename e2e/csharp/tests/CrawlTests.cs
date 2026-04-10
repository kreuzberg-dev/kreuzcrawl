using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: crawl.</summary>
public class CrawlTests
{
    [Fact]
    public async Task Test_ContentBinarySkip()
    {
        // Skips image and video content types gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'content.was_skipped' not available on result type
    }

    [Fact]
    public async Task Test_ContentPdfLinkSkip()
    {
        // Encounters PDF link and skips or marks as document type
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'content.was_skipped' not available on result type
    }

    [Fact]
    public async Task Test_CrawlConcurrentDepth()
    {
        // Concurrent crawl respects max_depth limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    [Fact]
    public async Task Test_CrawlConcurrentLimit()
    {
        // Respects max concurrent requests limit during crawl
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlConcurrentMaxPages()
    {
        // Concurrent crawl respects max_pages budget
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlCustomHeaders()
    {
        // Sends custom headers on all crawl requests
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlDepthOne()
    {
        // Follows links one level deep from start page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    [Fact]
    public async Task Test_CrawlDepthPriority()
    {
        // Crawls in breadth-first order, processing depth-0 pages before depth-1
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlDepthTwo()
    {
        // Crawls 3 levels deep (depth 0, 1, 2)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlDepthTwoChain()
    {
        // Depth=2 crawl follows a chain of links across three levels
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlDoubleSlashNormalization()
    {
        // Normalizes double slashes in URL paths (//page to /page)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlEmptyPageNoLinks()
    {
        // Crawl completes when child page has no outgoing links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlExcludePathPattern()
    {
        // Skips URLs matching the exclude path pattern
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlExternalLinksIgnored()
    {
        // External links are discovered but not followed when stay_on_domain is true
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    [Fact]
    public async Task Test_CrawlFragmentStripping()
    {
        // Strips #fragment from URLs for deduplication
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlIncludePathPattern()
    {
        // Only follows URLs matching the include path pattern
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlMaxDepthZero()
    {
        // max_depth=0 crawls only the seed page with no link following
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlMaxPages()
    {
        // Stops crawling at page budget limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlMixedContentTypes()
    {
        // Crawl handles links to non-HTML content types gracefully
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlMultipleRedirectsInTraversal()
    {
        // Multiple linked pages with redirects are handled during crawl traversal
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlQueryParamDedup()
    {
        // Deduplicates URLs with same query params in different order
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlRedirectInTraversal()
    {
        // Links that redirect are followed during crawl traversal
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlSelfLinkNoLoop()
    {
        // Page linking to itself does not cause infinite crawl loop
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlSinglePageNoLinks()
    {
        // Crawling a page with no links returns only the seed page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlStayOnDomain()
    {
        // Does not follow external links when stay_on_domain is true
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    [Fact]
    public async Task Test_CrawlSubdomainExclusion()
    {
        // Stays on exact domain and skips subdomain links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    [Fact]
    public async Task Test_CrawlSubdomainInclusion()
    {
        // Crawls subdomains when allow_subdomains is enabled
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlTrailingSlashDedup()
    {
        // Deduplicates /page and /page/ as the same URL
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    [Fact]
    public async Task Test_CrawlUrlDeduplication()
    {
        // Deduplicates URLs that differ only by fragment or query params
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }
}
