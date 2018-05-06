# binutils-rs

A Rust library that ease interacting with the
[binutils](https://www.gnu.org/software/binutils/) disassembly engine with a
high-level API. Its main goal is to simplify disassembling raw buffers into
instructions.

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

> **Note:**
By default all architectures supported by binutils will be built. The resulting
library will be over 60MB. When size is an issue, the `TARGETS` environment
variable can be set to only build specific architectures (i.e.
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

## Motivation

- ease using binutils; wonderful set of libraries but hard to use
- playground for binutils experiments
- Rust API
- not pure FFI bindings, focus on disassembling from Rust

## Resources

Examples in C and archived documentations are available in the
[resources/](resources/) directory.
