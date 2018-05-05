# binutils-rs

A Rust library that ease interacting with the
[binutils](https://www.gnu.org/software/binutils/) disassembly engine with a
high-level API.

## Usage

You need to add the following lines to your `Cargo.toml`:
```toml
[dependencies]
binutils = "0.1.0"
```

and make sure that your are using it in your code:
```rust
extern crate binutils;
```

## Examples

... TODO ...

## Motivation

- ease using binutils; wonderful set of libraries but hard to use
- playground for binutils experiments
- Rust API
- not pure FFI bindings, focus on disassembling from Rust

## Resources

Examples in C and archived documentations are available in the
[resources/](resources/) directory.
