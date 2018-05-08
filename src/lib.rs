// Guillaume Valadon <guillaume@valadon.net>
// binutils - lib.rs

pub mod bfd;
pub mod helpers;
pub mod instruction;
pub mod mach;
pub mod opcodes;
pub mod section;
pub mod utils;

extern crate libc;

use std::fmt;

// Specific errors
#[derive(Debug)]
pub enum Error {
    BfdError(u32, String),
    DisassembleInfoError(String),
    SectionError(String),
    CommonError(String),
    NulError(std::ffi::NulError),
    Utf8Error(std::str::Utf8Error),
    NullPointerError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BfdError(tag, ref msg) => write!(f, "{} ({})", msg, tag),
            Error::DisassembleInfoError(ref msg) => write!(f, "{}", msg),
            Error::SectionError(ref section) => write!(f, "Can't find '{}' section!", section),
            Error::CommonError(ref msg) => write!(f, "{}", msg),
            Error::NulError(ref error) => write!(f, "{}", error),
            Error::Utf8Error(ref error) => write!(f, "{}", error),
            Error::NullPointerError(ref error) => write!(f, "{}", error),
        }
    }
}

// Needed to use the ? operator on Cstring::new()
impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

// Needed to use the ? operator on Cstr.to_str()
impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8Error(error)
    }
}
