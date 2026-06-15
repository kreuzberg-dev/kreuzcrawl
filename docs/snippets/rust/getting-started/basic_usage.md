```rust
use kreuzcrawl::{create_engine, scrape};

#[tokio::main]
async fn main() -> kreuzcrawl::Result<()> {
    let engine = create_engine(None)?;
    let result = scrape(&engine, "https://example.com").await?;

    println!("{}", result.metadata.title.as_deref().unwrap_or("(no title)"));
    println!("{}", result.markdown.content);
    println!("{} links", result.links.len());

    Ok(())
}
```
