## Rust

[x] newer binutils version
[x] disassemble a buffer passed from Rust
[x] build binutils with cargo
[x] build libraries with all architectures using build.rs
[x] refactor the crate with 'mod': Bfd, DisassembleInfo, ...
[ ] binutils-rs
[x] find a way to avoid specifying `LD_LIBRARY_PATH`
[x] implement the DisassemleInfo destructor
[ ] rename tmp_buf_asm: the ptr is likely useless!
[ ] clean and comment functions
[ ] rename scan_arch to set_arch_mach

[x] make disassemble return an Instruction structure

[ ] build a high level API to disassemble a section from an ELF and a buffer
    disassemble_elf_section!("file", ".text")
    disassemble_buffer!((arch, mach), buffer)

[ ] write examples
[ ] write tests
[ ] fuzz the disassembler

[ ] use info->stop_vma ?
[ ] use features to select architectures

[x] make disassemble return an iterator
[ ] generate mach.rs with build.rs
[ ] download binutils with curl or git

## Examples

[ ] autobuild binutils and libopcodes
[ ] lint C examples
[ ] clean them
[ ] get instruction types from binutils ?
[ ] r2lo\_mep: ad and A\_
    [ ] change the instructions style (mov $r1, 2 -> MOV R1, 2) ; intel vs att ?
[ ] Python bindings to libopcodes.so
    [ ] disassembling
    [ ] assembling
