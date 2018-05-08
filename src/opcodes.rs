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

    pub fn configure(&self, section: Section, bfd: Bfd) -> Result<(), Error> {
        if self.info.is_null() {
            return Err(Error::DisassembleInfoError(
                "info pointer is null!".to_string(),
            ));
        }
        if section.raw().is_null() {
            return Err(Error::DisassembleInfoError(
                "section pointer is null!".to_string(),
            ));
        }
        if bfd.raw().is_null() {
            return Err(Error::DisassembleInfoError(
                "bfd pointer is null!".to_string(),
            ));
        }

        if !unsafe { helpers::configure_disassemble_info(self.info, section.raw(), bfd.raw()) } {
            return Err(Error::DisassembleInfoError(
                "Error while calling configure_disassemble_info() !".to_string(),
            ));
        }

        Ok(())
    }

    pub fn init_buffer(&mut self, buffer: &[u8], bfd: Bfd, offset: u64) -> Result<(), Error> {
        let disassemble_fn = match bfd.raw_disassembler(bfd.arch_mach.0, false, bfd.arch_mach.1) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        self.configure_buffer(bfd.arch_mach.0, bfd.arch_mach.1, buffer, offset)?;
        self.configure_disassembler(disassemble_fn)?;
        self.init()?;

        Ok(())
    }

    pub fn configure_buffer(
        &self,
        arch: c_uint,
        mach: c_ulong,
        buffer: &[u8],
        offset: u64,
    ) -> Result<(), Error> {
        if self.info.is_null() {
            return Err(Error::DisassembleInfoError(
                "info pointer is null!".to_string(),
            ));
        }
        unsafe {
            let ptr = buffer.as_ptr();
            if ptr.is_null() {
                return Err(Error::DisassembleInfoError(
                    "buffer pointer is null!".to_string(),
                ));
            };
            let len = buffer.len();
            if len == 0 {
                return Err(Error::DisassembleInfoError(
                    "buffer lenght is 0!".to_string(),
                ));
            };
            helpers::configure_disassemble_info_buffer(self.info, arch, mach);

            if helpers::set_buffer(self.info, ptr, len as u32, offset).is_null() {
                return Err(Error::DisassembleInfoError(
                    "set_buffer() malloc error!".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn init(&self) -> Result<(), Error> {
        if self.info.is_null() {
            return Err(Error::DisassembleInfoError(
                "info pointer is null!".to_string(),
            ));
        }
        unsafe { disassemble_init_for_target(self.info) };
        Ok(())
    }

    pub fn set_print_address_func(
        &self,
        print_function: extern "C" fn(c_ulong, *const DisassembleInfoRaw),
    ) -> Result<(), Error> {
        if self.info.is_null() {
            return Err(Error::DisassembleInfoError(
                "info pointer is null!".to_string(),
            ));
        }
        unsafe { helpers::set_print_address_func(self.info, print_function) }

        Ok(())
    }

    pub fn configure_disassembler(
        &mut self,
        disassembler: Box<DisassemblerFunction>,
    ) -> Result<(), Error> {
        if self.info.is_null() {
            self.pc = 0;
            self.disassembler = None;
            return Err(Error::DisassembleInfoError(
                "info pointer is null!".to_string(),
            ));
        }

        if unsafe { helpers::get_disassemble_info_section(self.info) }.is_null() {
            return Err(Error::DisassembleInfoError(
                "section pointer is null!".to_string(),
            ));
        }

        self.pc = unsafe { helpers::get_disassemble_info_section_vma(self.info) };
        self.disassembler = Some(disassembler);

        Ok(())
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
        match di.init_buffer(&[0x90], bfd, 0) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };

        let _ = bfd.set_arch_mach("i386:x86-64");
        let _ = di.configure_buffer(bfd.arch_mach.0, bfd.arch_mach.1, &[0x90], 1);

        let disassemble_fn = bfd.raw_disassembler(bfd.arch_mach.0, false, bfd.arch_mach.1)
            .unwrap();
        let _ = di.configure_disassembler(disassemble_fn);
    }

    #[test]
    fn test_configure_null() {
        // Make sure that configure() test null pointers
        use std;
        use bfd;
        use opcodes;
        use section;

        let di = opcodes::DisassembleInfo::new().unwrap();
        assert_ne!(di.info, std::ptr::null());

        let section = section::Section::null();
        match di.configure(section, bfd::Bfd::empty()) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

        let section = section::Section::from_raw(0x2807 as *const section::SectionRaw);
        match di.configure(section.unwrap(), bfd::Bfd::empty()) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

        let mut bfd = bfd::Bfd::empty();
        let _ = bfd.set_arch_mach("i386:x86-64");
        let _ = di.configure_buffer(bfd.arch_mach.0, bfd.arch_mach.1, &[], 0);
    }

    #[test]
    fn test_configure_disassembler() {
        use std;
        use bfd;
        use opcodes;

        let mut di = opcodes::DisassembleInfo::new().unwrap();
        assert_ne!(di.info, std::ptr::null());

        let mut bfd = bfd::Bfd::empty();
        let _ = bfd.set_arch_mach("i386:x86-64");

        let disassemble_fn = bfd.raw_disassembler(bfd.arch_mach.0, false, bfd.arch_mach.1)
            .unwrap();
        let _ = di.configure_disassembler(disassemble_fn);
    }
}
