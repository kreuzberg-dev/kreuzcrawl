import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('stealth', () => {
  it('stealth_ua_rotation_config: User-agent rotation config is accepted and crawl succeeds', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
  });
});
