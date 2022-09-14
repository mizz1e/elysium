use super::MaterialFlag;
use crate::vtable_validate;
use cake::ffi::CUtf8Str;
use cake::ffi::VTablePad;
use cake::mem::UninitArray;

#[repr(C)]
struct VTable {
    name: unsafe extern "thiscall" fn(this: *const Material) -> *const libc::c_char,
    group: unsafe extern "thiscall" fn(this: *const Material) -> *const libc::c_char,
    _pad0: VTablePad<25>,
    set_alpha: unsafe extern "thiscall" fn(this: *const Material, alpha: f32),
    set_rgb: unsafe extern "thiscall" fn(this: *const Material, red: f32, green: f32, blue: f32),
    set_flag: unsafe extern "thiscall" fn(this: *const Material, flag: MaterialFlag, enabled: bool),
    _pad1: VTablePad<14>,
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
    group => 1,
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
    pub fn name(&self) -> Box<str> {
        unsafe {
            let pointer = (self.vtable.name)(self);
            let name = CUtf8Str::from_ptr(pointer).as_str();

            Box::from(name)
        }
    }

    /// Material texture group.
    #[inline]
    pub fn group(&self) -> Box<str> {
        unsafe {
            let pointer = (self.vtable.group)(self);
            let group = CUtf8Str::from_ptr(pointer).as_str();

            Box::from(group)
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
        let mut rgb = UninitArray::uninit();
        let [r, g, b] = UninitArray::each_mut_ptr(&mut rgb);

        unsafe {
            (self.vtable.rgb)(self, r, g, b);

            UninitArray::assume_init(rgb)
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
