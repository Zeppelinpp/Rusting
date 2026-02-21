# Rust Learning Repository

This is my personal repository for learning the Rust programming language.

## Purpose

To learn Rust from the ground up, focusing on understanding its ownership system, type system, and modern language features.

## Current Learning Progress

### Topics Covered

- **Basic Project Structure**: Set up a Cargo project and understanding the workspace layout
- **Structs and Methods**: Created `Musician` and `Touring` structs with associated methods
- **Enums and Pattern Matching**: Implemented `Status` enum with variants carrying data, using `match` and `if let` for control flow
- **Ownership and Borrowing**: Working with owned `String` types and references

### Code Examples

| File | Description |
|------|-------------|
| `src/bin/struct_method.rs` | Demonstrates structs, methods, enums, and pattern matching through a musician/touring example |

## Running the Code

```bash
# Run the main binary
cargo run

# Run the struct/method example
cargo run --bin struct_method
```

## Next Steps

- [ ] Learn about ownership, borrowing, and lifetimes in depth
- [ ] Explore collections (Vec, HashMap)
- [ ] Error handling with Result and Option
- [ ] Traits and generics (Impl traits at [strcut_trait](./src/bin/struct_trait.rs))
- [ ] Concurrency and async/await
