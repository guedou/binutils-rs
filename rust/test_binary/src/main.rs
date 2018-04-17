// Guillaume Valadon <guillaume@valadon.net>

extern crate libc;
use libc::{c_char, c_int};

use std::ffi::{CStr, CString};

// libbfd bindings
#[link(name = "bfd-2.28-multiarch")]
extern "C" {
    fn bfd_init();

    fn bfd_get_error() -> c_int;
    fn bfd_errmsg(error_tag: c_int) -> *const c_char;

    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *const c_int;

    fn bfd_check_format(bfd: *const c_int, bfd_format: c_int) -> bool;
}

#[allow(non_camel_case_types)] // use the same enum names as libbfd
#[allow(dead_code)] // don't warn that some variants are not used
enum BfdFormat {
    bfd_unknown = 0,
    bfd_object,
    bfd_archive,
    bfd_core,
    bfd_type_end,
}

fn main() {
    let filename = CString::new("/bin/ls").unwrap();
    let target = CString::new("elf64-x86-64").unwrap();

    unsafe { bfd_init() };

    let bfd_file = unsafe { bfd_openr(filename.as_ptr(), target.as_ptr()) };
    if bfd_file.is_null() {
        let error = unsafe { bfd_get_error() };
        let msg = unsafe { bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    if !unsafe { bfd_check_format(bfd_file, BfdFormat::bfd_object as i32) } {
        let error = unsafe { bfd_get_error() };
        let msg = unsafe { bfd_errmsg(error) };
        println!("Error [{}]: {:?}", error, unsafe { CStr::from_ptr(msg) });
        return;
    }

    println!("Work In Progress !");
}
