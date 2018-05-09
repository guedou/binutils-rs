// Guillaume Valadon <guillaume@valadon.net>
// binutils - utils.rs

use std::cmp;

extern crate libc;
use libc::c_char;

use Error;
use bfd::Bfd;
use helpers;
use opcodes::{DisassembleInfo, DisassembleInfoRaw};

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

pub fn opcode_buffer_append(string: *const c_char) {
    unsafe {
        // Compute the size of the offset from the base address
        let addr_end = helpers::buffer_asm_ptr as usize;
        let addr_start = (&helpers::buffer_asm as *const u8) as usize;
        let offset = addr_end - addr_start;

        if offset == 0 {
            let error_message = "offset is nul!";
            libc::strncat(
                helpers::buffer_asm_ptr,
                error_message.as_ptr() as *const i8,
                error_message.len(),
            );
            return;
        }
        libc::strncat(helpers::buffer_asm_ptr, string, cmp::min(63 - offset, 63));
    }
}
