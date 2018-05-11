// Guillaume Valadon <guillaume@valadon.net>
// binutils - instruction.rs

use std::fmt;

use Error;
use bfd::Bfd;
use helpers;
use opcodes::DisassembleInfo;

#[allow(dead_code)]
pub struct Instruction<'a> {
    pub length: u64,
    pub offset: u64,
    pub opcode: &'a str,
    info: Option<&'a mut DisassembleInfo>,
    pub error: Option<Error>,
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X} {}", self.offset, self.opcode)
    }
}

pub(crate) fn get_opcode<'a>() -> Result<&'a str, Error> {
    unsafe {
        let ret = match helpers::CURRENT_OPCODE {
            None => Err(Error::DisassembleInfoError("Empty opcode!".to_string())),
            Some(ref opcode) => Ok(opcode.as_str()),
        };
        helpers::CURRENT_OPCODE = None;
        ret
    }
}

pub fn get_instruction<'a>(offset: u64, length: u64) -> Result<Instruction<'a>, Error> {
    Ok(Instruction {
        offset,
        length,
        opcode: get_opcode()?,
        info: None,
        error: None,
    })
}

impl<'a> Instruction<'a> {
    pub fn empty_with_error(error: Option<Error>) -> Instruction<'a> {
        Instruction {
            offset: 0,
            length: 0,
            opcode: "",
            info: None,
            error,
        }
    }
    pub fn from_buffer(
        info: &'a mut DisassembleInfo,
        bfd: Bfd,
        buffer: &[u8],
        offset: u64,
    ) -> Instruction<'a> {
        match info.init_buffer(buffer, bfd, offset) {
            Ok(_) => (),
            Err(e) => return Instruction::empty_with_error(Some(e)),
        };

        Instruction {
            offset: 0,
            length: 0,
            opcode: "",
            info: Some(info),
            error: None,
        }
    }
}

impl<'a> Iterator for Instruction<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Temporarily remove info from the structure
        let info = match self.info.take() {
            Some(i) => i,
            None => {
                return Some(Instruction::empty_with_error(Some(
                    Error::DisassembleInfoError("empty".to_string()),
                )))
            }
        };

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_init() {
        use instruction;

        assert!(instruction::get_opcode().is_err());
        assert!(instruction::get_instruction(0, 0).is_err());
    }

    #[test]
    fn test_iterator() {
        use bfd;
        use instruction;
        use opcodes;

        let mut bfd = bfd::Bfd::empty();
        let _ = bfd.set_arch_mach("i386:x86-x64");

        let mut info = opcodes::DisassembleInfo::new().unwrap();

        let mut instruction = instruction::Instruction::from_buffer(&mut info, bfd, &vec![0x90], 0);
        match instruction.next() {
            Some(i) => assert_eq!(i.opcode, "nop"),
            None => assert!(false),
        };
    }
}
