// Guillaume Valadon <guillaume@valadon.net>
// binutils libbfd bindings - bfd.rs

use libc::{c_char, c_uint, c_ulong, uintptr_t};

use std;
use std::ffi::{CStr, CString};

use Error;
use helpers::{buffer_asm, buffer_asm_ptr, get_arch, get_mach, get_start_address,
              macro_bfd_big_endian};
use opcodes::{disassembler, DisassembleInfo, DisassemblerFunction};
use section::{Section, SectionRaw};
use utils;

extern "C" {
    fn bfd_init();

    pub fn bfd_get_error() -> c_uint;

    pub fn bfd_errmsg(error_tag: c_uint) -> *const c_char;

    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *const BfdRaw;

    fn bfd_check_format(bfd: *const BfdRaw, bfd_format: BfdFormat) -> bool;

    fn bfd_get_section_by_name(bfd: *const BfdRaw, name: *const c_char) -> *const SectionRaw;

    fn bfd_arch_list() -> *const uintptr_t;

    fn bfd_scan_arch(string: *const c_char) -> *const c_uint;

    fn bfd_get_arch(bfd: *const BfdRaw) -> c_uint;

    fn bfd_get_mach(bfd: *const BfdRaw) -> c_ulong;
}

// Rust bfd types
// Note: - trick from https://doc.rust-lang.org/nomicon/ffi.html
//       - it allows to use the Rust type checker
pub enum BfdRaw {}

#[derive(Clone, Copy)]
pub struct Bfd {
    bfd: *const BfdRaw,
    pub arch_mach: (u32, u64),
}

impl Bfd {
    pub fn raw(&self) -> *const BfdRaw {
        self.bfd
    }

    pub fn empty() -> Bfd {
        unsafe { bfd_init() };
        Bfd {
            bfd: std::ptr::null(),
            arch_mach: (0, 0),
        }
    }

    pub fn openr(filename: &str, target: &str) -> Result<Bfd, Error> {
        unsafe { bfd_init() };

        let filename_cstring = CString::new(filename)?;
        let target_cstring = CString::new(target)?;

        let bfd = unsafe { bfd_openr(filename_cstring.as_ptr(), target_cstring.as_ptr()) };
        if bfd.is_null() {
            return Err(bfd_convert_error());
        };

        Ok(Bfd {
            bfd,
            arch_mach: (0, 0),
        })
    }

    pub fn check_format(&self, format: BfdFormat) -> Result<(), Error> {
        utils::check_null_pointer(self.bfd, "bfd pointer is null!")?;

        if !unsafe { bfd_check_format(self.bfd, format) } {
            return Err(bfd_convert_error());
        };

        Ok(())
    }

    pub fn get_section_by_name(&self, section_name: &str) -> Result<Section, Error> {
        utils::check_null_pointer(self.bfd, "bfd pointer is null!")?;

        let section_name_cstring = CString::new(section_name)?;

        let section = unsafe { bfd_get_section_by_name(self.bfd, section_name_cstring.as_ptr()) };
        if section.is_null() {
            return Err(Error::SectionError(section_name.to_string()));
        };

        Ok(Section::from_raw(section)?)
    }

    pub fn disassembler(&self) -> Result<Box<DisassemblerFunction>, Error> {
        utils::check_null_pointer(self.bfd, "bfd pointer is null!")?;

        let arch = unsafe { bfd_get_arch(self.bfd) };
        let big_endian = match self.is_big_endian() {
            Ok(be) => be,
            Err(e) => return Err(e),
        };
        let mach = unsafe { bfd_get_mach(self.bfd) };
        self.raw_disassembler(arch, big_endian, mach)
    }

    pub fn raw_disassembler(
        &self,
        arch: c_uint,
        big_endian: bool,
        mach: c_ulong,
    ) -> Result<Box<DisassemblerFunction>, Error> {
        let disassemble = unsafe { disassembler(arch, big_endian, mach, self.bfd) };
        if (disassemble as *const c_uint).is_null() {
            return Err(Error::BfdError(
                0,
                String::from("Error creating disassembler!"),
            ));
        };

        let disassemble_closure = move |p: c_ulong, di: &DisassembleInfo| -> c_ulong {
            // Reset the buffer pointer
            unsafe {
                buffer_asm_ptr = buffer_asm.as_ptr() as *mut c_char;
            };
            disassemble(p, di.raw())
        };

        Ok(Box::new(disassemble_closure))
    }

    pub fn get_start_address(&self) -> Result<c_ulong, Error> {
        utils::check_null_pointer(self.bfd, "bfd pointer is null!")?;

        Ok(unsafe { get_start_address(self.bfd) })
    }

    pub fn is_big_endian(&self) -> Result<bool, Error> {
        utils::check_null_pointer(self.bfd, "bfd pointer is null!")?;

        Ok(unsafe { macro_bfd_big_endian(self.bfd) })
    }

    pub fn arch_list(&self) -> Vec<&str> {
        let mut ret_vec = Vec::new();
        let mut index = 0;
        let mut stop = false;

        let list = unsafe { bfd_arch_list() };
        if list.is_null() {
            return ret_vec;
        }

        loop {
            let slice = unsafe { std::slice::from_raw_parts(list.offset(index), 32) };
            for item in slice.iter().take(32) {
                if *item == 0 {
                    stop = true;
                    break;
                }
                let arch = unsafe { CStr::from_ptr(*item as *const i8).to_str() };
                match arch {
                    Ok(s) => ret_vec.push(s),
                    Err(_) => ret_vec.push("arch_list() - from_ptr() error !"),
                }
            }
            if stop {
                break;
            } else {
                index += 32;
            }
        }

        ret_vec
    }

    pub fn set_arch_mach(&mut self, arch: &str) -> Result<(u32, u64), Error> {
        let arch_cstring = CString::new(arch)?;
        let arch_info = unsafe { bfd_scan_arch(arch_cstring.as_ptr()) };
        if arch_info.is_null() {
            return Err(Error::BfdError(0, "architecture not found!".to_string()));
        };
        self.arch_mach = unsafe { (get_arch(arch_info), get_mach(arch_info)) };
        Ok(self.arch_mach)
    }
}

fn bfd_convert_error() -> Error {
    let error = unsafe { bfd_get_error() };
    let msg_char = unsafe { bfd_errmsg(error) };
    let msg_str = match unsafe { CStr::from_ptr(msg_char).to_str() } {
        Ok(s) => s,
        Err(e) => return Error::Utf8Error(e),
    };
    Error::BfdError(error, msg_str.to_string())
}

#[allow(non_camel_case_types)] // use the same enum names as libbfd
#[allow(dead_code)] // don't warn that some variants are not used
#[repr(C)]
pub enum BfdFormat {
    bfd_unknown = 0,
    bfd_object,
    bfd_archive,
    bfd_core,
    bfd_type_end,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bfd_empty() {
        use std;
        use bfd;

        let bfd = bfd::Bfd::empty();
        assert_eq!(bfd.bfd, std::ptr::null());
        assert_eq!(bfd.arch_mach, (0, 0));

        match bfd.get_start_address() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
        match bfd.is_big_endian() {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn test_bfd_openr() {
        use std;
        use bfd;
        use Error;

        let raw_binary_name = b"bin\0name".to_vec();
        let binary_name = unsafe { std::str::from_utf8_unchecked(&raw_binary_name) };
        match bfd::Bfd::openr(binary_name, "elf64-x86-64") {
            Ok(_) => assert!(false),
            Err(Error::NulError(_)) => assert!(true),
            Err(_) => assert!(false),
        };

        match bfd::Bfd::openr("/bin/ls", "elf64-x86-64") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        };

        match bfd::Bfd::openr("", "") {
            Ok(_) => assert!(false),
            Err(Error::BfdError(_, _)) => assert!(true),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn test_bfd_get_section_bad() {
        use std;
        use bfd;
        use Error;

        let bfd = bfd::Bfd::openr("/bin/ls", "elf64-x86-64").unwrap();
        let raw_section_name = b".\0text".to_vec();
        let section_name = unsafe { std::str::from_utf8_unchecked(&raw_section_name) };
        match bfd.get_section_by_name(section_name) {
            Ok(_) => assert!(true),
            Err(Error::NulError(_)) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
