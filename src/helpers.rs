// Guillaume Valadon <guillaume@valadon.net>
// binutils bindings to helpers.c - helpers.rs

use libc::{c_char, c_uint, c_ulong};

use bfd::BfdRaw;

extern "C" {
    pub(crate) fn macro_bfd_big_endian(bfd: *const BfdRaw) -> bool;

    pub(crate) fn get_start_address(bfd: *const BfdRaw) -> c_ulong;

    pub(crate) fn get_arch(arch_info: *const c_uint) -> u32;

    pub(crate) fn get_mach(arch_info: *const c_uint) -> u64;

    pub static buffer_asm: [u8; 64];

    pub static mut buffer_asm_ptr: *mut c_char;
}
