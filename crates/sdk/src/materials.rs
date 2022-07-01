use frosting::ffi::vtable;

pub use kind::MaterialKind;
pub use var::Var;

mod kind;
mod material;
mod var;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<83>,
    create: unsafe extern "thiscall" fn(
        this: *const Materials,
        name: *const u8,
        settings: *const u8,
    ) -> *const u8,
    find: unsafe extern "thiscall" fn(
        name: *const u8,
        texture_group: *const u8,
        complain: bool,
        complain_prefix: *const u8,
    ) -> *const u8,
}

#[repr(C)]
pub struct Materials {}

impl Materials {
    // settings is keyvalues
    #[inline]
    pub fn create(&self, name: &str, settings: *const u8) -> *const u8 {}

    #[inline]
    pub fn find(&self, name: &str, texture_group: &str) -> *const u8 {}
}
