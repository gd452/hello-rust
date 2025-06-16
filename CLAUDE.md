# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Setup

This is a Rust project. To initialize:
```bash
cargo init --name hello-rust
```

## Common Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run
cargo run --release
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show println! output
cargo test test_name       # Run specific test
```

### Lint and Format
```bash
cargo fmt              # Format code
cargo clippy          # Lint code
cargo clippy -- -W clippy::all  # Strict linting
```

### Check
```bash
cargo check           # Quick compilation check without producing binary
```

## Project Structure

Standard Rust project layout:
- `src/main.rs` - Entry point for binary crate
- `src/lib.rs` - Entry point for library crate (if applicable)
- `Cargo.toml` - Project manifest with dependencies and metadata
- `target/` - Build artifacts (git-ignored)