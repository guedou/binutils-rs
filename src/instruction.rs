// Guillaume Valadon <guillaume@valadon.net>
// binutils - instruction.rs

use std::ffi::CStr;
use std::fmt;

use Error;
use bfd::Bfd;
use helpers::buffer_asm;
use opcodes::DisassembleInfo;

#[allow(dead_code)]
pub struct Instruction<'a> {
    pub length: u64,
    pub offset: u64,
    pub opcode: &'a str,
    //bytes: Vec<u8>,
    info: Option<&'a mut DisassembleInfo>,
    pub error: Option<Error>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X} {}", self.offset, self.opcode)
    }
}

pub fn get_opcode<'a>() -> Result<&'a str, Error> {
    // Look for the first nul byte in the array
    let mut buffer_itr = unsafe { buffer_asm.iter() };
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
    let opcode_raw = unsafe { CStr::from_bytes_with_nul_unchecked(&buffer_asm[0..index + 1]) };
    Ok(opcode_raw.to_str().unwrap())
}

pub fn get_instruction<'a>(offset: u64, length: u64) -> Result<Instruction<'a>, Error> {
    Ok(Instruction {
        offset: offset,
        length: length,
        opcode: get_opcode().unwrap(),
        info: None,
        error: None,
    })
}

impl<'a> Instruction<'a> {
    pub fn from_buffer(
        info: &'a mut DisassembleInfo,
        bfd: Bfd,
        buffer: &Vec<u8>,
        offset: u64,
    ) -> Instruction<'a> {
        info.init_buffer(buffer, bfd, offset);
        let mut instruction = info.disassemble().unwrap().unwrap(); // TODO: fix it!
        instruction.info = Some(info);
        instruction
    }
}

impl<'a> Iterator for Instruction<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let info = self.info.take().unwrap(); // Remove info for the structure
        let i = info.disassemble();
        match i {
            Some(r) => match r {
                Ok(i) => {
                    self.info = Some(info);
                    Some(i)
                }
                Err(_) => None,
            },
            None => None,
        }
    }
}
