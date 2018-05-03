// Guillaume Valadon <guillaume@valadon.net>
// binutils libopcodes bindings- opcodes.rs

use libc::{c_uchar, c_uint, c_ulong};

use bfd::{Bfd, BfdRaw};
use instruction::{get_instruction, Instruction};
use section::{Section, SectionRaw};
use super::Error;

extern "C" {
    pub fn disassembler(
        arc: c_uint,
        big_endian: bool,
        mach: c_ulong,
        bfd: *const BfdRaw,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfoRaw) -> c_ulong;

    fn disassemble_init_for_target(dinfo: *const DisassembleInfoRaw);

    // Custom helpers
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

    fn set_print_address_func(
        info: *const DisassembleInfoRaw,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    );

    fn set_buffer(
        info: *const DisassembleInfoRaw,
        buffer: *const c_uchar,
        length: c_uint,
        vma: c_ulong,
    );

    fn mep_disassemble_info(info: *const DisassembleInfoRaw);

    fn free_disassemble_info(info: *const DisassembleInfoRaw);

    fn get_disassemble_info_section_vma(info: *const DisassembleInfoRaw) -> c_ulong;
}

pub enum DisassembleInfoRaw {}

pub struct DisassembleInfo {
    info: *const DisassembleInfoRaw,
    disassembler: Option<Box<Fn(c_ulong, &DisassembleInfo) -> c_ulong>>,
    pc: u64,
}

impl DisassembleInfo {
    pub fn new() -> Result<DisassembleInfo, Error> {
        let new_info = unsafe { new_disassemble_info() };
        if new_info.is_null() {
            return Err(Error::CommonError(String::from(
                "Error while getting disassemble_info!",
            )));
        }
        Ok(DisassembleInfo {
            info: new_info,
            disassembler: None,
            pc: 0,
        })
    }

    pub fn raw(&self) -> *const DisassembleInfoRaw {
        self.info
    }

    pub fn configure(&self, section: Section, bfd: Bfd) {
        unsafe { configure_disassemble_info(self.info, section.raw(), bfd.raw()) }
    }

    pub fn init_buffer(&mut self, buffer: &Vec<u8>, bfd: Bfd, offset: u64) {
        let disassemble_fn = match bfd.raw_disassembler(bfd.arch_mach.0, false, bfd.arch_mach.1) {
            Ok(d) => d,
            Err(e) => {
                println!("Error with raw_disassembler() - {}", e);
                return;
            }
        };
        self.configure_buffer(bfd.arch_mach.0, bfd.arch_mach.1, buffer, offset);
        self.configure_disassembler(disassemble_fn);
        self.init();
    }

    pub fn configure_buffer(&self, arch: c_uint, mach: c_ulong, buffer: &Vec<u8>, offset: u64) {
        unsafe {
            let ptr = buffer.as_ptr();
            let len = buffer.len();
            configure_disassemble_info_buffer(self.info, arch, mach);
            set_buffer(self.info, ptr, len as u32, offset);
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

    pub fn configure_disassembler(
        &mut self,
        disassembler: Box<Fn(c_ulong, &DisassembleInfo) -> c_ulong>,
    ) {
        self.pc = unsafe { get_disassemble_info_section_vma(self.info) };
        self.disassembler = Some(disassembler)
    }

    pub fn disassemble<'a>(&mut self) -> Option<Result<Instruction<'a>, Error>> {
        let f = match self.disassembler {
            Some(ref f) => f,
            None => {
                return Some(Err(Error::CommonError(
                    "disassembler not configured!".to_string(),
                )))
            }
        };

        let count = f(self.pc, self);
        //TODO: use an unsigned integer !
        if count == 4294967295 {
            return None;
        }
        let instruction = get_instruction(self.pc, count);
        self.pc += count;
        Some(instruction)
    }
}

impl Drop for DisassembleInfo {
    fn drop(&mut self) {
        unsafe {
            free_disassemble_info(self.info);
        };
    }
}
