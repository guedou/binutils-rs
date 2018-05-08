## Before releasing 0.1.0

[ ] the iterator API example does not print the first instruction!
[x] is DisassembleInfo::empty() useful ? aka could di.raw be really null ?
[ ] is it possible to have a specific Bfd::empty() implementation ?
[ ] function that wraps checking for null pointers:
    fn xxx(ptr, message) {
      xxx.is_null()
      Ok()
      Err()
    }
[ ] test_ls() should return Result<(), Error>

## Wish list / Roadmap

[ ] add Travis support: test and rustfmt
[ ] write more tests
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] use info->stop_vma ?
[ ] use the error_chain crate
[ ] generate documentation from comments
[ ] rewrite copy_buffer in Rust
