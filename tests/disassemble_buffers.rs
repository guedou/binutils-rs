extern crate binutils;
use binutils::utils::disassemble_buffer;
use binutils::opcodes::DisassembleInfo;

#[test]
fn compact_loop() {
    // Prepare the disassembler
    let mut info = disassemble_buffer("i386", &[0xc3, 0x90, 0x66, 0x90], 0x2800)
        .unwrap_or(DisassembleInfo::empty());

    // Iterate over the instructions
    loop {
        match info.disassemble()
            .ok_or(2807)
            .map(|i| Some(i.unwrap()))
            .unwrap_or(None)
        {
            Some(instruction) => println!("{}", instruction),
            None => break,
        }
    }
}
