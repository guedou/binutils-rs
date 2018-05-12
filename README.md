# binutils-rs

A Rust library that ease interacting with the
[binutils](https://www.gnu.org/software/binutils/) disassembly engine with a
high-level API. Its main goal is to simplify disassembling raw buffers into
instructions.

[![crates.io badge](https://img.shields.io/crates/v/binutils.svg)](https://crates.io/crates/binutils/)
[![doc.rs badge](https://docs.rs/mio/badge.svg)](https://docs.rs/crate/binutils/)

## Usage

You need to add the following lines to your `Cargo.toml`:
```toml
[dependencies]
binutils = "0.1.1"
```

and make sure that your are using it in your code:
```rust
extern crate binutils;
```

> **Note:**
By default, all architectures supported by binutils will be built by cargo. The
resulting library will be over 60MB. When size is an issue, the `TARGETS`
environment variable can be set to only build specific architectures (i.e.
`TARGETS=arm-linux,mep`) as defined in `bfd/config.bfd`.
>

## Examples

Here is how to disassemble a buffer containing x86 instructions while being
gentle with errors:
```rust
extern crate binutils;
use binutils::utils::disassemble_buffer;
use binutils::opcodes::DisassembleInfo;

// Prepare the disassembler
let mut info = disassemble_buffer("i386", &[0xc3, 0x90, 0x66, 0x90], 0x2800)
    .unwrap_or(DisassembleInfo::empty());

// Iterate over the instructions
loop {
    match info.disassemble()
        .ok_or(2807)
        .map(|i| Some(i.unwrap()))
        .unwrap_or(None)
    {
        Some(instruction) => println!("{}", instruction),
        None => break,
    }
}
```

Other examples are located in the [examples/](examples) directory, and be used
with `cargo run --example`.

## Resources

Examples in C and archived documentations are available in the
[resources/](resources/) directory.

## Roadmap

- [ ] add Travis support: test and rustfmt
- [ ] code coverage with tarpaulin
- [ ] write more tests
- [ ] investigate stripping libraries
- [ ] convert check_null_pointer() to a macro to add file and line numbers to the Error
- [ ] fuzz the disassembler
- [ ] generate mach.rs with build.rs
- [ ] generate documentation from comments
- [ ] use the error_chain crate
- [ ] investigate info->stop_vma
- [ ] rewrite copy_buffer in Rust
