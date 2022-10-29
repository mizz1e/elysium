use elysium_ptr::PtrMut;
use std::ffi::CStr;
use std::{ffi, mem};

use super::{Error, Scope};

/// The `with` parameter shared to the application system linker.
static WITH: Scope<&[(&str, PtrMut<'_, ()>)]> = Scope::new();

/// Application system linker.
unsafe extern "C" fn linker<'b>(
    requested_name: *const ffi::c_char,
    result: *mut ffi::c_int,
) -> Option<PtrMut<'b, ()>> {
    if requested_name.is_null() {
        return None;
    }

    let requested_name = CStr::from_ptr(requested_name);
    let requested_name_bytes = requested_name.to_bytes();

    // `linker` is never called before `WITH` is given a value.
    let with = WITH.get();

    let found = with
        .iter()
        .find(|(interface_name, _interface)| requested_name_bytes == interface_name.as_bytes())
        .map(|(_interface_name, interface)| PtrMut::clone(interface));

    if !result.is_null() {
        result.write_unaligned(found.is_some().into());
    }

    match found {
        Some(interface) => {
            log::info!("Linked {requested_name:?}");

            Some(interface)
        }
        None => {
            log::warn!("Skipped {requested_name:?}");

            None
        }
    }
}

/// Link application system signature.
type Link = for<'a, 'b> unsafe extern "C" fn(
    system: PtrMut<'a, System>,
    linker: unsafe extern "C" fn(
        interface_name: *const ffi::c_char,
        result: *mut ffi::c_int,
    ) -> Option<PtrMut<'b, ()>>,
) -> u8;

/// An application system's virtual table.
#[repr(C)]
struct SystemVTable {
    link: Link,
}

/// An application system.
#[repr(C)]
pub struct System {
    vtable: &'static SystemVTable,
}

impl System {
    /// Link this application system with other interfaces.
    pub unsafe fn link(
        self: PtrMut<'_, System>,
        with: &[(&str, PtrMut<'_, ()>)],
    ) -> Result<(), Error> {
        // Reference to the reference.
        let with = &with;

        // Forge a static lifetime in order to cast to a pointer.
        let static_with = mem::transmute::<
            &'_ &[(&str, PtrMut<'_, ()>)],
            &'static &[(&str, PtrMut<'_, ()>)],
        >(with);

        // `WITH` is guarded behind the exclusive access of `Framework::link`, no other system can link while we're here.
        WITH.set(static_with);

        let result = (self.vtable.link)(self, linker);

        if result == 1 {
            Ok(())
        } else {
            Err(Error::Link)
        }
    }
}
