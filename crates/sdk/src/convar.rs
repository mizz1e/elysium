use crate::Ptr;
use bevy::prelude::*;
use std::ffi;
use std::ffi::{CString, OsStr};
use std::marker::PhantomData;
use std::os::unix::ffi::OsStrExt;

/// `public/tier1/convar.h`.
#[derive(Resource)]
pub struct ICvar {
    pub(crate) ptr: Ptr,
}

impl ICvar {
    #[inline]
    pub fn find_var<T>(&self, var_name: impl AsRef<OsStr>) -> Option<ConVar<T>> {
        let var_name = var_name.as_ref().as_bytes();
        let var_name = CString::new(var_name).ok()?;

        let method: unsafe extern "C" fn(this: *mut u8, var_name: *const ffi::c_char) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(15) };

        let convar = unsafe { (method)(self.ptr.as_ptr(), var_name.as_ptr()) };
        let ptr = Ptr::new("ConVar", convar)?;

        let _phantom = PhantomData;

        Some(ConVar { ptr, _phantom })
    }
}

#[derive(Resource)]
pub struct SvCheats(pub(crate) ConVar<i32>);

#[derive(Resource)]
pub struct ConVar<T> {
    pub(crate) ptr: Ptr,
    _phantom: PhantomData<T>,
}

impl ConVar<i32> {
    pub fn read(&self) -> i32 {
        let method: unsafe extern "C" fn(this: *mut u8) -> i32 =
            unsafe { self.ptr.vtable_entry(16) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn write(&self, value: i32) {
        let method: unsafe extern "C" fn(this: *mut u8, value: i32) =
            unsafe { self.ptr.vtable_entry(19) };

        unsafe { (method)(self.ptr.as_ptr(), value) }
    }
}

impl ConVar<f32> {
    pub fn read(&self) -> f32 {
        let method: unsafe extern "C" fn(this: *mut u8) -> f32 =
            unsafe { self.ptr.vtable_entry(15) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn write(&self, value: f32) {
        let method: unsafe extern "C" fn(this: *mut u8, value: f32) =
            unsafe { self.ptr.vtable_entry(18) };

        unsafe { (method)(self.ptr.as_ptr(), value) }
    }
}
