```elixir
# Create engine with default settings
{:ok, engine} = Kreuzcrawl.create_engine(nil)

# Scrape a single page
{:ok, result} = Kreuzcrawl.scrape(engine, "https://example.com")
IO.puts("Title: #{result.metadata.title}")
IO.puts("Status: #{result.status_code}")
IO.puts("Links: #{length(result.links)}")
```
