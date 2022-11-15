use super::{Flag, Group, Var};
use crate::{ffi, vtable_validate};
use cake::ffi::VTablePad;
use cake::ffi::{CBytes, CUtf8Str};
use cake::mem::UninitArray;
use std::ffi::OsStr;

#[repr(C)]
struct VTable {
    name: unsafe extern "C" fn(this: *const Material) -> *const libc::c_char,
    group: unsafe extern "C" fn(this: *const Material) -> *const libc::c_char,
    _pad0: VTablePad<9>,
    var: unsafe extern "C" fn(
        this: *const Material,
        name: *const libc::c_char,
        found: *mut bool,
        complain: bool,
    ) -> *mut Var,
    _pad1: VTablePad<15>,
    set_alpha: unsafe extern "C" fn(this: *const Material, alpha: f32),
    set_rgb: unsafe extern "C" fn(this: *const Material, red: f32, green: f32, blue: f32),
    set_flag: unsafe extern "C" fn(this: *const Material, flag: Flag, enabled: bool),
    _pad2: VTablePad<14>,
    alpha: unsafe extern "C" fn(this: *const Material) -> f32,
    rgb:
        unsafe extern "C" fn(this: *const Material, red: *mut f32, green: *mut f32, blue: *mut f32),
}

vtable_validate! {
    name => 0,
    group => 1,
    var => 11,
    set_alpha => 27,
    set_rgb => 28,
    set_flag => 29,
    alpha => 44,
    rgb => 45,
}

#[repr(C)]
pub struct Material {
    vtable: &'static VTable,
}

impl Material {
    /// Material name.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            let pointer = (self.vtable.name)(self);
            let name = CUtf8Str::from_ptr(pointer).as_str();

            name
        }
    }

    /// Material texture group.
    #[inline]
    pub fn group(&self) -> Group<'_> {
        unsafe {
            let pointer = (self.vtable.group)(self);
            let group = CBytes::from_ptr(pointer).as_bytes();

            Group::from_bytes(group)
        }
    }

    #[inline]
    pub fn set_rgb(&self, rgb: [f32; 3]) {
        let [r, g, b] = rgb;

        unsafe { (self.vtable.set_rgb)(self, r, g, b) }

        if let Some(var) = self.var("$envmaptint\0") {
            var.set_vec3(rgb);
        }
    }

    #[inline]
    pub fn set_alpha(&self, alpha: f32) {
        unsafe { (self.vtable.set_alpha)(self, alpha) }
    }

    #[inline]
    pub fn set_rgba(&self, rgba: [f32; 4]) {
        // already set, don't keep setting it.
        if self.rgba() == rgba {
            return;
        }

        let [r, g, b, a] = rgba;

        self.set_rgb([r, g, b]);
        self.set_alpha(a);
    }

    #[inline]
    pub fn rgb(&self) -> [f32; 3] {
        let mut rgb = UninitArray::uninit();
        let [r, g, b] = UninitArray::each_mut_ptr(&mut rgb);

        unsafe {
            (self.vtable.rgb)(self, r, g, b);

            UninitArray::assume_init(rgb)
        }
    }

    #[inline]
    pub fn set_flag(&self, flag: Flag, enabled: bool) {
        unsafe { (self.vtable.set_flag)(self, flag, enabled) }
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        unsafe { (self.vtable.alpha)(self) }
    }

    #[inline]
    pub fn rgba(&self) -> [f32; 4] {
        let [r, g, b] = self.rgb();
        let a = self.alpha();

        [r, g, b, a]
    }

    #[inline]
    pub fn var<S>(&self, name: S) -> Option<&mut Var>
    where
        S: AsRef<OsStr>,
    {
        let mut exists = false;
        let var = ffi::with_cstr_os_str(name, |name| unsafe {
            (self.vtable.var)(self, name.as_ptr(), &mut exists, true).as_mut()
        });

        exists.then(|| var).flatten()
    }
}
