import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('stream', () => {
  it('crawl_stream_events: Crawl stream produces page and complete events', async () => {
    const result = await scrape();
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(4);
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
  });

  it('stream_depth_crawl: Stream produces events for multi-depth crawl with link following', async () => {
    const result = await scrape();
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(5);
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
  });

  it('stream_with_error_event: Stream emits page and complete events even when some pages fail', async () => {
    const result = await scrape();
    expect(result.stream.hasPageEvent).toBe(true);
    expect(result.stream.hasCompleteEvent).toBe(true);
    expect(result.stream.eventCountMin).toBeGreaterThanOrEqual(2);
  });
});
