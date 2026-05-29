# kreuzcrawl

High-performance web crawling engine

## Installation

Install Zig from [ziglang.org](https://ziglang.org/download/).

## Building

```sh
zig build
zig build test
```

## Usage

Add to your `build.zig.zon`:

```text
.dependencies = .{
    .kreuzcrawl = .{
        .path = "path/to/kreuzcrawl",
    },
},
```

## License

Elastic-2.0
