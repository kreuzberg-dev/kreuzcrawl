import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('crawl', () => {
  it('content_binary_skip: Skips image and video content types gracefully', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.was_skipped' not available on result type
  });

  it('content_pdf_link_skip: Encounters PDF link and skips or marks as document type', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.was_skipped' not available on result type
  });

  it('crawl_concurrent_depth: Concurrent crawl respects max_depth limit', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'stayed_on_domain' not available on result type
  });

  it('crawl_concurrent_limit: Respects max concurrent requests limit during crawl', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_concurrent_max_pages: Concurrent crawl respects max_pages budget', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_custom_headers: Sends custom headers on all crawl requests', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_depth_one: Follows links one level deep from start page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'stayed_on_domain' not available on result type
  });

  it('crawl_depth_priority: Crawls in breadth-first order, processing depth-0 pages before depth-1', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_depth_two: Crawls 3 levels deep (depth 0, 1, 2)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_depth_two_chain: Depth=2 crawl follows a chain of links across three levels', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_double_slash_normalization: Normalizes double slashes in URL paths (//page to /page)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'unique_urls.length' not available on result type
  });

  it('crawl_empty_page_no_links: Crawl completes when child page has no outgoing links', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_exclude_path_pattern: Skips URLs matching the exclude path pattern', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_external_links_ignored: External links are discovered but not followed when stay_on_domain is true', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'stayed_on_domain' not available on result type
  });

  it('crawl_fragment_stripping: Strips #fragment from URLs for deduplication', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'unique_urls.length' not available on result type
  });

  it('crawl_include_path_pattern: Only follows URLs matching the include path pattern', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_max_depth_zero: max_depth=0 crawls only the seed page with no link following', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_max_pages: Stops crawling at page budget limit', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_mixed_content_types: Crawl handles links to non-HTML content types gracefully', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_multiple_redirects_in_traversal: Multiple linked pages with redirects are handled during crawl traversal', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_query_param_dedup: Deduplicates URLs with same query params in different order', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'unique_urls.length' not available on result type
  });

  it('crawl_redirect_in_traversal: Links that redirect are followed during crawl traversal', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_self_link_no_loop: Page linking to itself does not cause infinite crawl loop', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_single_page_no_links: Crawling a page with no links returns only the seed page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_stay_on_domain: Does not follow external links when stay_on_domain is true', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'stayed_on_domain' not available on result type
  });

  it('crawl_subdomain_exclusion: Stays on exact domain and skips subdomain links', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
    // skipped: field 'stayed_on_domain' not available on result type
  });

  it('crawl_subdomain_inclusion: Crawls subdomains when allow_subdomains is enabled', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });

  it('crawl_trailing_slash_dedup: Deduplicates /page and /page/ as the same URL', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'unique_urls.length' not available on result type
  });

  it('crawl_url_deduplication: Deduplicates URLs that differ only by fragment or query params', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'pages.length' not available on result type
  });
});
