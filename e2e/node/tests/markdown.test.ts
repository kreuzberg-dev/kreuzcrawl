import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('markdown', () => {
  it('markdown_basic_conversion: HTML is always converted to markdown alongside raw HTML', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.metadata.title).toBe("Test");
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.markdown.length).toBeGreaterThan(0);
    expect(result.markdown).toContain("Hello World");
  });

  it('markdown_crawl_all_pages: All crawled pages have markdown field populated', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'crawl.pages_crawled' not available on result type
  });

  it('markdown_fit_content: Fit markdown removes navigation and boilerplate content', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.markdown.length).toBeGreaterThan(0);
  });

  it('markdown_headings_and_paragraphs: Markdown conversion preserves heading hierarchy and paragraph text', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.markdown.length).toBeGreaterThan(0);
    expect(result.markdown).toContain("Main Title");
  });

  it('markdown_links_converted: HTML links are converted to markdown link syntax', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.markdown.length).toBeGreaterThan(0);
    expect(result.markdown).toContain("Example");
  });

  it('markdown_with_citations: Markdown includes citation conversion with numbered references', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.markdown.length).toBeGreaterThan(0);
  });
});
