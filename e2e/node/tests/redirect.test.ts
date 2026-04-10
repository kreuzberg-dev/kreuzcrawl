import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('redirect', () => {
  it('redirect_301_permanent: Follows 301 permanent redirect and returns final page content', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/target");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_302_found: Follows 302 Found redirect correctly', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/found-target");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_303_see_other: Follows 303 See Other redirect (method changes to GET)', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/see-other");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_307_temporary: Follows 307 Temporary Redirect (preserves method)', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/temp-target");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_308_permanent: Follows 308 Permanent Redirect (preserves method)', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/perm-target");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_chain: Follows a chain of redirects (301 -> 302 -> 200)', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/step2");
    expect(result.redirectCount).toBe(2);
  });

  it('redirect_cross_domain: Reports cross-domain redirect target without following to external domain', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/external-redirect");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_loop: Detects redirect loop (A -> B -> A) and returns error', async () => {
    const result = await scrape();
    expect(result.isError).toBe(true);
  });

  it('redirect_max_exceeded: Aborts when redirect count exceeds max_redirects limit', async () => {
    const result = await scrape();
    expect(result.isError).toBe(true);
  });

  it('redirect_meta_refresh: Follows HTML meta-refresh redirect to target page', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/target");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_refresh_header: Handles HTTP Refresh header redirect', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/refreshed");
    expect(result.redirectCount).toBe(1);
  });

  it('redirect_to_404: Redirect target returns 404 Not Found', async () => {
    const result = await scrape();
    expect(result.finalUrl).toContain("/gone");
    expect(result.redirectCount).toBe(1);
    expect(result.isError).toBe(true);
  });
});
