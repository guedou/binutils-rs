// Guillaume Valadon <guillaume@valadon.net>
// binutils libopcodes bindings- opcodes.rs

use libc::{c_uchar, c_uint, c_ulong};

use bfd::{Bfd, BfdRaw};
use section::{Section, SectionRaw};
use super::Error;

#[link(name = "opcodes-2.29.1")]
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
}

pub enum DisassembleInfoRaw {}

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
