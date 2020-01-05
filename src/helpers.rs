// Guillaume Valadon <guillaume@valadon.net>
// binutils bindings to helpers.c - helpers.rs

#![doc(hidden)]

use std::ffi::CStr;

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

    pub(crate) fn free_disassemble_info(info: *const DisassembleInfoRaw, free_section: bool);

    pub(crate) fn get_disassemble_info_section(
        info: *const DisassembleInfoRaw,
    ) -> *const DisassembleInfoRaw;

    pub(crate) fn get_disassemble_info_section_vma(info: *const DisassembleInfoRaw) -> c_ulong;

    // Custom helpers
    #[allow(dead_code)]
    pub(crate) fn show_buffer(info: *const DisassembleInfoRaw);
}

pub(crate) static mut CURRENT_OPCODE: Option<String> = None;

#[no_mangle]
pub unsafe extern "C" fn buffer_to_rust(buffer: *const c_char) {
    let buffer_cstr = CStr::from_ptr(buffer);
    let current_string = match CURRENT_OPCODE {
        Some(ref o) => o,
        None => "",
    };
    let new_string = match buffer_cstr.to_str() {
        Ok(s) => s.to_string(),
        Err(e) => format!("buffer_to_rust() - {}", e),
    };
    CURRENT_OPCODE = Some(format!("{}{}", current_string, new_string));
}
