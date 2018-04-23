// Guillaume Valadon <guillaume@valadon.net>

use std::ffi::CString;
use std::slice;

extern crate libc;
use libc::{c_uchar, c_uint, c_ulong, c_void};

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


fn test_ls() {
    println!("From an ELF");

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


static RUST_BUFFER: [u8; 4] = [0xc3, 0x90, 0x66, 0x90];
static mut RUST_BUFFER_INDEX: usize = 0;

extern "C" fn rust_buffer_to_c(
    _memaddr: c_ulong,
    buffer: *mut c_uchar,
    length: c_uint,
    _info: *const binutils::DisassembleInfoRaw,
) -> c_uint {
    if length > RUST_BUFFER.len() as u32 {
        return 1;
    }

    unsafe {
        libc::memcpy(
            buffer as *mut c_void,
            RUST_BUFFER[RUST_BUFFER_INDEX as usize..RUST_BUFFER_INDEX + length as usize].as_ptr()
                as *const c_void,
            length as usize,
        );
        RUST_BUFFER_INDEX += length as usize;
    }
  
    return 0;
}


fn test_buffer() {
    println!("---");
    println!("From a buffer");

    let bfd = binutils::Bfd::empty();


    unsafe {
        // List available architectures
        let list = binutils::bfd_arch_list();
        let s = slice::from_raw_parts(list, 128);
        let mut i = 0;
        loop {
            if s[i] == 0 {
                break;
            }
            //println!("{:?}", CStr::from_ptr(s[i] as *const i8).to_str());
            i += 1;
        }
        //println!("---");

        // Retrieve the architecture value
        let _arch_info = binutils::bfd_scan_arch(CString::new("i386:x86-64").unwrap().as_ptr());
        //println!("{:?}", arch_info);
        /* It can be used to retrieve the arch: arch_info->arch */

    };
    //  grep bfd_mach bfd.h |grep define
    //  TODO: build a compile time using build.rs ?
    let _bfd_mach_mep = 1;
    let bfd_mach_x86_64 = 1 << 3;

    let bfd_arch_i386 = 9;

    //println!("---");
    // Construct disassembler_ftype class
    let disassemble = match bfd.raw_disassembler(bfd_arch_i386, false, bfd_mach_x86_64) {
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
    // Set the arch and mach members
    // TODO: bfd_set_arch_mach
    info.configure_buffer(bfd_arch_i386, bfd_mach_x86_64, rust_buffer_to_c); // set_arch_mach
    // TODO: info.set_arch_mach_from_bfd(bfd);
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

        // if pc > buffer.len() { break; }
    }

}

fn main() {
    test_ls();
    test_buffer();
}
