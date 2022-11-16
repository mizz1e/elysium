use crate::{Ptr, TextureGroup};
use bevy::prelude::*;
use std::ffi;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;

const ENV_TINT_MAP: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"$envtintmap\0") };

pub struct IMaterial {
    pub(crate) ptr: Ptr,
}

pub struct IMaterialVar {
    pub(crate) ptr: Ptr,
}

impl IMaterial {
    #[inline]
    pub fn name(&self) -> Box<OsStr> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(0) };

        let name = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!name.is_null());

        let name = unsafe { CStr::from_ptr(name).to_bytes() };

        Box::from(OsStr::from_bytes(name))
    }

    #[inline]
    pub fn texture_group(&self) -> TextureGroup {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(1) };

        let name = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!name.is_null());

        let name = unsafe { CStr::from_ptr(name).to_bytes() };

        TextureGroup::from_bytes(name)
    }

    #[inline]
    pub fn var(&self, name: &CStr) -> Option<IMaterialVar> {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            name: *const ffi::c_char,
            found: *mut bool,
            complain: *const bool,
        ) -> *mut u8 = unsafe { self.ptr.vtable_entry(1) };

        let mut found = false;

        let material_var = unsafe {
            let complain = false;

            (method)(self.ptr.as_ptr(), name.as_ptr(), &mut found, &complain)
        };

        let ptr = unsafe { Ptr::new("IMaterialVar", material_var)? };

        found.then(|| IMaterialVar { ptr })
    }

    #[inline]
    pub fn color_modulate(&self, color: Color) {
        let [r, g, b, a] = color.as_rgba_f32();

        let method: unsafe extern "C" fn(this: *mut u8, a: f32) =
            unsafe { self.ptr.vtable_entry(27) };

        unsafe {
            (method)(self.ptr.as_ptr(), a);
        }

        let method: unsafe extern "C" fn(this: *mut u8, r: f32, g: f32, b: f32) =
            unsafe { self.ptr.vtable_entry(28) };

        unsafe {
            (method)(self.ptr.as_ptr(), r, g, b);
        }
    }

    #[inline]
    pub fn set_tint(&self, color: Color) -> bool {
        if let Some(var) = self.var(ENV_TINT_MAP) {
            let [r, g, b, _a] = color.as_rgba_f32();

            var.set_vec3(Vec3::new(r, g, b));

            true
        } else {
            false
        }
    }

    #[inline]
    pub fn set_flag(&self, flag: MaterialVarFlags_t, enabled: bool) {
        let method: unsafe extern "C" fn(this: *mut u8, flag: MaterialVarFlags_t, enabled: bool) =
            unsafe { self.ptr.vtable_entry(29) };

        unsafe {
            (method)(self.ptr.as_ptr(), flag, enabled);
        }
    }
}

impl IMaterialVar {
    #[inline]
    pub fn set_vec3(&self, vec: Vec3) {
        let method: unsafe extern "C" fn(this: *mut u8, x: f32, y: f32, z: f32) =
            unsafe { self.ptr.vtable_entry(12) };

        let [x, y, z] = vec.to_array();

        unsafe { (method)(self.ptr.as_ptr(), x, y, z) }
    }
}
