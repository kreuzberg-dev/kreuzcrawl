import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('engine', () => {
  it('engine_batch_basic: CrawlEngine with defaults batch scrapes like the free function', async () => {
    const result = await scrape();
    expect(result.batch.completedCount).toBe(2);
    expect(result.batch.totalCount).toBe(2);
  });

  it('engine_crawl_basic: CrawlEngine with defaults crawls multiple pages like the free function', async () => {
    const result = await scrape();
    expect(result.crawl.pagesCrawled).toBe(3);
    expect(result.crawl.minPages).toBeGreaterThanOrEqual(3);
  });

  it('engine_map_basic: CrawlEngine with defaults discovers URLs like the free function', async () => {
    const result = await scrape();
    expect(result.map.minUrls).toBeGreaterThanOrEqual(2);
  });

  it('engine_scrape_basic: CrawlEngine with defaults scrapes a page identically to the free function', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.contentType).toBe("text/html");
    expect(result.metadata.title).toBe("Engine Test");
    expect(result.metadata.descriptionContains).toContain("Testing the engine");
    expect(result.links.minCount).toBeGreaterThanOrEqual(1);
    expect(result.headings.h1Count).toBe(1);
    expect(result.headings.h1Text).toBe("Hello Engine");
  });

  it('engine_stream_basic: CrawlEngine with defaults streams events like the free function', async () => {
    const result = await scrape();
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(3);
  });
});
