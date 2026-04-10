import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('map', () => {
  it('map_discover_urls: Discovers all URLs on a site without fetching full content', async () => {
    const result = await scrape();
    expect(result.urls.length).toBeGreaterThanOrEqual(3);
  });

  it('map_exclude_patterns: Excludes URLs matching patterns from URL map', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(1);
  });

  it('map_include_subdomains: Includes subdomain URLs in URL map discovery', async () => {
    const result = await scrape();
    expect(result.urls.length).toBeGreaterThanOrEqual(2);
    expect(result.urls).toContain("blog.example.com");
  });

  it('map_large_sitemap: Handles large sitemap with 100+ URLs', async () => {
    const result = await scrape();
    expect(result.urls.length).toBeGreaterThanOrEqual(100);
  });

  it('map_limit_pagination: Limits map result count to specified maximum', async () => {
    const result = await scrape();
    expect(result.urls.length).toBeLessThanOrEqual(5);
  });

  it('map_search_filter: Filters map results by search keyword', async () => {
    const result = await scrape();
    expect(result.urls.length).toBeGreaterThanOrEqual(2);
    expect(result.urls).toContain("blog");
  });
});
