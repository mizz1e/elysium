use crate::{ffi, vtable_validate, Vdf};
use cake::ffi::VTablePad;
use core::ptr;
use std::ffi::OsStr;

pub use flag::MaterialFlag;
pub use group::Group;
pub use kind::MaterialKind;
pub use material::Material;
pub use var::Var;

mod flag;
mod group;
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
    ) -> Option<&'static mut Material>,
    find: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        name: *const libc::c_char,
        group: *const libc::c_char,
        complain: bool,
        complain_prefix: *const libc::c_char,
    ) -> Option<&'static mut Material>,
    _pad1: VTablePad<1>,
    first: unsafe extern "thiscall" fn(this: *const MaterialSystem) -> u16,
    next: unsafe extern "thiscall" fn(this: *const MaterialSystem, index: u16) -> u16,
    invalid: unsafe extern "thiscall" fn(this: *const MaterialSystem) -> u16,
    get: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        index: u16,
    ) -> Option<&'static mut Material>,
}

vtable_validate! {
    create => 83,
    find => 84,
    first => 86,
    next => 87,
    invalid => 88,
    get => 89,
}

#[repr(C)]
pub struct MaterialSystem {
    vtable: &'static VTable,
}

impl MaterialSystem {
    #[inline]
    pub fn from_vdf<S>(&self, name: S, vdf: Option<&Vdf>) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
    {
        let vdf = match vdf {
            Some(vdf) => vdf as *const Vdf,
            None => ptr::null(),
        };

        ffi::with_cstr_os_str(name, |name| unsafe {
            (self.vtable.create)(self, name.as_ptr(), vdf)
        })
    }

    #[inline]
    pub fn from_bytes<S, T, U>(
        &self,
        name: S,
        base: T,
        vdf: Option<U>,
    ) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
        U: AsRef<OsStr>,
    {
        let vdf = Vdf::from_bytes::<T, U>(base, vdf);

        self.from_vdf(name, vdf)
    }

    #[inline]
    pub fn from_kind(&self, kind: MaterialKind) -> Option<&'static mut Material> {
        let name = kind.name();
        let base = kind.base();
        let vdf = kind.vdf();

        self.from_bytes(name, base, vdf)
    }

    #[inline]
    pub fn find<S, T>(&self, name: S, group: T) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(name, |name| {
            ffi::with_cstr_os_str(group, |group| unsafe {
                (self.vtable.find)(self, name.as_ptr(), group.as_ptr(), true, ptr::null())
            })
        })
    }

    #[inline]
    pub fn iter(&self) -> MaterialIter<'_> {
        MaterialIter {
            interface: self,
            index: self.first(),
        }
    }

    #[inline]
    fn first(&self) -> u16 {
        unsafe { (self.vtable.first)(self) }
    }

    #[inline]
    fn next(&self, index: u16) -> u16 {
        unsafe { (self.vtable.next)(self, index) }
    }

    #[inline]
    fn invalid(&self) -> u16 {
        unsafe { (self.vtable.invalid)(self) }
    }

    #[inline]
    fn get(&self, index: u16) -> Option<&'static mut Material> {
        unsafe { (self.vtable.get)(self, index) }
    }
}

pub struct MaterialIter<'a> {
    interface: &'a MaterialSystem,
    index: u16,
}

impl<'a> Iterator for MaterialIter<'a> {
    type Item = &'static mut Material;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index != self.interface.invalid() {
            let index = self.index;

            self.index = self.interface.next(index);

            let material = self.interface.get(index);

            match material {
                Some(material) => return Some(material),
                None => continue,
            }
        }

        None
    }
}
