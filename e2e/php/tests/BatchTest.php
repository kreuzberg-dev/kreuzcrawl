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
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, $result->batch->completed_count);
        $this->assertEquals(0, $result->batch->failed_count);
        $this->assertEquals(3, $result->batch->total_count);
    }

    /** Batch scrape with one URL failing returns partial results */
    public function test_scrape_batch_partial_failure(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, $result->batch->completed_count);
        $this->assertEquals(1, $result->batch->failed_count);
        $this->assertEquals(3, $result->batch->total_count);
    }

    /** Batch scrape results include specific URL */
    public function test_scrape_batch_progress(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, $result->batch->total_count);
        $this->assertStringContainsString("/target", $result->batch->results);
    }
}
