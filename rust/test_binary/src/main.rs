// Guillaume Valadon <guillaume@valadon.net>

extern crate libc;
use libc::{c_char, c_int, c_ulong};

use std::ffi::{CStr, CString};
use std::io::Write;


// libbfd bindings

// Rust bfd types
// Note: - trick from https://doc.rust-lang.org/nomicon/ffi.html
//       - it allows to use the Rust type checker
enum Section {}
enum Bfd {}

#[allow(non_camel_case_types)] // use the same enum names as libbfd
#[allow(dead_code)] // don't warn that some variants are not used
#[repr(C)]
enum BfdFormat {
    bfd_unknown = 0,
    bfd_object,
    bfd_archive,
    bfd_core,
    bfd_type_end,
}

#[link(name = "bfd-2.28-multiarch")]
extern "C" {
    fn bfd_init();

    fn bfd_get_error() -> c_int;
    fn bfd_errmsg(error_tag: c_int) -> *const c_char;

    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *const Bfd;

    fn bfd_check_format(bfd: *const Bfd, bfd_format: BfdFormat) -> bool;

    fn bfd_get_section_by_name(bfd: *const Bfd, name: *const c_char) -> *const Section;

/*
 * binutils 2.29.1
    fn bfd_get_arch(bfd: *const c_int) -> c_int;
    fn bfd_get_mach(bfd: *const c_int) -> c_long;
*/
}


// libopcodes bindings
enum DisassembleInfo {}

#[link(name = "opcodes-2.28-multiarch")]
extern "C" {
    fn disassembler(
        bfd: *const Bfd,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfo) -> c_ulong;
    /*
     * binutils 2.29.1
    fn disassembler(arc: c_int, big: bool, mach: c_long, bfd: *const c_int) -> *const c_int;
    */
    fn disassemble_init_for_target(dinfo: *const DisassembleInfo);
}


// Custom bindings that ease disassembler_info manipulation
extern "C" {
    fn new_disassemble_info() -> *const DisassembleInfo;
    fn configure_disassemble_info(
        info: *const DisassembleInfo,
        section: *const Section,
        bfd: *const Bfd,
    );
    fn get_start_address(bfd: *const Bfd) -> c_ulong;
    fn get_section_size(section: *const Section) -> c_ulong;
    fn set_print_address_func(
        info: *const DisassembleInfo,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfo),
    );

    fn flush_stdout();
}


extern "C" fn override_print_address(addr: c_ulong, _info: *const DisassembleInfo) {
    unsafe { flush_stdout() };
    print!("0x{:x}", addr);
    let _ = std::io::stdout().flush();
}


fn main() {
    let filename = CString::new("/bin/ls").unwrap();
    let target = CString::new("elf64-x86-64").unwrap();

    unsafe { bfd_init() };

    let bfd_file = unsafe { bfd_openr(filename.as_ptr(), target.as_ptr()) };
    if bfd_file.is_null() {
        let error = unsafe { bfd_get_error() };
        let msg = unsafe { bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    if !unsafe { bfd_check_format(bfd_file, BfdFormat::bfd_object) } {
        let error = unsafe { bfd_get_error() };
        let msg = unsafe { bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    // Retrieve the .text code section
    let section_name = CString::new(".text").unwrap();
    let section = unsafe { bfd_get_section_by_name(bfd_file, section_name.as_ptr()) };
    if section.is_null() {
        println!("Error accessing .text section");
        return;
    }

    // Construct disassembler_ftype class
    let disassemble = unsafe { disassembler(bfd_file) };
    if (disassemble as *const c_int).is_null() {
        println!("Error creating disassembler");
        return;
    }

    // Make a disassemble_info structure
    let info = unsafe { new_disassemble_info() };
    if info.is_null() {
        println!("Error while getting disassemble_info!");
        return;
    }

    // Configure the disassemble_info structure
    unsafe { configure_disassemble_info(info, section, bfd_file) };
    unsafe { set_print_address_func(info, override_print_address) };
    unsafe { disassemble_init_for_target(info) };

    // Disassemble the binary
    let mut pc = unsafe { get_start_address(bfd_file) };
    let section_size = unsafe { get_section_size(section) };

    loop {
        print!("0x{:x}  ", pc);
        let _ = std::io::stdout().flush();
        let count = disassemble(pc, info);
        pc += count;
        unsafe { flush_stdout() };
        println!("");
        if !(count > 0 && pc <= section_size) {
            break;
        }
    }
}
