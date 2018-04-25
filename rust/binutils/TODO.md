[x] newer binutils version
[x] disassemble a buffer passed from Rust
[x] build binutils with cargo
[x] build libraries with all architectures using build.rs
[ ] refactor the crate with 'mod': Bfd, DisassembleInfo, ...
[ ] binutils-rs
[x] find a way to avoid specifying `LD_LIBRARY_PATH`
[ ] download binutils with curl or git
[ ] generate mach.rs with build.rs
[x] implement the DisassemleInfo destructor
[ ] rename tmp_buf_asm

[x] make disassemble return an Instruction structure
[ ] make disassemble return an iterator

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[ ] write examples
[ ] write tests
[ ] fuzz the disassembler

[ ] use features to select architectures
