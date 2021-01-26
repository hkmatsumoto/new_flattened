# new_flattened

[![Cargo](https://img.shields.io/crates/v/new_flattened.svg)](https://crates.io/crates/new_flattened)

This crates provides `new_flattened!` macro which enables you to write
```rust
new_flattened!(42, Box, Box, Box)
```
instead of
```rust
Box::new(Box::new(Box::new(42)))
```
