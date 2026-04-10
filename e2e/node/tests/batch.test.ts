import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('batch', () => {
  it('scrape_batch_basic: Batch scrape of multiple URLs all succeeding', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.batch.completedCount).toBe(3);
    expect(result.batch.failedCount).toBe(0);
    expect(result.batch.totalCount).toBe(3);
  });

  it('scrape_batch_partial_failure: Batch scrape with one URL failing returns partial results', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.batch.completedCount).toBe(2);
    expect(result.batch.failedCount).toBe(1);
    expect(result.batch.totalCount).toBe(3);
  });

  it('scrape_batch_progress: Batch scrape results include specific URL', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.batch.totalCount).toBe(2);
    expect(result.batch.results).toContain("/target");
  });
});
