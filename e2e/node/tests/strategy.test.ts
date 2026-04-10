import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('strategy', () => {
  it('strategy_best_first_seed: BestFirst strategy always processes the seed URL first', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(3);
    expect(result.strategy.firstPageUrlContains).toContain("/");
  });

  it('strategy_bfs_default_order: BFS strategy visits pages in breadth-first order', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(5);
    expect(result.strategy.crawlOrder).toBe(["/", "/a", "/b", "/a/1", "/b/1"]);
  });

  it('strategy_dfs_depth_first: DFS strategy visits pages in depth-first order', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.crawl.pagesCrawled).toBe(5);
    expect(result.strategy.crawlOrder).toBe(["/", "/b", "/b/1", "/a", "/a/1"]);
  });
});
