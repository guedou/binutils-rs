// Guillaume Valadon <guillaume@valadon.net>
// binutils - main.rs

use std::ffi::CString;

extern crate libc;
use libc::c_ulong;

extern crate binutils;
use binutils::bfd;
use binutils::instruction;
use binutils::instruction::Instruction;
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
        let addr_end = bfd::buffer_asm_ptr as usize;
        let addr_start = (&bfd::buffer_asm as *const u8) as usize;
        let offset = addr_end - addr_start;

        libc::strncat(bfd::buffer_asm_ptr, fmt_cstring.as_ptr(), 64 - offset);
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
    let info = match DisassembleInfo::new() {
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

        if !(length > 0 && pc <= section.get_size()) {
            break;
        }

        if !max_instructions.is_none() && max_instructions.unwrap() <= counter {
            break;
        }
    }
}

fn test_buffer_full(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (full API) - {}", arch_name);

    let mut bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    // Retrive bfd_arch and bfd_mach from the architecture name
    let bfd_arch_mach = bfd.scan_arch(arch_name);

    // Construct disassembler_ftype class
    let disassemble = match bfd.raw_disassembler(bfd_arch_mach.0, false, bfd_arch_mach.1) {
        Ok(d) => d,
        Err(e) => {
            println!("Error with raw_disassembler() - {}", e);
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
    info.configure_buffer(bfd_arch_mach.0, bfd_arch_mach.1, &buffer, offset);
    info.init();

    // Disassemble the buffer
    let mut pc = offset;
    for _i in 0..3 {
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
    }
}

fn test_buffer_simplified(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (simplified API) - {}", arch_name);

    let mut bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    // Set bfd_arch and bfd_mach from the architecture name
    bfd.scan_arch(arch_name);

    // Create a disassemble_info structure
    let mut info = match DisassembleInfo::new() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Configure the disassemble_info structure
    info.init_buffer(&buffer, bfd, offset);

    // Disassemble the buffer
    loop {
        let instruction = match info.disassemble() {
            None => break,
            Some(i) => match i {
                Ok(i) => i,
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            },
        };
        println!("{}", instruction);
    }
}

fn test_buffer_iter(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (iter) - {}", arch_name);

    let mut bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    // Set bfd_arch and bfd_mach from the architecture name
    bfd.scan_arch(arch_name);

    // Create a disassemble_info structure
    let mut info = match DisassembleInfo::new() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Disassemble the buffer using an iterator
    for instruction in Instruction::from_buffer(&mut info, bfd, &buffer, offset) {
        println!("{}", instruction);
    }
}

fn main() {
    test_ls(Some(3));

    test_buffer_full("i386:x86-64", vec![0xc3, 0x90, 0x66, 0x90], 0xA00);

    test_buffer_simplified(
        "mep",
        vec![
            0x53, 0x53, 0x08, 0xd8, 0x01, 0x00, 0x53, 0x53, 0x30, 0xeb, 0x5b, 0x00
        ],
        0xC00000,
    );

    test_buffer_iter(
        "mep",
        vec![
            0x53, 0x53, 0x08, 0xd8, 0x01, 0x00, 0x53, 0x53, 0x30, 0xeb, 0x5b, 0x00
        ],
        0xC00000,
    );
}
