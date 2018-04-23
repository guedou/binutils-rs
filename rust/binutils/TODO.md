[x] newer binutils version
[_] disassemble a buffer passed from Rust
[_] build binutils with cargo
[_] build libraries wil all architectures using build.rs

[_] make disassemble return an Instruction structure
[_] make disassemble return an interator

[_] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[_] write examples
[_] write tests

[_] use features to select architectures
