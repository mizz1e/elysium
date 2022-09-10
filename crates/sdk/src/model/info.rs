use super::{Hdr, Model, ModelRenderInfo};
use crate::{ffi, vtable_validate};
use cake::ffi::{CUtf8Str, VTablePad};
use std::ffi::OsStr;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<2>,
    get: unsafe extern "thiscall" fn(this: *const ModelInfo, index: i32) -> *const Model,
    index_of:
        unsafe extern "thiscall" fn(this: *const ModelInfo, file_name: *const libc::c_char) -> i32,
    name_of: unsafe extern "thiscall" fn(
        this: *const ModelInfo,
        model: *const Model,
    ) -> *const libc::c_char,
    _pad1: VTablePad<13>,
    materials: unsafe extern "thiscall" fn(
        this: *const ModelInfo,
        model: *const Model,
        len: i32,
        materials: *mut *mut u8,
    ),
    _pad2: VTablePad<12>,
    studio: unsafe extern "thiscall" fn(this: *const ModelInfo, model: *const Model) -> *const Hdr,
}

vtable_validate! {
    get => 2,
    index_of => 3,
    name_of => 4,
    materials => 18,
    studio => 31,
}

/// Model info.
#[repr(C)]
pub struct ModelInfo {
    vtable: &'static VTable,
}

impl ModelInfo {
    /// Returns a model at `index`.
    #[inline]
    pub fn get(&self, index: i32) -> Option<&Model> {
        unsafe { (self.vtable.get)(self, index).as_ref() }
    }

    /// Returns the index of a model by it's file name.
    #[inline]
    pub fn index_of<S>(&self, file_name: S) -> i32
    where
        S: AsRef<OsStr>,
    {
        ffi::with_cstr_os_str(file_name, |file_name| unsafe {
            (self.vtable.index_of)(self, file_name.as_ptr())
        })
    }

    /// Returns the name of a model.
    #[inline]
    pub fn name_of(&self, model: &Model) -> Box<str> {
        unsafe {
            let pointer = (self.vtable.name_of)(self, model);
            let name = CUtf8Str::from_ptr(pointer).as_str();

            Box::from(name)
        }
    }

    /// Returns the name of a model from rendering information.
    #[inline]
    pub(crate) fn name_from_info(&self, info: &ModelRenderInfo) -> Option<Box<str>> {
        let model = unsafe { info.model.as_ref()? };
        let name = self.name_of(model);

        Some(name)
    }

    /// Returns the studio model.
    #[inline]
    pub fn studio(&self, model: &Model) -> *const Hdr {
        unsafe { (self.vtable.studio)(self, model) }
    }
}
