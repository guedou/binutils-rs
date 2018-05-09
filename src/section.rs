// Guillaume Valadon <guillaume@valadon.net>
// binutils - section.rs

use libc::c_ulong;

use std::ptr;

use Error;
use utils;

extern "C" {
    fn get_section_size(section: *const SectionRaw) -> c_ulong;
}

pub enum SectionRaw {}

#[derive(Clone, Copy)]
pub struct Section {
    pub section: *const SectionRaw,
}

impl Section {
    #[allow(dead_code)]
    pub(crate) fn null() -> Section {
        Section {
            section: ptr::null(),
        }
    }

    pub(crate) fn raw(&self) -> *const SectionRaw {
        self.section
    }

    pub fn from_raw(section_raw: *const SectionRaw) -> Result<Section, Error> {
        utils::check_null_pointer(section_raw, "raw section pointer is null!")?;

        Ok(Section {
            section: section_raw,
        })
    }

    pub fn get_size(&self) -> Result<c_ulong, Error> {
        utils::check_null_pointer(self.section, "section pointer is null!")?;

        Ok(unsafe { get_section_size(self.section) })
    }
}
