## Before releasing 0.1.0

[ ] check binutils tarball checksum
[ ] check if the MeP specific helper is really useful

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

## Wish list / Roadmap

[ ] write more tests
[ ] generate mach.rs with build.rs
[ ] use features to select architectures
[ ] use info->stop_vma ?
[ ] use the error_chain crate
[ ] generate documentation from comments
[ ] rewrite copy_buffer in Rust
[ ] fuzz the disassembler
