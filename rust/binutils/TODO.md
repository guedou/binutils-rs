[x] newer binutils version
[x] disassemble a buffer passed from Rust
[ ] refactor the crate with 'mod': Bfd, DisassembleInfo, ...
[ ] build binutils with cargo
[ ] build libraries wil all architectures using build.rs

[ ] make disassemble return an Instruction structure
[ ] make disassemble return an interator

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[ ] write examples
[ ] write tests

[ ] use features to select architectures
