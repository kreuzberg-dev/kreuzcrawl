import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('stream', () => {
  it('crawl_stream_events: Crawl stream produces page and complete events', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(4);
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
  });

  it('stream_depth_crawl: Stream produces events for multi-depth crawl with link following', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(5);
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
  });

  it('stream_with_error_event: Stream emits page and complete events even when some pages fail', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(2);
  });
});
