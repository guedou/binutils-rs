// Guillaume Valadon <guillaume@valadon.net>
// binutils - lib.rs

pub mod bfd;
pub mod mach;


extern crate libc;

use libc::{c_char, c_uchar, c_uint, c_ulong};

use std::ffi::CStr;
use std::fmt;

use bfd::{Bfd, BfdRaw};


#[derive(Debug)]
pub enum Error {
    BfdErr(u32, String),
    SectionError(String),
    CommonError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::BfdErr(tag, ref msg) => write!(f, "{} ({})", msg, tag),
            &Error::SectionError(ref section) => write!(f, "Can't find '{}' section!", section),
            &Error::CommonError(ref msg) => write!(f, "{}", msg),
        }
    }
}


pub enum SectionRaw {}


#[derive(Clone, Copy)]
pub struct Section {
    section: *const SectionRaw,
}

impl Section {

    pub fn raw(&self) -> *const SectionRaw {
        self.section
    }

    pub fn from_raw(section_raw: *const SectionRaw) -> Section {
        Section {
            section: section_raw,
        }
    }

    pub fn get_size(&self) -> c_ulong {
        unsafe { get_section_size(self.section) }
    }
}

// libopcodes bindings
pub enum DisassembleInfoRaw {}

//#[derive(Clone, Copy)]
pub struct DisassembleInfo {
    info: *const DisassembleInfoRaw,
}

impl DisassembleInfo {
    pub fn new() -> Result<DisassembleInfo, Error> {
        let new_info = unsafe { new_disassemble_info() };
        if new_info.is_null() {
            return Err(Error::CommonError(String::from(
                "Error while getting disassemble_info!",
            )));
        }
        Ok(DisassembleInfo { info: new_info })
    }

    pub fn raw(&self) -> *const DisassembleInfoRaw {
        self.info
    }

    pub fn configure(&self, section: Section, bfd: Bfd) {
        unsafe { configure_disassemble_info(self.info, section.raw(), bfd.raw()) }
    }

    pub fn configure_buffer(&self, arch: c_uint, mach: c_ulong, buffer: &Vec<u8>) {
        unsafe {
            //let new_buffer = buffer; //.to_vec(); // prevent the vector from being freed
            let ptr = buffer.as_ptr();
            let len = buffer.len();
            configure_disassemble_info_buffer(self.info, arch, mach);
            set_buffer(self.info, ptr, len as u32, 0);
            // MeP
            if arch == 60 {
                mep_disassemble_info(self.info);
            }
        }
    }

    pub fn init(&self) {
        unsafe { disassemble_init_for_target(self.info) };
    }

    pub fn set_print_address_func(
        &self,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    ) {
        unsafe { set_print_address_func(self.info, print_function) }
    }
}

impl Drop for DisassembleInfo {
    fn drop(&mut self) {
        unsafe {
            free_disassemble_info(self.info);
        };
    }
}


#[link(name = "opcodes-2.29.1")]
extern "C" {
    pub fn disassembler(
        arc: c_uint,
        big_endian: bool,
        mach: c_ulong,
        bfd: *const BfdRaw,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfoRaw) -> c_ulong;

    fn disassemble_init_for_target(dinfo: *const DisassembleInfoRaw);
}


// Custom bindings that ease disassembler_info manipulation
extern "C" {
    fn new_disassemble_info() -> *const DisassembleInfoRaw;
    fn configure_disassemble_info(
        info: *const DisassembleInfoRaw,
        section: *const SectionRaw,
        bfd: *const BfdRaw,
    );
    fn configure_disassemble_info_buffer(
        info: *const DisassembleInfoRaw,
        arch: c_uint,
        mach: c_ulong,
    );
    fn mep_disassemble_info(info: *const DisassembleInfoRaw);
    fn get_start_address(bfd: *const BfdRaw) -> c_ulong;
    pub fn get_section_size(section: *const SectionRaw) -> c_ulong;
    fn set_print_address_func(
        info: *const DisassembleInfoRaw,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    );
    fn get_arch(arch_info: *const c_uint) -> u32;

    fn set_buffer(
        info: *const DisassembleInfoRaw,
        buffer: *const c_uchar,
        length: c_uint,
        vma: c_ulong,
    );
    pub fn free_disassemble_info(info: *const DisassembleInfoRaw);
    pub fn show_buffer(info: *const DisassembleInfoRaw);

    fn call_bfd_big_endian(bfd: *const BfdRaw) -> bool;

    pub static tmp_buf_asm: [u8; 64];
    pub static mut tmp_buf_asm_ptr: *mut c_char;
}


pub fn get_instruction<'a>(offset: u64, length: u64) -> Result<Instruction<'a>, Error> {
    // Look for the first nul byte in the array
    let mut buffer_itr = unsafe { tmp_buf_asm.iter() };
    let index_opt = buffer_itr.position(|&c| c == 0);

    let index = match index_opt {
        Some(i) => i,
        None => {
            return Err(Error::CommonError(String::from(
                "No nul byte found in disassembly result!",
            )))
        }
    };

    // Extract the instruction string
    let opcode_raw = unsafe { CStr::from_bytes_with_nul_unchecked(&tmp_buf_asm[0..index + 1]) };
    let opcode = opcode_raw.to_str().unwrap();

    Ok(Instruction { offset: offset, length: length, opcode: opcode})
}


pub struct Instruction<'a> {
    length: u64,
    offset: u64,
    opcode: &'a str,
    //bytes: Vec<u8>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x} {}", self. offset, self.opcode)
    }
}
