// Guillaume Valadon <guillaume@valadon.net>

use std::ffi::CString;

extern crate libc;
use libc::c_ulong;

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

    let bfd = match binutils::Bfd::openr("/bin/ls", "elf64-x86-64") {
        Ok(b) => b,
        Err(e) => {
            println!("Error with openr() - {}", e);
            return;
        }
    };
    
    let error = bfd.check_format(binutils::BfdFormat::bfd_object);
    match error {
        None => (),
        Some(e) => {
            println!("Error with check_format() - {}", e);
            return;
        }
    };

    // Retrieve the .text code section
    let section = match bfd.get_section_by_name(".text") {
        Ok(s) => s,
        Err(e) => {
            println!("Error with get_section_by_name() - {}", e);
            return;
        }
    };

    // Construct disassembler_ftype class
    let disassemble = match bfd.disassembler() {
        Ok(d) => d,
        Err(e) => {
            println!("Error with disassembler() - {}", e);
            return;
        }
    };

    // Create a disassemble_info structure
    let info = match binutils::DisassembleInfo::new() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Configure the disassemble_info structure
    info.configure(section, bfd);
    info.set_print_address_func(change_address);
    info.init();

    // Disassemble the binary
    let mut pc = bfd.get_start_address();
    loop {
        let count = disassemble(pc, info); // TODO: return an Instruction
        let instruction = binutils::get_instruction();
        /*
        struct Instruction {
            length: u8,
            asm: Vec<u8>,
            dis: String,
        }
        impl fmt::Display for Instruction {
        }
        */

        println!("0x{:x}  {}", pc, instruction);

        pc += count;

        if !(count > 0 && pc <= section.get_size()) {
            break;
        }
    }
}
