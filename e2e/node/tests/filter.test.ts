import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('filter', () => {
  it('filter_bm25_crawl_integration: BM25 filter works during multi-page crawl, keeping relevant pages', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.filter.remainingContainKeyword).toContain("rust");
  });

  it('filter_bm25_empty_query: BM25 filter with empty query passes all pages through', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(2);
  });

  it('filter_bm25_high_threshold: BM25 filter with very high threshold filters out all pages', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.filter.pagesAfterFilter).toBe(0);
  });

  it('filter_bm25_relevant_pages: BM25 filter keeps only pages relevant to the query', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.filter.remainingContainKeyword).toContain("rust");
  });

  it('filter_bm25_threshold_zero: BM25 filter with zero threshold passes all pages', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(2);
  });

  it('filter_noop_crawl_all_kept: NoopFilter keeps all pages during a multi-page crawl', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.filter.pagesAfterFilter).toBe(3);
  });

  it('filter_noop_passes_all: No content filter passes all crawled pages through', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(3);
  });
});
