## Before releasing 0.1.0

[ ] check binutils tarball checksum
[x] use clippy
[x] use features to select architectures
[x] improve the README: examples and TARGETS
  [x] write a more compact example using utils::disassemble_buffer
[ ] errors should use &str instead of String
[ ] look for incorrect pointers usage!
  [ ] Bfd methods should return errors -> BfdError
  [ ] Section methods should return errors -> SectionError
  [ ] DisassembleInfo methods should return errors -> DisassembleInfoError
  [ ] helpers.c must not use NULL pointers!

## Wish list / Roadmap

[ ] add Travis support: test and rustfmt
[ ] write more tests
[ ] fuzz the disassembler
[ ] generate mach.rs with build.rs
[ ] use info->stop_vma ?
[ ] use the error_chain crate
[ ] generate documentation from comments
[ ] rewrite copy_buffer in Rust
