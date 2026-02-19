# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a personal Rust learning repository using the 2024 edition. It contains practice code for learning Rust concepts like structs, methods, enums, and pattern matching.

## Commands

### Build and Run

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the main binary
cargo run

# Run a specific binary from src/bin/
cargo run --bin struct_method
```

### Development

```bash
# Check code for errors without building
cargo check

# Format code
cargo fmt

# Run Clippy linter
cargo clippy

# Clean build artifacts
cargo clean
```

### Testing

```bash
# Run all tests
cargo test

# Run a specific test by name
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture
```

## Architecture

This is a simple Cargo workspace with the following structure:

- **`src/main.rs`** - Entry point (currently a "Hello, world!" placeholder)
- **`src/bin/`** - Additional binaries for learning exercises
  - `struct_method.rs` - Practice with structs, methods, enums, and pattern matching (Musician/Touring example)

There are currently no external dependencies defined in `Cargo.toml`.

## Project Edition

This repository uses **Rust 2024 edition** as specified in `Cargo.toml`.
