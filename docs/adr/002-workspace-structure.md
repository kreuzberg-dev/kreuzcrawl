# ADR-002: Workspace Structure — Two-Crate Layout

**Status**: Accepted

**Date**: 2026-03-09

## Context

kreuzcrawl needs to be consumable both as a Rust library (embedded in kreuzberg-cloud or other services) and as a standalone CLI tool. The codebase must support independent versioning of the library API vs the CLI interface, and allow downstream consumers to depend on the library without pulling in CLI-specific dependencies (clap, terminal formatting).

We considered three approaches:
1. **Single crate** with CLI behind a feature flag
2. **Two-crate workspace**: library + CLI binary
3. **Multi-crate workspace**: core types, web engine, CLI (3+ crates)

## Decision

### Two-crate Cargo workspace

```
kreuzcrawl/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── kreuzcrawl/         # Library crate (the engine)
│   └── kreuzcrawl-cli/     # Binary crate (CLI wrapper)
```

### Workspace Configuration

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
rust-version = "1.91"
```

All dependencies declared in `[workspace.dependencies]` — members reference via `workspace = true`.

### Why two crates, not more

The engine is small enough (~5K LOC target) that splitting into core/web/extract crates would create unnecessary import friction without meaningful compilation benefits. Module boundaries within the single library crate provide sufficient separation. If the codebase grows significantly (>15K LOC) or gains distinct feature domains (Chrome engine, Git crawler), we can split then.

### Why not a single crate with feature-gated CLI

Feature-gated binaries in Cargo are awkward:
- `cargo install kreuzcrawl --features cli` is poor UX
- Conditional `[[bin]]` sections don't exist in Cargo
- Library consumers would still download clap sources even if not compiled

### Relationship to kreuzberg-dev repos

Follows the same patterns established in kreuzberg-cloud (ADR-005):
- `crates/` directory for Rust code
- Workspace-level dependency management
- Workspace-level version inheritance
- Edition 2024, resolver 2

## Consequences

### Positive

- **Clean library API**: `kreuzcrawl` crate has zero CLI dependencies
- **Independent binary**: CLI can evolve its UX without affecting library API
- **Simple dependency**: `kreuzcrawl = "0.1"` in downstream Cargo.toml
- **Consistent with org patterns**: Same structure as kreuzberg, kreuzberg-cloud

### Negative

- **Two Cargo.toml files to maintain**: Minor overhead vs single crate
- **Version coupling**: Both crates share workspace version — CLI release tied to library release

## Notes

Implementation:
- `/Cargo.toml` — Workspace definition
- `crates/kreuzcrawl/Cargo.toml` — Library crate
- `crates/kreuzcrawl-cli/Cargo.toml` — CLI binary crate
