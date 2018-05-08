## Before releasing 0.1.0

[x] look for possible errors in get_opcode and get_instruction
[x] remove unwrap() calls from src/
[ ] README: write that binutils is built from source an statically linked
[ ] custom errors cleanup and tests

## Wish list / Roadmap

[ ] convert check_null_pointer() to a macro to add file and line numbers to the Error
[ ] write more tests
[ ] add Travis support: test and rustfmt
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] generate documentation from comments
[ ] use the error_chain crate
[ ] investigate info->stop_vma
[ ] rewrite copy_buffer in Rust
