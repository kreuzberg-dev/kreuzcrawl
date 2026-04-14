```php
<?php
use Kreuzcrawl\Kreuzcrawl;

// Create engine with default settings
$engine = Kreuzcrawl::createEngine(null);

// Scrape a single page
$result = Kreuzcrawl::scrape($engine, "https://example.com");
echo "Title: " . $result->metadata->title . "\n";
echo "Status: " . $result->status_code . "\n";
echo "Links: " . count($result->links) . "\n";
```
