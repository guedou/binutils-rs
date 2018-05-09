## Before releasing 0.1.0

[ ] build binutils inside target/
[ ] do pointer arithmetic with checked_add()
[ ] write a wrapper to fill the buffer in change_address()
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
