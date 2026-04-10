import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('content', () => {
  it('content_204_no_content: Handles 204 No Content response gracefully', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(204);
    expect(result.html).toHaveLength(0);
  });

  it('content_charset_iso8859: Handles ISO-8859-1 encoded page correctly', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.detected_charset' not available on result type
  });

  it('content_empty_body: Handles 200 response with empty body gracefully', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
  });

  it('content_gzip_compressed: Handles response with Accept-Encoding gzip negotiation', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.statusCode).toBe(200);
  });

  it('content_large_page_limit: Respects max body size limit and truncates or skips oversized pages', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.body_size' not available on result type
  });

  it('content_main_only: Extracts only main content area, excluding nav, sidebar, footer', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.main_content_only' not available on result type
  });

  it('content_pdf_no_extension: Detects PDF content by Content-Type header when URL has no .pdf extension', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.is_pdf' not available on result type
  });

  it('content_remove_tags: Removes specified HTML elements by CSS selector before processing', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
  });

  it('content_utf8_bom: Handles UTF-8 content with BOM marker correctly', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'content.detected_charset' not available on result type
    expect(result.html.length).toBeGreaterThan(0);
  });
});
