use crate::{ffi, vtable_validate, Vdf};
use cake::ffi::VTablePad;
use core::ptr;
use std::ffi::OsStr;

pub use flag::MaterialFlag;
pub use kind::MaterialKind;
pub use material::Material;
pub use var::Var;

mod flag;
mod kind;
mod material;
mod var;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<83>,
    create: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        name: *const libc::c_char,
        vdf: *const Vdf,
    ) -> *const Material,
    find: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        name: *const libc::c_char,
        group: *const libc::c_char,
        complain: bool,
        complain_prefix: *const libc::c_char,
    ) -> *const Material,
}

vtable_validate! {
    create => 83,
    find => 84,
}

#[repr(C)]
pub struct MaterialSystem {
    vtable: &'static VTable,
}

impl MaterialSystem {
    #[inline]
    pub fn create<S>(&self, name: S, vdf: &Vdf) -> Option<&'static Material>
    where
        S: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(name, |name| unsafe {
            (self.vtable.create)(self, name.as_ptr(), vdf).as_ref()
        })
    }

    #[inline]
    pub fn find<S, T>(&self, name: S, group: T) -> Option<&'static Material>
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(name, |name| {
            ffi::with_cstr_os_str(group, |group| unsafe {
                (self.vtable.find)(self, name.as_ptr(), group.as_ptr(), true, ptr::null()).as_ref()
            })
        })
    }
}
