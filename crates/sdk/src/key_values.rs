use crate::{global, Ptr};
use bevy::prelude::*;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{ffi, ptr};

#[derive(Resource)]
pub struct FromString(
    pub(crate)  unsafe extern "C" fn(
        name: *const ffi::c_char,
        string_value: *const ffi::c_char,
        end_of_parse: *const *const ffi::c_char,
    ) -> *mut u8,
);

/// `public/tier1/keyvalues.h`.
#[derive(Resource)]
pub struct KeyValues {
    pub(crate) ptr: Ptr,
}

impl KeyValues {
    #[inline]
    pub fn from_str(name: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Option<Self> {
        let name = name.as_ref().as_bytes();
        let name = CString::new(name).ok()?;

        let value = value.as_ref().as_bytes();
        let value = CString::new(value).ok()?;

        let method = global::with_app(|app| app.world.resource::<FromString>().0);

        let ptr = unsafe { (method)(name.as_ptr(), value.as_ptr(), ptr::null()) };
        let ptr = unsafe { Ptr::new("KeyValues", ptr)? };

        Some(KeyValues { ptr })
    }
}
