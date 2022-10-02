#![allow(dead_code)]

use crate::ffi::with_cstr_os_str;
use crate::{vtable_validate, Vdf};
use cake::ffi::VTablePad;
use crossbeam_utils::atomic::AtomicCell;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::{ffi, ptr};

pub use category::Category;
pub use flag::Flag;
pub use group::Group;
pub use kind::{Kind, Shader};
pub use material::Material;
pub use var::Var;

mod category;
mod flag;
mod group;
mod kind;
mod material;
mod var;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct BorrowedMaterial {
    material: *mut Material,
}

impl BorrowedMaterial {
    pub fn from_mut(material: &'static mut Material) -> Self {
        Self { material }
    }

    pub fn get(&self) -> &'static mut Material {
        unsafe { &mut *self.material }
    }
}

pub mod method {
    use super::{Handle, Material, Materials};
    use crate::Vdf;
    use core::ffi;

    pub type Create = unsafe extern "C" fn(
        this: &Materials,
        name_pointer: *const ffi::c_char,
        vdf: Option<&Vdf>,
    ) -> Option<&'static mut Material>;

    pub type Find = unsafe extern "C" fn(
        this: &Materials,
        name_pointer: *const ffi::c_char,
        group_pointer: *const ffi::c_char,
        complain: bool,
        complain_prefix_pointer: *const ffi::c_char,
    ) -> Option<&'static mut Material>;

    pub type First = unsafe extern "C" fn(this: &Materials) -> Handle;

    pub type Next = unsafe extern "C" fn(this: &Materials, current: Handle) -> Handle;

    pub type Invalid = unsafe extern "C" fn(this: &Materials) -> Handle;

    pub type Get =
        unsafe extern "C" fn(this: &Materials, current: Handle) -> Option<&'static mut Material>;
}

pub type Handle = ffi::c_ushort;

#[repr(C)]
pub struct VTable {
    _pad0: VTablePad<83>,
    create: method::Create,
    find: method::Find,
    _pad1: VTablePad<1>,
    first: method::First,
    next: method::Next,
    invalid: method::Invalid,
    get: method::Get,
}

vtable_validate! {
    create => 83,
    find => 84,
    first => 86,
    next => 87,
    invalid => 88,
    get => 89,
}

static CREATE: AtomicCell<method::Create> = AtomicCell::new(create);
static FIND: AtomicCell<method::Find> = AtomicCell::new(find);

unsafe extern "C" fn create(
    _this: &Materials,
    _name_pointer: *const libc::c_char,
    _vdf: Option<&Vdf>,
) -> Option<&'static mut Material> {
    unimplemented!();
}

unsafe extern "C" fn find(
    _this: &Materials,
    _name_pointer: *const libc::c_char,
    _group_pointer: *const libc::c_char,
    _complain: bool,
    _complain_prefix_pointer: *const libc::c_char,
) -> Option<&'static mut Material> {
    unimplemented!();
}

#[repr(C)]
pub struct Materials {
    vtable: &'static mut VTable,
}

impl Materials {
    #[inline]
    pub unsafe fn hook_create(&mut self, f: method::Create) {
        let addr = ptr::addr_of_mut!(self.vtable.create);

        elysium_mem::unprotect(addr, |addr, prot| {
            addr.write(f);
            prot
        });
    }

    #[inline]
    pub unsafe fn hook_find(&mut self, f: method::Find) {
        let addr = ptr::addr_of_mut!(self.vtable.find);

        elysium_mem::unprotect(addr, |addr, prot| {
            addr.write(f);
            prot
        });
    }

    #[inline]
    pub unsafe fn init(&self) {
        CREATE.store(self.vtable.create);
        FIND.store(self.vtable.find);
    }

    #[inline]
    pub fn from_vdf<S>(&self, name: S, vdf: Option<&Vdf>) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
    {
        with_cstr_os_str(name, |name| unsafe {
            (CREATE.load())(self, name.as_ptr(), vdf)
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
    pub fn from_kind<S>(&self, name: S, kind: Kind) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
    {
        let base = kind.shader().base();
        let vdf = kind.vdf();

        self.from_bytes(name, base, vdf)
    }

    #[inline]
    pub fn find<S>(&self, name: S, group: Option<Group>) -> Option<&'static mut Material>
    where
        S: AsRef<OsStr>,
    {
        with_cstr_os_str(name, |name| match group {
            Some(group) => with_cstr_os_str(OsStr::from_bytes(group.as_bytes()), |group| unsafe {
                (FIND.load())(self, name.as_ptr(), group.as_ptr(), true, ptr::null())
            }),
            None => unsafe { (FIND.load())(self, name.as_ptr(), ptr::null(), true, ptr::null()) },
        })
    }

    #[inline]
    pub fn iter(&self) -> MaterialIter<'_> {
        MaterialIter {
            interface: self,
            index: self.iter_first(),
        }
    }

    #[inline]
    fn iter_first(&self) -> Handle {
        unsafe { (self.vtable.first)(self) }
    }

    #[inline]
    fn iter_next(&self, current: Handle) -> Handle {
        unsafe { (self.vtable.next)(self, current) }
    }

    #[inline]
    fn iter_invalid(&self) -> Handle {
        unsafe { (self.vtable.invalid)(self) }
    }

    #[inline]
    fn iter_get(&self, current: Handle) -> Option<&'static mut Material> {
        unsafe { (self.vtable.get)(self, current) }
    }
}

pub struct MaterialIter<'a> {
    interface: &'a Materials,
    index: Handle,
}

impl<'a> Iterator for MaterialIter<'a> {
    type Item = &'static mut Material;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index != self.interface.iter_invalid() {
            let index = self.index;

            self.index = self.interface.iter_next(index);

            let material = self.interface.iter_get(index);

            match material {
                Some(material) => return Some(material),
                None => continue,
            }
        }

        None
    }
}
