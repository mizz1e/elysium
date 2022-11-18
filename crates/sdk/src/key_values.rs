use crate::{global, pattern, Ptr};
use bevy::prelude::*;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{ffi, mem, ptr};

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

        let method = unsafe { global::with_app(|app| app.world.resource::<FromString>().0) };

        let ptr = unsafe { (method)(name.as_ptr(), value.as_ptr(), ptr::null()) };
        let ptr = Ptr::new("KeyValues", ptr)?;

        Some(KeyValues { ptr })
    }

    #[inline]
    pub unsafe fn setup() {
        tracing::trace!("obtain KeyValues::FromString");

        let module = link::load_module("client_client.so").unwrap();
        let bytes = module.bytes();
        let opcode = &pattern::KEY_VALUES_FROM_STRING.find(bytes).unwrap().1[..5];

        tracing::trace!("KeyValues::FromString = {opcode:02X?}");

        let ip = opcode.as_ptr().byte_add(1);
        let reladdr = ip.cast::<i32>().read() as isize;
        let absaddr = ip.byte_add(4).byte_offset(reladdr);
        let from_string = mem::transmute(absaddr);

        global::with_app_mut(|app| {
            app.insert_resource(FromString(from_string));
        });
    }
}
