use super::MaterialFlag;
use crate::{ffi, vtable_validate};
use core::mem::MaybeUninit;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    name: unsafe extern "thiscall" fn(this: *const Material) -> *const u8,
    texture_group: unsafe extern "thiscall" fn(this: *const Material) -> *const u8,
    _pad0: vtable::Pad<25>,
    set_alpha: unsafe extern "thiscall" fn(this: *const Material, alpha: f32),
    set_rgb: unsafe extern "thiscall" fn(this: *const Material, red: f32, green: f32, blue: f32),
    set_flag: unsafe extern "thiscall" fn(this: *const Material, flag: MaterialFlag, enabled: bool),
    _pad1: vtable::Pad<14>,
    alpha: unsafe extern "thiscall" fn(this: *const Material) -> f32,
    rgb: unsafe extern "thiscall" fn(
        this: *const Material,
        red: *mut f32,
        green: *mut f32,
        blue: *mut f32,
    ),
}

vtable_validate! {
    name => 0,
    texture_group => 1,
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
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            let ptr = (self.vtable.name)(self);

            ffi::str_from_ptr(ptr)
        }
    }

    #[inline]
    pub fn texture_group(&self) -> &str {
        unsafe {
            let ptr = (self.vtable.texture_group)(self);

            ffi::str_from_ptr(ptr)
        }
    }

    #[inline]
    pub fn set_rgb(&self, rgb: [f32; 3]) {
        let [r, g, b] = rgb;

        unsafe { (self.vtable.set_rgb)(self, r, g, b) }
    }

    #[inline]
    pub fn set_alpha(&self, alpha: f32) {
        unsafe { (self.vtable.set_alpha)(self, alpha) }
    }

    #[inline]
    pub fn set_rgba(&self, rgb: [f32; 4]) {
        let [r, g, b, a] = rgb;

        self.set_rgb([r, g, b]);
        self.set_alpha(a);
    }

    #[inline]
    pub fn rgb(&self) -> [f32; 3] {
        let mut rgb = MaybeUninit::uninit_array();

        unsafe {
            let ptr = rgb.as_mut_ptr().cast::<f32>();

            (self.vtable.rgb)(self, ptr, ptr.add(1), ptr.add(2));

            MaybeUninit::array_assume_init(rgb)
        }
    }

    #[inline]
    pub fn set_flag(&self, flag: MaterialFlag, enabled: bool) {
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
}
