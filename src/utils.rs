// Guillaume Valadon <guillaume@valadon.net>
// binutils - utils.rs

use std::ffi::CString;

use libc;

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
        let buffer_as_ptr = helpers::buffer_asm.as_ptr() as *mut i8;

        if libc::strlen(buffer_as_ptr) + string.len() > helpers::BUFFER_MAX_SIZE as usize {
            let message = match CString::new("Can't append to buffer!") {
                Ok(cstr) => cstr,
                // The following call to unwrap is ok as long as the error message does not contain a NUL byte
                Err(msg) => CString::new(format!("{}", msg)).unwrap(),
            };
            libc::strncpy(
                buffer_as_ptr,
                message.as_ptr(),
                helpers::BUFFER_MAX_SIZE as usize,
            );
        } else {
            libc::strncat(buffer_as_ptr, string.as_ptr() as *const i8, string.len());
        }

        // Update the buffer pointer
        let buffer_len = libc::strlen(buffer_as_ptr) as isize;
        helpers::buffer_asm_ptr = buffer_as_ptr.offset(buffer_len as isize);
    }
}
