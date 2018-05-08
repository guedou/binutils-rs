// Guillaume Valadon <guillaume@valadon.net>
// binutils - test_buffer.rs

extern crate binutils;
use binutils::bfd;
use binutils::instruction;
use binutils::instruction::Instruction;
use binutils::opcodes::DisassembleInfo;
use binutils::utils;

fn test_buffer_full(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (full API) - {}", arch_name);

    let mut bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    // Retrive bfd_arch and bfd_mach from the architecture name
    let bfd_arch_mach = match bfd.set_arch_mach(arch_name) {
        Ok(arch_mach) => arch_mach,
        Err(e) => {
            println!("Error with set_arch_mach() - {}", e);
            return;
        }
    };

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
    match info.configure_buffer(bfd_arch_mach.0, bfd_arch_mach.1, &buffer, offset) {
        Ok(_) => (),
        Err(e) => {
            println!("configure_buffer() - {}", e);
            return;
        }
    };
    match info.init() {
        Ok(_) => (),
        Err(e) => {
            println!("Error init() - {}", e);
            return;
        }
    };

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

fn test_buffer_compact(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (compact API) - {}", arch_name);

    let mut bfd = bfd::Bfd::empty();

    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        println!("Unsuported architecture ({})!", arch_name);
        return;
    }

    // Set bfd_arch and bfd_mach from the architecture name
    let _ = bfd.set_arch_mach(arch_name);

    // Create a disassemble_info structure
    let mut info = match DisassembleInfo::new() {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Configure the disassemble_info structure
    match info.init_buffer(&buffer, bfd, offset) {
        Ok(_) => (),
        Err(e) => {
            println!("init_buffer() - {}", e);
            return;
        }
    };

    // Disassemble the buffer
    loop {
        let instruction = match info.disassemble() {
            None => break,
            Some(i) => match i {
                Ok(i) => i,
                Err(e) => {
                    println!("disassemble() - {}", e);
                    break;
                }
            },
        };
        println!("{}", instruction);
    }
}

fn test_buffer_utils(arch_name: &str, buffer: Vec<u8>, offset: u64) {
    println!("---");
    println!("From a buffer (binutils::utils) - {}", arch_name);

    let mut info = match utils::disassemble_buffer(arch_name, &buffer, offset) {
        Ok(i) => i,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Disassemble the buffer
    loop {
        let instruction = match info.disassemble() {
            None => break,
            Some(i) => match i {
                Ok(i) => i,
                Err(e) => {
                    println!("disassemble() - {}", e);
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
    let _ = bfd.set_arch_mach(arch_name);

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
    test_buffer_full("i386:x86-64", vec![0xc3, 0x90, 0x66, 0x90], 0xA00);

    test_buffer_compact(
        "mep",
        vec![
            0x53, 0x53, 0x08, 0xd8, 0x01, 0x00, 0x53, 0x53, 0x30, 0xeb, 0x5b, 0x00
        ],
        0xC00000,
    );

    test_buffer_utils(
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
