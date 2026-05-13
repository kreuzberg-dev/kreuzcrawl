```ruby title="Ruby"
require "kreuzcrawl"

# Simplest case: scrape a single page with default settings.
engine = Kreuzcrawl.create_engine
result = Kreuzcrawl.scrape(engine, "https://example.com/")
puts "Title: #{result.metadata.title}"
puts "Status: #{result.status_code}"
puts "Links found: #{result.links.length}"

# Crawl from a seed URL, limited to one hop and a handful of pages.
config = Kreuzcrawl::CrawlConfig.new(max_depth: 1, max_pages: 5)
crawl_engine = Kreuzcrawl.create_engine(config)
crawl_result = Kreuzcrawl.crawl(crawl_engine, "https://en.wikipedia.org/wiki/Web_scraping")
puts "Pages crawled: #{crawl_result.pages.length}"
```
