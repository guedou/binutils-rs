// Guillaume Valadon <guillaume@valadon.net>
// nutils - utils.rs

use opcodes::DisassembleInfoRaw;

extern "C" {
    pub fn show_buffer(info: *const DisassembleInfoRaw);
}
