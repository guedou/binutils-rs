// Guillaume Valadon <guillaume@valadon.net>
// binutils libbfd bindings - bfd.rs

use libc::{c_char, c_uint, c_ulong, uintptr_t};

use std::ffi::{CStr, CString};
use std;

use Error;
use opcodes::{disassembler, DisassembleInfo};
use section::{Section, SectionRaw};

#[link(name = "bfd-2.29.1")]
extern "C" {
    fn bfd_init();

    pub fn bfd_get_error() -> c_uint;

    pub fn bfd_errmsg(error_tag: c_uint) -> *const c_char;

    pub fn bfd_openr(filename: *const c_char, target: *const c_char) -> *const BfdRaw;

    pub fn bfd_check_format(bfd: *const BfdRaw, bfd_format: BfdFormat) -> bool;

    pub fn bfd_get_section_by_name(bfd: *const BfdRaw, name: *const c_char) -> *const SectionRaw;

    fn bfd_arch_list() -> *const uintptr_t;

    fn bfd_scan_arch(string: *const c_char) -> *const c_uint;

    fn bfd_get_arch(bfd: *const BfdRaw) -> c_uint;

    fn bfd_get_mach(bfd: *const BfdRaw) -> c_ulong;

    // Custom bindings
    fn get_start_address(bfd: *const BfdRaw) -> c_ulong;

    fn call_bfd_big_endian(bfd: *const BfdRaw) -> bool;

    fn get_arch(arch_info: *const c_uint) -> u32;

    pub static tmp_buf_asm: [u8; 64];

    pub static mut tmp_buf_asm_ptr: *mut c_char;
}

// Rust bfd types
// Note: - trick from https://doc.rust-lang.org/nomicon/ffi.html
//       - it allows to use the Rust type checker
pub enum BfdRaw {}

#[derive(Clone, Copy)]
pub struct Bfd {
    bfd: *const BfdRaw,
}

impl Bfd {
    pub fn raw(&self) -> *const BfdRaw {
        self.bfd
    }

    pub fn empty() -> Bfd {
        // TODO: check results!
        unsafe { bfd_init() };
        Bfd {
            bfd: std::ptr::null(),
        }
    }

    pub fn openr(filename: &str, target: &str) -> Result<Bfd, Error> {
        // TODO: check results!
        unsafe { bfd_init() };

        let filename_cstring = CString::new(filename).unwrap();
        let target_cstring = CString::new(target).unwrap();

        let bfd_file = unsafe { bfd_openr(filename_cstring.as_ptr(), target_cstring.as_ptr()) };
        if bfd_file.is_null() {
            return Err(bfd_convert_error());
        };
        Ok(Bfd { bfd: bfd_file })
    }

    pub fn check_format(&self, format: BfdFormat) -> Option<Error> {
        // TODO: check results!
        if !unsafe { bfd_check_format(self.bfd, format) } {
            return Some(bfd_convert_error());
        }
        None
    }

    pub fn get_section_by_name(&self, section_name: &str) -> Result<Section, Error> {
        let section_name_cstring = CString::new(section_name).unwrap();
        let section = unsafe { bfd_get_section_by_name(self.bfd, section_name_cstring.as_ptr()) };
        if section.is_null() {
            return Err(Error::SectionError(section_name.to_string()));
        };
        Ok(Section::from_raw(section))
    }

    pub fn disassembler(&self) -> Result<Box<Fn(c_ulong, &DisassembleInfo) -> c_ulong>, Error> {
        let arch = unsafe { bfd_get_arch(self.bfd) };
        let big_endian = self.is_big_endian();
        let mach = unsafe { bfd_get_mach(self.bfd) };
        self.raw_disassembler(arch, big_endian, mach)
    }

    pub fn raw_disassembler(
        &self,
        arch: c_uint,
        big_endian: bool,
        mach: c_ulong,
    ) -> Result<Box<Fn(c_ulong, &DisassembleInfo) -> c_ulong>, Error> {
        let disassemble = unsafe { disassembler(arch, big_endian, mach, self.bfd) };

        if (disassemble as *const c_uint).is_null() {
            return Err(Error::CommonError(String::from(
                "Error creating disassembler!",
            )));
        };

        let disassemble_closure = move |p: c_ulong, di: &DisassembleInfo| -> c_ulong {
            unsafe {
                tmp_buf_asm_ptr = tmp_buf_asm.as_ptr() as *mut c_char; // TODO: not always useful!
            };
            disassemble(p, di.raw())
        };
        Ok(Box::new(disassemble_closure))
    }

    pub fn get_start_address(&self) -> c_ulong {
        unsafe { get_start_address(self.bfd) }
    }

    pub fn is_big_endian(&self) -> bool {
        unsafe { call_bfd_big_endian(self.bfd) }
    }

    pub fn arch_list(&self) -> Vec<&str> {
        let mut ret_vec = Vec::new();
        let mut index = 0;
        let mut stop = false;

        let list = unsafe { bfd_arch_list() };
        loop {
            let slice = unsafe { std::slice::from_raw_parts(list.offset(index), 32) };
            for i in 0..32 {
                if slice[i] == 0 {
                    stop = true;
                    break;
                }
                let arch = unsafe { CStr::from_ptr(slice[i] as *const i8).to_str() };
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

    pub fn scan_arch(&self, arch: &str) -> u32 {
        let arch_cstring = CString::new(arch).unwrap();
        let arch_info = unsafe { bfd_scan_arch(arch_cstring.as_ptr()) };
        unsafe { get_arch(arch_info) }
    }
}

fn bfd_convert_error() -> Error {
    let error = unsafe { bfd_get_error() };
    let msg_char = unsafe { bfd_errmsg(error) };
    let msg_str = unsafe { CStr::from_ptr(msg_char) };
    Error::BfdErr(error, msg_str.to_str().unwrap().to_string())
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
