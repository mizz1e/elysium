use crate::app_system::AppSystemVTable;
use crate::{ffi, vtable_validate};
use cake::ffi::VTablePad;
use std::ffi::OsStr;
use std::fmt;

pub use var::{Kind, Var, VarKind, Vars};

mod var;

#[derive(Debug)]
#[repr(C)]
pub struct VTable {
    _pad0: VTablePad<6>,
    var: unsafe extern "C" fn(this: *const Console, name: *const libc::c_char) -> *const (),
    _pad1: VTablePad<11>,
    write: unsafe extern "C" fn(
        this: *mut Console,
        fmt: *const libc::c_char,
        text: *const libc::c_char,
    ),
}

vtable_validate! {
    var => 15,
    write => 27,
}

#[derive(Debug)]
#[repr(C)]
pub struct Console {
    vtable: &'static VTable,
}

impl Console {
    /// Get a config variable.
    #[inline]
    pub fn var<S, T>(&self, name: S) -> Option<&'static Var<T>>
    where
        S: AsRef<OsStr>,
        T: Kind,
    {
        ffi::with_cstr_os_str(name, |name| unsafe {
            (self.vtable.var)(self, name.as_ptr())
                .cast::<Var<T>>()
                .as_ref()
        })
    }

    /// Write to the console.
    #[inline]
    pub fn write(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut buffer = String::new();

        fmt::write(&mut buffer, args)?;

        ffi::with_cstr_os_str(buffer, |buffer| unsafe {
            (self.vtable.write)(self, ffi::const_cstr("%s\0").as_ptr(), buffer.as_ptr());
        });

        Ok(())
    }
}
