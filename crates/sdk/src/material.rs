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

bitflags::bitflags! {
    /// Material flag.
    ///
    /// Used with `pIMaterial->SetFlag`.
    ///
    /// See [`public/materialsystem/imaterial.h`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/materialsystem/imaterial.h).
    #[repr(transparent)]
    pub struct MaterialFlag: u32 {
        const DEBUG = 1 << 0;
        const NO_DEBUG_OVERRIDE = 1 << 1;
        const NO_DRAW = 1 << 2;
        const USE_IN_FILL_RATE_MODE = 1 << 3;
        const VERTEX_COLOR = 1 << 4;
        const VERTEX_ALPHA = 1 << 5;
        const SELF_ILLLUM = 1 << 6;
        const ADDITIVE = 1 << 7;
        const ALPHA_TEST = 1 << 8;
        const PSEUDO_TRANSLUCENT = 1 << 9;
        const Z_NEARER = 1 << 10;
        const MODEL = 1 << 11;
        const FLAT = 1 << 12;
        const NO_CULL = 1 << 13;
        const NO_FOG = 1 << 14;
        const IGNORE_Z = 1 << 15;
        const DECAL = 1 << 16;
        const ENV_MAP_SPHERE = 1 << 17;
        const AO_PRE_PASS = 1 << 18;
        const ENV_MAP_CAMERA_SPACE = 1 << 19;
        const BASE_ALPHA_ENV_MAP_MASK = 1 << 20;
        const TRANSLUCENT = 1 << 21;
        const NORMAL_MAP_ALPHA_ENV_MAP_MASK = 1 << 22;
        const NEEDS_SOFTWARE_SKINNING = 1 << 23;
        const OPAQUE_TEXTURE = 1 << 24;
        const MULTIPLY = 1 << 25;
        const SUPPRESS_DECALS = 1 << 26;
        const HALF_LAMBERT = 1 << 27;
        const WIREFRAME = 1 << 28;
        const ALLOW_ALPHA_TO_COVERAGE = 1 << 29;
        const ALPHA_MODIFIED_BY_PROXY = 1 << 30;
        const VERTEX_FOG = 1 << 31;
    }
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

        let ptr = Ptr::new("IMaterialVar", material_var)?;

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
    pub fn set_flag(&self, flag: MaterialFlag, enabled: bool) {
        let method: unsafe extern "C" fn(this: *mut u8, flag: MaterialFlag, enabled: bool) =
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
