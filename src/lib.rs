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
