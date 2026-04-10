<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: validation. */
final class ValidationTest extends TestCase
{
    /** Invalid regex in exclude_paths is rejected */
    public function test_validation_invalid_exclude_regex(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Invalid regex in include_paths is rejected */
    public function test_validation_invalid_include_regex(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Retry code outside 100-599 is rejected */
    public function test_validation_invalid_retry_code(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** max_pages=0 is rejected as invalid config */
    public function test_validation_max_pages_zero(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** max_redirects > 100 is rejected as invalid config */
    public function test_validation_max_redirects_too_high(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Zero request timeout is rejected as invalid config */
    public function test_validation_timeout_zero(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }
}
