use super::Property;
use crate::ffi;
use cake::ffi::{BytePad, CUtf8Str};
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Table {
    properties: (*const Property, i32),
    _pad0: BytePad<8>,
    name: *const libc::c_char,
    _pad1: BytePad<2>,
}

impl Table {
    #[inline]
    pub fn name(&self) -> Box<str> {
        unsafe {
            let name = CUtf8Str::from_ptr(self.name).as_str();

            Box::from(name)
        }
    }

    #[inline]
    pub fn properties(&self) -> &[Property] {
        let (data, len) = self.properties;

        unsafe { ffi::slice_from_i32(data, len) }
    }
}

impl fmt::Debug for Table {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Table")
            .field("properties", &self.properties())
            .field("name", &self.name())
            .finish()
    }
}
