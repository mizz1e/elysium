use super::Property;
use cake::ffi::BytePad;
use core::{ffi, fmt, slice};

#[non_exhaustive]
#[repr(C)]
pub struct Table {
    properties: (*const Property, i32),
    _pad0: BytePad<8>,
    name: *const ffi::c_char,
    _pad1: BytePad<2>,
}

impl Table {
    #[inline]
    pub fn name(&self) -> &[u8] {
        unsafe { ffi::CStr::from_ptr(self.name).to_bytes() }
    }

    #[inline]
    pub fn properties(&self) -> &[Property] {
        let (data, len) = self.properties;

        unsafe { slice::from_raw_parts(data, len as usize) }
    }
}

impl fmt::Debug for Table {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Table")
            .field("name", &self.name())
            .field("properties", &self.properties())
            .finish()
    }
}
