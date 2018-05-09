## Before releasing 0.1.0

[ ] build binutils inside target/
[x] do pointer arithmetic with checked_add() (i.e. when manipulating buffer_asm)
[x] write a wrapper to fill the buffer in change_address()
[ ] change_address() seems broken
[ ] custom errors cleanup and tests

## Wish list / Roadmap

[ ] add Travis support: test and rustfmt
[ ] code coverage with tarpaulin
[ ] write more tests
[ ] convert check_null_pointer() to a macro to add file and line numbers to the Error
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] generate documentation from comments
[ ] use the error_chain crate
[ ] investigate info->stop_vma
[ ] rewrite copy_buffer in Rust
