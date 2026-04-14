```python
from kreuzcrawl import create_engine, scrape

# Create engine with default settings
engine = create_engine()

# Scrape a single page
result = scrape(engine, "https://example.com")
print(f"Title: {result.metadata.title}")
print(f"Status: {result.status_code}")
print(f"Links: {len(result.links)}")
```
