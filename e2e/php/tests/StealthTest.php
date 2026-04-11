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
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/stealth_ua_rotation_config';
        $result = Kreuzcrawl::scrape($engine, $url);
        $this->assertEquals(200, $result->status_code);
    }
}
