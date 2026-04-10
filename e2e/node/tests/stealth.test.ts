import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('stealth', () => {
  it('stealth_ua_rotation_config: User-agent rotation config is accepted and crawl succeeds', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
  });
});
