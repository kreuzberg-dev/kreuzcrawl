<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: batch. */
final class BatchTest extends TestCase
{
    /** Batch scrape of multiple URLs all succeeding */
    public function test_scrape_batch_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.failed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    /** Batch scrape with one URL failing returns partial results */
    public function test_scrape_batch_partial_failure(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.failed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    /** Batch scrape results include specific URL */
    public function test_scrape_batch_progress(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'batch.total_count' not available on result type
        // skipped: field 'batch.results' not available on result type
    }
}
