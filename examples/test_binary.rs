// Guillaume Valadon <guillaume@valadon.net>
// binutils - test_binary.rs

use std::ffi::CString;

extern crate libc;
use libc::c_ulong;

extern crate binutils;
use binutils::bfd;
use binutils::helpers;
use binutils::instruction;
use binutils::opcodes::{DisassembleInfo, DisassembleInfoRaw};

extern "C" fn change_address(addr: c_ulong, _info: *const DisassembleInfoRaw) {
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
        let addr_end = helpers::buffer_asm_ptr as usize;
        let addr_start = (&helpers::buffer_asm as *const u8) as usize;
        let offset = addr_end - addr_start;

        libc::strncat(helpers::buffer_asm_ptr, fmt_cstring.as_ptr(), 64 - offset);
    }
}

fn test_ls(max_instructions: Option<u8>) {
    println!("From an ELF");

    let bfd = match bfd::Bfd::openr("/bin/ls", "elf64-x86-64") {
        Ok(b) => b,
        Err(e) => {
            println!("Error with openr() - {}", e);
            return;
        }
    };

    match bfd.check_format(bfd::BfdFormat::bfd_object) {
        Ok(_) => (),
        Err(e) => {
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
    let info = match DisassembleInfo::new() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Configure the disassemble_info structure
    match info.configure(section, bfd) {
        Ok(_) => (),
        Err(e) => {
            println!("Error configure() - {}", e);
            return;
        }
    };
    match info.set_print_address_func(change_address) {
        Ok(_) => (),
        Err(e) => {
            println!("Error set_print_address_func() - {}", e);
            return;
        }
    };
    info.init();

    // Disassemble the binary
    let mut pc = match bfd.get_start_address() {
        Ok(a) => a,
        Err(e) => {
            println!("Error with get_start_address() - {}", e);
            return;
        }
    };
    let section_size = match section.get_size() {
        Ok(a) => a,
        Err(e) => {
            println!("Error with get_size() - {}", e);
            return;
        }
    };
    let mut counter = 0;
    loop {
        let length = disassemble(pc, &info);
        let instruction = match instruction::get_instruction(pc, length) {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        println!("{}", instruction);

        pc += length;
        counter += 1;

        if !(length > 0 && pc <= section_size) {
            break;
        }

        if !max_instructions.is_none() && max_instructions.unwrap() <= counter {
            break;
        }
    }
}

fn main() {
    test_ls(Some(3));
}
