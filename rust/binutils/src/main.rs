// Guillaume Valadon <guillaume@valadon.net>
// binutils - main.rs

use std::ffi::CString;

extern crate libc;
use libc::c_ulong;

extern crate binutils;
use binutils::{bfd, tmp_buf_asm, tmp_buf_asm_ptr, mach::bfd_mach};


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


fn test_ls() {
    println!("From an ELF");

    let bfd = match bfd::Bfd::openr("/bin/ls", "elf64-x86-64") {
        Ok(b) => b,
        Err(e) => {
            println!("Error with openr() - {}", e);
            return;
        }
    };

    let error = bfd.check_format(bfd::BfdFormat::bfd_object);
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
        let instruction = match binutils::get_instruction() {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
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
        break; // TODO: remove
    }
}


fn test_buffer(arch_name: &str, mach: u64, buffer: Vec<u8>) {
    println!("---");
    println!("From a buffer - {}", arch_name);

    let bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    let bfd_arch = bfd.scan_arch(arch_name); // TODO: also retrieve mach !

    // Construct disassembler_ftype class
    let disassemble = match bfd.raw_disassembler(bfd_arch, false, mach) {
        Ok(d) => d,
        Err(e) => {
            println!("Error with raw_disassembler() - {}", e);
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
    info.configure_buffer(bfd_arch, mach, buffer);
    info.init();

    // Disassemble the buffer
    let mut pc = 0;
    for _i in 0..3 {
        let count = disassemble(pc, info);
        let instruction = match binutils::get_instruction() {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        println!("0x{:x}  {}", pc, instruction);
        pc += count;
    }
}

fn main() {

    test_ls();

    test_buffer(
        "i386:x86-64",
        bfd_mach::bfd_mach_x86_64 as u64,
        vec![0xc3, 0x90, 0x66, 0x90],
    );

    test_buffer(
        "mep",
        bfd_mach::bfd_mach_mep as u64,
        vec![
            0x53, 0x53, 0x08, 0xd8, 0x01, 0x00, 0x53, 0x53, 0x30, 0xeb, 0x5b, 0x00
        ],
    );
}
