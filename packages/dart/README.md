# kreuzcrawl

High-performance web crawling engine

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  kreuzcrawl: ^0.3.0-rc.37
```

Then run:

```sh
dart pub get
```

## Building

From the repository root:

```sh
cargo build -p kreuzcrawl-dart
flutter_rust_bridge_codegen generate
dart pub get
dart analyze
dart test
```

## License

Elastic-2.0
