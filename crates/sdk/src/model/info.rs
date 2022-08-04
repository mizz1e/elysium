use super::{Hdr, Model};
use crate::ffi;
use crate::vtable_validate;
use cake::ffi::VTablePad;
use std::ffi::OsStr;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<2>,
    model: unsafe extern "thiscall" fn(this: *const ModelInfo, index: i32) -> *const u8,
    model_index: unsafe extern "thiscall" fn(this: *const ModelInfo, file_name: *const u8) -> i32,
    model_name: unsafe extern "thiscall" fn(this: *const ModelInfo, model: *const u8) -> *const u8,
    _pad1: VTablePad<13>,
    model_materials: unsafe extern "thiscall" fn(
        this: *const ModelInfo,
        model: *const u8,
        len: i32,
        materials: *mut *mut u8,
    ),
    _pad2: VTablePad<12>,
    studio_model:
        unsafe extern "thiscall" fn(this: *const ModelInfo, model: *const u8) -> *const u8,
}

vtable_validate! {
    model => 2,
    model_index => 3,
    model_name => 4,
    model_materials => 18,
    studio_model => 31,
}

/// Model info.
#[repr(C)]
pub struct ModelInfo {
    vtable: &'static VTable,
}

impl ModelInfo {
    pub fn model(&self, index: i32) -> *const u8 {
        unsafe { (self.vtable.model)(self, index) }
    }

    pub fn model_index<S>(&self, file_name: S) -> i32
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(file_name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.model_index)(self, ptr) }
    }

    pub fn model_name(&self, model: &Model) -> &str {
        let model = <*const Model>::cast(model);

        unsafe {
            let ptr = (self.vtable.model_name)(self, model);

            ffi::str_from_ptr(ptr)
        }
    }

    pub fn studio_model(&self, model: &Model) -> *const Hdr {
        let model = <*const Model>::cast(model);

        unsafe { (self.vtable.studio_model)(self, model).cast() }
    }
}
