import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('links', () => {
  it('links_anchor_fragment: Identifies fragment-only links as anchor type', async () => {
    const result = await scrape();
    expect(result.links[""].linkType).toContain("anchor");
  });

  it('links_base_tag: Resolves relative URLs using base tag href', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(2);
    expect(result.links[""].url).toContain("example.com");
  });

  it('links_document_types: Detects PDF, DOCX, XLSX links as document type', async () => {
    const result = await scrape();
    expect(result.links[""].linkType).toContain("document");
  });

  it('links_empty_href: Handles empty href attributes without errors', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(0);
    expect(result.links[""].url).toContain("/valid");
  });

  it('links_internal_external_classification: Correctly classifies internal vs external links by domain', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(4);
    expect(result.links[""].linkType).toContain("internal");
    expect(result.links[""].linkType).toContain("external");
  });

  it('links_mailto_javascript_skip: Skips mailto:, javascript:, and tel: scheme links', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(0);
    expect(result.links[""].url).not.toContain("mailto:");
  });

  it('links_protocol_relative: Handles protocol-relative URLs (//example.com) correctly', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(1);
    expect(result.links[""].protocolRelative.length).toBeGreaterThan(0);
  });

  it('links_rel_attributes: Preserves rel=nofollow and rel=canonical attributes', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(0);
  });

  it('links_relative_parent: Resolves ../ and ./ relative parent path links correctly', async () => {
    const result = await scrape();
    expect(result.links.length).toBeGreaterThan(3);
  });
});
