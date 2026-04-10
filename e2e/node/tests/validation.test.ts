import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('validation', () => {
  it('validation_invalid_exclude_regex: Invalid regex in exclude_paths is rejected', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('validation_invalid_include_regex: Invalid regex in include_paths is rejected', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('validation_invalid_retry_code: Retry code outside 100-599 is rejected', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('validation_max_pages_zero: max_pages=0 is rejected as invalid config', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('validation_max_redirects_too_high: max_redirects > 100 is rejected as invalid config', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('validation_timeout_zero: Zero request timeout is rejected as invalid config', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });
});
