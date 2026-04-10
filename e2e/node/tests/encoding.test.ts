import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('encoding', () => {
  it('encoding_double_encoded: Handles double-encoded URL characters (%25C3%25B6)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.links.length).toBeGreaterThanOrEqual(1);
  });

  it('encoding_mixed_charset_page: Handles charset mismatch between HTTP header and HTML meta tag', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
  });

  it('encoding_percent_encoded_path: Handles percent-encoded spaces and characters in URL paths', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.links.length).toBeGreaterThanOrEqual(2);
  });

  it('encoding_unicode_url: Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
  });
});
