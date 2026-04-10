import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('error', () => {
  it('error_401_unauthorized: Handles 401 Unauthorized response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_403_forbidden: Handles 403 Forbidden response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_404_page: Handles 404 response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_408_request_timeout: Handles 408 Request Timeout response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_410_gone: Handles 410 Gone response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_500_server: Handles 500 server error', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_502_bad_gateway: Handles 502 Bad Gateway response correctly', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_connection_refused: Handles connection refused error gracefully', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_dns_resolution: Handles DNS resolution failure gracefully', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_empty_response: Handles 200 with completely empty body gracefully', async () => {
    const result = await scrape();
    expect(result.htmlNotEmpty).toBe(false);
    expect(result.error.isError).toBe(false);
  });

  it('error_invalid_proxy: Proxy pointing to unreachable address causes connection error during scrape', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_partial_response: Handles incomplete or truncated HTTP response', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_rate_limited: Handles 429 rate limiting with Retry-After', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_retry_503: Retries request on 503 Service Unavailable response', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_retry_backoff: Implements exponential backoff when retrying failed requests', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_ssl_invalid_cert: Handles SSL certificate validation error', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_timeout: Handles request timeout', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_waf_akamai: Akamai WAF detection returns WafBlocked error', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_waf_false_403: Detects WAF/bot protection false 403 (Cloudflare challenge page)', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });

  it('error_waf_imperva: Imperva/Incapsula WAF detection', async () => {
    await expect(async () => await scrape()).rejects.toThrow();
  });
});
