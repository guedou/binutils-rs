// Guillaume Valadon <guillaume@valadon.net>
// binutils - section.rs

use libc::c_ulong;

extern "C" {
    fn get_section_size(section: *const SectionRaw) -> c_ulong;
}

pub enum SectionRaw {}

#[derive(Clone, Copy)]
pub struct Section {
    section: *const SectionRaw,
}

impl Section {
    pub fn raw(&self) -> *const SectionRaw {
        self.section
    }

    pub fn from_raw(section_raw: *const SectionRaw) -> Section {
        Section {
            section: section_raw,
        }
    }

    pub fn get_size(&self) -> c_ulong {
        unsafe { get_section_size(self.section) }
    }
}
