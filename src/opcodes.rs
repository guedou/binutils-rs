// Guillaume Valadon <guillaume@valadon.net>
// binutils libopcodes bindings - opcodes.rs

use libc::{c_uint, c_ulong};
use std;

use super::Error;
use bfd::{Bfd, BfdRaw};
use helpers;
use instruction::{get_instruction, Instruction};
use section::Section;

extern "C" {
    pub fn disassembler(
        arc: c_uint,
        big_endian: bool,
        mach: c_ulong,
        bfd: *const BfdRaw,
    ) -> extern "C" fn(pc: c_ulong, info: *const DisassembleInfoRaw) -> c_ulong;

    fn disassemble_init_for_target(dinfo: *const DisassembleInfoRaw);
}

pub type DisassemblerFunction = Fn(c_ulong, &DisassembleInfo) -> c_ulong;

pub enum DisassembleInfoRaw {}

pub struct DisassembleInfo {
    info: *const DisassembleInfoRaw,
    disassembler: Option<Box<DisassemblerFunction>>,
    pc: u64,
}

impl DisassembleInfo {
    pub fn empty() -> DisassembleInfo {
        DisassembleInfo {
            info: std::ptr::null(),
            disassembler: None,
            pc: 0,
        }
    }

    pub fn new() -> Result<DisassembleInfo, Error> {
        let new_info = unsafe { helpers::new_disassemble_info() };
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
        if self.info.is_null() {
            return;
        }
        unsafe { helpers::configure_disassemble_info(self.info, section.raw(), bfd.raw()) }
    }

    pub fn init_buffer(&mut self, buffer: &[u8], bfd: Bfd, offset: u64) {
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

    pub fn configure_buffer(&self, arch: c_uint, mach: c_ulong, buffer: &[u8], offset: u64) {
        if self.info.is_null() {
            return;
        }
        unsafe {
            let ptr = buffer.as_ptr();
            let len = buffer.len();
            helpers::configure_disassemble_info_buffer(self.info, arch, mach);
            helpers::set_buffer(self.info, ptr, len as u32, offset);
        }
    }

    pub fn init(&self) {
        if self.info.is_null() {
            return;
        }
        unsafe { disassemble_init_for_target(self.info) };
    }

    pub fn set_print_address_func(
        &self,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    ) {
        if self.info.is_null() {
            return;
        }
        unsafe { helpers::set_print_address_func(self.info, print_function) }
    }

    pub fn configure_disassembler(&mut self, disassembler: Box<DisassemblerFunction>) {
        if self.info.is_null() {
            self.pc = 0;
            self.disassembler = None;
            return;
        }
        self.pc = unsafe { helpers::get_disassemble_info_section_vma(self.info) };
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
        if count == 4_294_967_295 {
            return None;
        }
        let instruction = get_instruction(self.pc, count);
        self.pc += count;
        Some(instruction)
    }
}

impl Drop for DisassembleInfo {
    fn drop(&mut self) {
        if !self.info.is_null() {
            unsafe {
                helpers::free_disassemble_info(self.info);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_di_empty() {
        use std;
        use bfd;
        use opcodes;

        let mut di = opcodes::DisassembleInfo::empty();
        assert_eq!(di.info, std::ptr::null());

        let mut bfd = bfd::Bfd::empty();
        let _ = bfd.set_arch_mach("i386:x86-64");
        di.configure_buffer(bfd.arch_mach.0, bfd.arch_mach.1, &[], 0);

        let disassemble_fn = bfd.raw_disassembler(bfd.arch_mach.0, false, bfd.arch_mach.1)
            .unwrap();
        di.configure_disassembler(disassemble_fn);
    }
}
