import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('metadata', () => {
  it('metadata_article_times: Extracts article:published_time, modified_time, author, section, and tags', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.article.publishedTime).toBe("2024-01-15T10:00:00Z");
    expect(result.article.modifiedTime).toBe("2024-06-20T14:30:00Z");
    expect(result.article.author).toBe("Jane Developer");
    expect(result.article.section).toBe("Technology");
    expect(result.article.tags.length).toBe(3);
  });

  it('metadata_favicons: Extracts favicon link tags including apple-touch-icon', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.favicons.length).toBe(5);
    expect(result.favicons[""].appleTouch.length).toBeGreaterThan(0);
  });

  it('metadata_headings: Extracts heading hierarchy (h1-h6) from HTML page', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.headings.h1.length).toBe(1);
    expect(result.headings.h1["0"].text).toBe("Primary Heading");
    expect(result.headings.length).toBe(8);
  });

  it('metadata_hreflang: Extracts hreflang alternate link tags', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.hreflang.length).toBe(4);
    expect(result.hreflang[""].lang).toContain("en");
  });

  it('metadata_keywords_author: Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.metadata.title).toBe("Comprehensive Metadata Test Page");
    expect(result.metadata.canonicalUrl.length).toBeGreaterThan(0);
    expect(result.metadata.keywords.length).toBeGreaterThan(0);
    expect(result.metadata.keywords).toContain("rust");
    expect(result.metadata.author).toBe("Jane Developer");
    expect(result.metadata.viewport.length).toBeGreaterThan(0);
    expect(result.metadata.generator).toBe("kreuzcrawl/1.0");
    expect(result.metadata.themeColor).toBe("#ff6600");
    expect(result.metadata.robots).toBe("index, follow");
    expect(result.metadata.lang).toBe("en");
    expect(result.metadata.dir).toBe("ltr");
  });

  it('metadata_og_video_audio: Extracts og:video, og:audio, and og:locale:alternate metadata', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.og.video).toBe("https://example.com/video.mp4");
    expect(result.og.audio).toBe("https://example.com/audio.mp3");
    expect(result.og.localeAlternate.length).toBe(2);
  });

  it('metadata_response_headers: Extracts response metadata from HTTP headers (etag, server, content-language)', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.responseHeaders.etag.length).toBeGreaterThan(0);
    expect(result.responseHeaders.lastModified.length).toBeGreaterThan(0);
    expect(result.responseHeaders.server).toContain("nginx");
    expect(result.responseHeaders.contentLanguage).toBe("en-US");
  });

  it('metadata_word_count: Computes word count from visible page text', async () => {
    const result = await scrape();
    expect(result.statusCode).toBe(200);
    expect(result.computed.wordCount).toBeGreaterThan(99);
    expect(result.computed.wordCount).toBeLessThan(301);
  });
});
