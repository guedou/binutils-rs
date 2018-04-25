// Guillaume Valadon <guillaume@valadon.net>
// binutils - instruction.rs

use std::ffi::CStr;
use std::fmt;

use super::Error;
use bfd::tmp_buf_asm;

#[allow(dead_code)]
pub struct Instruction<'a> {
    length: u64,
    offset: u64,
    opcode: &'a str,
    //bytes: Vec<u8>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x} {}", self.offset, self.opcode)
    }
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

    Ok(Instruction {
        offset: offset,
        length: length,
        opcode: opcode,
    })
}
