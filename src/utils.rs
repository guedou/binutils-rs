// Guillaume Valadon <guillaume@valadon.net>
// binutils - utils.rs

use Error;
use bfd::Bfd;
use helpers;
use opcodes::DisassembleInfo;

pub fn disassemble_buffer(
    arch_name: &str,
    buffer: &[u8],
    offset: u64,
) -> Result<DisassembleInfo, Error> {
    // Create a bfd structure
    let mut bfd = Bfd::empty();

    // Check if the architecture is supported
    if !bfd.arch_list().iter().any(|&arch| arch == arch_name) {
        let error = Error::CommonError(format!("Unsupported architecture ({})!", arch_name));
        return Err(error);
    }

    // Set bfd_arch and bfd_mach from the architecture name
    let _ = bfd.set_arch_mach(arch_name);

    // Create a disassemble_info structure
    let mut info = DisassembleInfo::new()?;

    // Configure the disassemble_info structure
    info.init_buffer(buffer, bfd, offset)?;

    Ok(info)
}

pub(crate) fn check_null_pointer<T>(pointer: *const T, message: &str) -> Result<(), Error> {
    if pointer.is_null() {
        Err(Error::NullPointerError(message.to_string()))
    } else {
        Ok(())
    }
}

pub fn opcode_buffer_append(string: &str) {
    unsafe {
        let tmp = match helpers::CURRENT_OPCODE {
            Some(ref o) => o,
            None => "",
        };
        helpers::CURRENT_OPCODE = Some(format!("{}{}", tmp, string));
    }
}
