## Rust

[x] newer binutils version
[x] disassemble a buffer passed from Rust
[x] build binutils with cargo
[x] build libraries with all architectures using build.rs
[x] refactor the crate with 'mod': Bfd, DisassembleInfo, ...
[x] binutils-rs
[x] find a way to avoid specifying `LD_LIBRARY_PATH`
[x] implement the DisassemleInfo destructor
[x] rename tmp_buf_asm: the ptr is likely useless!
[x] rename scan_arch to set_arch_mach
[x] clean and comment functions
[x] licence MIT
[ ] try to use the va crate!
[ ] move main.rs to examples/
[ ] check binutils tarball checksum
[x] build to to OUT_DIR & use static libraries !
[x] download binutils-2.29.1.tar.gz from build.rs!
[x] fix cargo package
[ ] check if the MeP specific helper is useful
[ ] get_opcode: use buffer_asm_ptr to get the NUL byte
[ ] use the error_chain crate

[x] make disassemble return an Instruction structure

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[ ] write examples
[ ] write tests
[ ] fuzz the disassembler
[ ] generate documentation from comments?

[ ] use info->stop_vma ?
[x] make disassemble return an iterator

## Wish list / Roadmap
[ ] use features to select architectures
[ ] generate mach.rs with build.rs
