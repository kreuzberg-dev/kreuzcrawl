```go
package main

import (
    "fmt"
    kcrawl "github.com/kreuzberg-dev/kreuzcrawl/packages/go"
)

func main() {
    // Create engine with default settings
    engine, err := kcrawl.CreateEngine()
    if err != nil {
        panic(err)
    }

    // Scrape a single page
    result, err := kcrawl.Scrape(engine, "https://example.com")
    if err != nil {
        panic(err)
    }
    fmt.Printf("Title: %s\n", result.Metadata.Title)
    fmt.Printf("Status: %d\n", result.StatusCode)
    fmt.Printf("Links: %d\n", len(result.Links))
}
```
