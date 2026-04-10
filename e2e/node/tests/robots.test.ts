import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('robots', () => {
  it('robots_allow_all: Permissive robots.txt allows all paths', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_allow_override: Allow directive overrides Disallow for specific paths', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_comments_handling: Correctly parses robots.txt with inline and line comments', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_crawl_delay: Respects crawl-delay directive from robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.crawlDelay).toBe(2);
  });

  it('robots_disallow_path: Robots.txt disallows specific paths', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(false);
  });

  it('robots_meta_nofollow: Detects nofollow meta robots tag and skips link extraction', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.nofollowDetected).toBe(true);
  });

  it('robots_meta_noindex: Detects noindex meta robots tag in HTML page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.noindexDetected).toBe(true);
  });

  it('robots_missing_404: Missing robots.txt (404) allows all crawling', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_multiple_user_agents: Picks the most specific user-agent block from robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_request_rate: Parses request-rate directive from robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.crawlDelay).toBe(5);
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_sitemap_directive: Discovers sitemap URL from Sitemap directive in robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(true);
  });

  it('robots_user_agent_specific: Matches user-agent specific rules in robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(false);
  });

  it('robots_wildcard_paths: Handles wildcard Disallow patterns in robots.txt', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.isAllowed).toBe(false);
  });

  it('robots_x_robots_tag: Respects X-Robots-Tag HTTP header directives', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.robots.xRobotsTag).toBe("noindex, nofollow");
    expect(result.robots.noindexDetected).toBe(true);
    expect(result.robots.nofollowDetected).toBe(true);
  });
});
