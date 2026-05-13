```php title="PHP"
<?php
declare(strict_types=1);

use Kreuzcrawl\CrawlConfig;
use Kreuzcrawl\Kreuzcrawl;

// Simplest case: scrape a single page with default settings.
$engine = Kreuzcrawl::createEngine(null);
$result = Kreuzcrawl::scrape($engine, "https://example.com/");
echo "Title: " . ($result->metadata->title ?? "") . "\n";
echo "Status: " . $result->statusCode . "\n";
echo "Links found: " . count($result->links) . "\n";

// Crawl from a seed URL, limited to one hop and a handful of pages.
$config = CrawlConfig::default();
$config->maxDepth = 1;
$config->maxPages = 5;
$crawlEngine = Kreuzcrawl::createEngine($config);
$crawlResult = Kreuzcrawl::crawl($crawlEngine, "https://en.wikipedia.org/wiki/Web_scraping");
echo "Pages crawled: " . count($crawlResult->pages) . "\n";
```
