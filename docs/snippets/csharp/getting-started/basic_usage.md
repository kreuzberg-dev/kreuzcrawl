```csharp
using Kreuzcrawl;

// Create engine with default settings
var engine = KreuzcrawlLib.CreateEngine(null);

// Scrape a single page
var result = await KreuzcrawlLib.Scrape(engine, "https://example.com");
Console.WriteLine($"Title: {result.Metadata.Title}");
Console.WriteLine($"Status: {result.StatusCode}");
Console.WriteLine($"Links: {result.Links.Count}");
```
