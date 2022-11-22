use crate::{Color, KeyValues, Ptr, TextureGroup};
use bevy::prelude::*;
use std::ffi;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;

const ENV_MAP_TINT: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"$envmaptint\0") };

#[derive(Resource)]
pub struct IMaterialSystem {
    pub(crate) ptr: Ptr,
}

pub struct IMaterial {
    pub(crate) ptr: Ptr,
}

pub struct IMaterialVar {
    pub(crate) ptr: Ptr,
}

#[derive(Resource)]
pub struct Glow(pub IMaterial);

bitflags::bitflags! {
    /// Material flag.
    ///
    /// Used with `pIMaterial->SetFlag`.
    ///
    /// See [`public/materialsystem/imaterial.h`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/materialsystem/imaterial.h).
    #[repr(transparent)]
    pub struct MaterialFlag: i32 {
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

impl IMaterialSystem {
    pub fn create(&self, name: impl AsRef<OsStr>, keyvalues: &KeyValues) -> Option<IMaterial> {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            name: *const ffi::c_char,
            keyvalues: *mut u8,
        ) -> *mut u8 = unsafe { self.ptr.vtable_entry(83) };

        let name = name.as_ref().as_bytes();
        let name = CString::new(name).ok()?;
        let ptr = unsafe { (method)(self.ptr.as_ptr(), name.as_ptr(), keyvalues.ptr.as_ptr()) };
        let ptr = Ptr::new("IMaterial", ptr)?;

        Some(IMaterial { ptr })
    }
}

impl IMaterial {
    pub fn name(&self) -> Box<OsStr> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(0) };

        let name = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!name.is_null());

        let name = unsafe { CStr::from_ptr(name).to_bytes() };

        Box::from(OsStr::from_bytes(name))
    }

    pub fn texture_group(&self) -> TextureGroup {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(1) };

        let name = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!name.is_null());

        let name = unsafe { CStr::from_ptr(name).to_bytes() };

        TextureGroup::from_bytes(name)
    }

    pub fn var(&self, name: &CStr) -> Option<IMaterialVar> {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            name: *const ffi::c_char,
            found: *mut bool,
            complain: *const bool,
        ) -> *mut u8 = unsafe { self.ptr.vtable_entry(11) };

        let mut found = false;

        let material_var = unsafe {
            let complain = false;

            (method)(self.ptr.as_ptr(), name.as_ptr(), &mut found, &complain)
        };

        let ptr = Ptr::new("IMaterialVar", material_var)?;

        found.then(|| IMaterialVar { ptr })
    }

    fn alpha_modulate(&self, alpha: f32) {
        let method: unsafe extern "C" fn(this: *mut u8, alpha: f32) =
            unsafe { self.ptr.vtable_entry(27) };

        unsafe {
            (method)(self.ptr.as_ptr(), alpha);
        }
    }

    fn color_modulate(&self, red: f32, green: f32, blue: f32) {
        let method: unsafe extern "C" fn(this: *mut u8, red: f32, green: f32, blue: f32) =
            unsafe { self.ptr.vtable_entry(28) };

        unsafe {
            (method)(self.ptr.as_ptr(), 1.0, 0.0, 0.0);
        }
    }

    pub fn set_color(&self, color: Color) {
        let Color {
            red,
            green,
            blue,
            alpha,
        } = color;

        self.color_modulate(red, green, blue);
        self.alpha_modulate(alpha);
        self.set_tint(color);
    }

    pub fn set_tint(&self, color: Color) {
        let Color {
            red, green, blue, ..
        } = color;

        if let Some(var) = self.var(ENV_MAP_TINT) {
            var.set_vec3(Vec3::new(red, green, blue));
        }
    }

    pub fn set_flag(&self, flag: MaterialFlag, enabled: bool) {
        let method: unsafe extern "C" fn(this: *mut u8, flag: MaterialFlag, enabled: bool) =
            unsafe { self.ptr.vtable_entry(29) };

        unsafe {
            (method)(self.ptr.as_ptr(), flag, enabled);
        }
    }
}

impl IMaterialVar {
    pub fn set_vec3(&self, vec: Vec3) {
        let method: unsafe extern "C" fn(this: *mut u8, x: f32, y: f32, z: f32) =
            unsafe { self.ptr.vtable_entry(12) };

        let [x, y, z] = vec.to_array();

        unsafe { (method)(self.ptr.as_ptr(), x, y, z) }
    }
}
