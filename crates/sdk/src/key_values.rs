use {
    crate::{global, Ptr},
    bevy::prelude::*,
    bevy_source_internal::pattern,
    std::{
        ffi::{self, CString, OsStr},
        mem,
        os::unix::ffi::OsStrExt,
        ptr,
    },
};

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
    pub fn from_str(name: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Option<Self> {
        let name = name.as_ref().as_bytes();
        let name = CString::new(name).ok()?;

        let value = value.as_ref().as_bytes();
        let value = CString::new(value).ok()?;

        let method = global::with_resource::<FromString, _>(|method| method.0);

        let ptr = unsafe { (method)(name.as_ptr(), value.as_ptr(), ptr::null()) };
        let ptr = Ptr::new("KeyValues", ptr)?;

        Some(KeyValues { ptr })
    }

    pub unsafe fn setup() {
        let module = link::load_module("client_client.so").unwrap();
        let bytes = module.bytes();

        let index = pattern!(r"\xE8....\x48\x89\xDF\x48\x89\x45\xE0")
            .find(bytes)
            .unwrap()
            .start();

        let opcode = &bytes[index..(index + 5)];
        let ip = opcode.as_ptr().byte_add(1);
        let reladdr = ip.cast::<i32>().read() as isize;
        let absaddr = ip.byte_add(4).byte_offset(reladdr);
        let from_string = mem::transmute(absaddr);

        global::with_app_mut(|app| {
            app.insert_resource(FromString(from_string));
        });
    }
}
