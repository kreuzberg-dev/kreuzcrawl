import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('auth', () => {
  it('auth_basic_http: Sends HTTP Basic authentication header', async () => {
    const result = await scrape();
    expect(result.authHeaderSent).toBe(true);
    expect(result.statusCode).toBe(200);
  });

  it('auth_bearer_token: Sends Bearer token in Authorization header', async () => {
    const result = await scrape();
    expect(result.authHeaderSent).toBe(true);
    expect(result.statusCode).toBe(200);
  });

  it('auth_custom_header: Sends authentication via custom header (X-API-Key)', async () => {
    const result = await scrape();
    expect(result.authHeaderSent).toBe(true);
    expect(result.statusCode).toBe(200);
  });
});
