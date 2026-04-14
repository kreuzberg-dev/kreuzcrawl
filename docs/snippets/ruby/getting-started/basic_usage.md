```ruby
require 'kreuzcrawl'

# Create engine with default settings
engine = Kreuzcrawl.create_engine(nil)

# Scrape a single page
result = Kreuzcrawl.scrape(engine, "https://example.com")
puts "Title: #{result.metadata.title}"
puts "Status: #{result.status_code}"
puts "Links: #{result.links.length}"
```
