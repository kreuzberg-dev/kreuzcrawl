import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('sitemap', () => {
  it('sitemap_basic: Parses a standard urlset sitemap', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(4);
    expect(result.hasLastmod).toBe(true);
  });

  it('sitemap_compressed_gzip: Parses a gzip-compressed sitemap file', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(3);
  });

  it('sitemap_empty: Handles empty sitemap gracefully', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(0);
  });

  it('sitemap_from_robots_txt: Discovers sitemap via robots.txt Sitemap directive', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(4);
  });

  it('sitemap_index: Follows sitemap index to discover child sitemaps', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(3);
  });

  it('sitemap_lastmod_filter: Filters sitemap URLs by lastmod date', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(4);
    expect(result.hasLastmod).toBe(true);
  });

  it('sitemap_only_mode: Uses sitemap URLs exclusively without following page links', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(4);
  });

  it('sitemap_xhtml_links: Parses sitemap with XHTML namespace alternate links', async () => {
    const result = await scrape();
    expect(result.urls.length).toBe(2);
    expect(result.hasLastmod).toBe(false);
  });
});
