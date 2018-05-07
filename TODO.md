## Before releasing 0.1.0

[ ] look for incorrect pointers usage!
  [ ] Bfd methods should return errors -> BfdError
  [ ] Section methods should return errors -> SectionError
  [ ] DisassembleInfo methods should return errors -> DisassembleInfoError
  [x] helpers.c must not use NULL pointers!
[ ] the iterator API example does not print the first instruction!
[ ] is DisassembleInfo::empty() useful ? aka could di.raw be really null ?

## Wish list / Roadmap

[ ] add Travis support: test and rustfmt
[ ] write more tests
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] use info->stop_vma ?
[ ] use the error_chain crate
[ ] generate documentation from comments
[ ] rewrite copy_buffer in Rust
