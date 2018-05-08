## Before releasing 0.1.0

[ ] the iterator API example does not print the first instruction!
[x] is DisassembleInfo::empty() useful ? aka could di.raw be really null ?
[ ] is it possible to have a specific Bfd::empty() implementation ?
[ ] test_ls() should return Result<(), Error>
[ ] convert check_null_pointer to a macro to add file and line numbers to the
    Error

## Wish list / Roadmap

[ ] add Travis support: test and rustfmt
[ ] write more tests
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] use info->stop_vma ?
[ ] use the error_chain crate
[ ] generate documentation from comments
[ ] rewrite copy_buffer in Rust
