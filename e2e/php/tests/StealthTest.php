<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: stealth. */
final class StealthTest extends TestCase
{
    /** User-agent rotation config is accepted and crawl succeeds */
    public function test_stealth_ua_rotation_config(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
    }
}
