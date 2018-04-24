[x] newer binutils version
[x] disassemble a buffer passed from Rust
[x] build binutils with cargo
[ ] build libraries with all architectures using build.rs
[ ] refactor the crate with 'mod': Bfd, DisassembleInfo, ...
[ ] binutils-rs
[ ] find a way to avoid specifying `LD_LIBRAR_PATH`

[ ] make disassemble return an Instruction structure
[ ] make disassemble return an interator

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[ ] write examples
[ ] write tests

[ ] use features to select architectures