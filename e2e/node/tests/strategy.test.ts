import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('strategy', () => {
  it('strategy_best_first_seed: BestFirst strategy always processes the seed URL first', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.first_page_url_contains' not available on result type
  });

  it('strategy_bfs_default_order: BFS strategy visits pages in breadth-first order', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.crawl_order' not available on result type
  });

  it('strategy_dfs_depth_first: DFS strategy visits pages in depth-first order', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.crawl_order' not available on result type
  });
});
