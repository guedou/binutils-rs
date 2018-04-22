// Guillaume Valadon <guillaume@valadon.net>

use std::ffi::{CStr, CString};

extern crate libc;
use libc::{c_char, c_int, c_ulong};

extern crate binutils;
use binutils::{tmp_buf_asm, tmp_buf_asm_ptr};


extern "C" fn change_address(addr: c_ulong, _info: *const binutils::DisassembleInfoRaw) {
    // Example of C callback that modifies an address used by an instruction

    //let fmt = "foo\0bar"; // TODO: use it for unit tests!

    // Format the address
    let fmt = format!("0x{:x}", addr);
    let fmt_cstring = match CString::new(fmt) {
        Ok(cstr) => cstr,
        // The following call to unwrap is ok as long as the error message does not contain a NUL byte
        Err(msg) => CString::new(format!("{}", msg)).unwrap(),
    };

    // Copy the address to the buffer
    unsafe {

        // Compute the size of the offset from the base address
        let addr_end = tmp_buf_asm_ptr as usize;
        let addr_start = (&tmp_buf_asm as *const u8) as usize;
        let offset = addr_end - addr_start;

        libc::strncat(tmp_buf_asm_ptr, fmt_cstring.as_ptr(), 64 - offset);
    }
}


fn main() {

    /*
     * crate API
    let bfd = Bfd::new();
    bfd.init();
    bfd.openr("/bin/ls","elf-x86-64");
    bfd.check_format(BfdFormat::bfd_object));
    bfd.get_section_by_name(".text");
    let disassemble = bfd.disassembler();
    */

    let filename = CString::new("/bin/ls").unwrap();
    let target = CString::new("elf64-x86-64").unwrap();

    unsafe { binutils::bfd_init() };

    let bfd_file = unsafe { binutils::bfd_openr(filename.as_ptr(), target.as_ptr()) };
    if bfd_file.is_null() {
        let error = unsafe { binutils::bfd_get_error() };
        let msg = unsafe { binutils::bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    if !unsafe { binutils::bfd_check_format(bfd_file, binutils::BfdFormat::bfd_object) } {
        let error = unsafe { binutils::bfd_get_error() };
        let msg = unsafe { binutils::bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    // Retrieve the .text code section
    let section_name = CString::new(".text").unwrap();
    let section = unsafe { binutils::bfd_get_section_by_name(bfd_file, section_name.as_ptr()) };
    if section.is_null() {
        println!("Error accessing .text section!");
        return;
    }

    // Construct disassembler_ftype class
    let disassemble = unsafe { binutils::disassembler(bfd_file) };
    if (disassemble as *const c_int).is_null() {
        println!("Error creating disassembler!");
        return;
    }

    // Create a disassemble_info structure
    let info = binutils::DisassembleInfo::new();
    if info.raw().is_null() {
        println!("Error while getting disassemble_info!");
        return;
    }

    // Configure the disassemble_info structure
    info.configure(section, bfd_file);
    info.set_print_address_func(change_address);
    info.init();

    // Disassemble the binary
    let raw_info = info.raw();
    let mut pc = unsafe { binutils::get_start_address(bfd_file) }; // bfd.get_start_address();
    let section_size = unsafe { binutils::get_section_size(section) }; // bfd.get_section_size();

    loop {
        unsafe {
            // TODO: in disassemble()
            tmp_buf_asm_ptr = tmp_buf_asm.as_ptr() as *mut c_char;
        };
        let count = disassemble(pc, raw_info); // TODO: return an Instruction
        let instruction = binutils::get_instruction();
        /*
        struct Instruction {
            length: u8,
            asm: Vec<u8>,
            dis: String,
        }
        */

        //println!("0x{:x}  {} {}", pc, count, instruction);
        println!("0x{:x}  {}", pc, instruction);

        pc += count;

        if !(count > 0 && pc <= section_size) {
            break;
        }
    }
}
