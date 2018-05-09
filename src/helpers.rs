// Guillaume Valadon <guillaume@valadon.net>
// binutils bindings to helpers.c - helpers.rs

#![doc(hidden)]

use libc::{c_char, c_uchar, c_uint, c_ulong, uintptr_t};

use bfd::BfdRaw;
use opcodes::DisassembleInfoRaw;
use section::SectionRaw;

extern "C" {
    // libbfd helpers
    pub(crate) fn macro_bfd_big_endian(bfd: *const BfdRaw) -> bool;

    pub(crate) fn get_start_address(bfd: *const BfdRaw) -> c_ulong;

    pub(crate) fn get_arch(arch_info: *const c_uint) -> u32;

    pub(crate) fn get_mach(arch_info: *const c_uint) -> u64;

    pub(crate) static buffer_asm: [u8; 64];

    pub(crate) static mut buffer_asm_ptr: *mut c_char;

    // libopcodes helpers
    pub(crate) fn new_disassemble_info() -> *const DisassembleInfoRaw;

    pub(crate) fn configure_disassemble_info(
        info: *const DisassembleInfoRaw,
        section: *const SectionRaw,
        bfd: *const BfdRaw,
    ) -> bool;

    pub(crate) fn configure_disassemble_info_buffer(
        info: *const DisassembleInfoRaw,
        arch: c_uint,
        mach: c_ulong,
    );

    pub(crate) fn set_print_address_func(
        info: *const DisassembleInfoRaw,
        print_function: extern "C" fn(c_ulong, *const uintptr_t),
    );

    pub(crate) fn set_buffer(
        info: *const DisassembleInfoRaw,
        buffer: *const c_uchar,
        length: c_uint,
        vma: c_ulong,
    ) -> *const SectionRaw;

    pub(crate) fn free_disassemble_info(info: *const DisassembleInfoRaw);

    pub(crate) fn get_disassemble_info_section(
        info: *const DisassembleInfoRaw,
    ) -> *const DisassembleInfoRaw;

    pub(crate) fn get_disassemble_info_section_vma(info: *const DisassembleInfoRaw) -> c_ulong;

    // Custom helpers
    #[allow(dead_code)]
    pub(crate) fn show_buffer(info: *const DisassembleInfoRaw);
}
