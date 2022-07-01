use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<12>,
    set_tint: unsafe extern "thiscall" fn(
        var: *const Material,
        r: *const f32,
        g: *const f32,
        b: *const f32,
    ),
}

#[repr(C)]
pub struct Material {
    vtable: &'static VTable,
}

impl Material {
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { (self.vtable.name)(self) }
    }

    #[inline]
    pub fn texture_group(&self) -> &str {
        unsafe { (self.vtable.texture_group)(self) }
    }

    #[inline]
    pub fn set_rgb(&self, rgb: [f32; 3]) {
        unsafe { (self.vtable.set_rgb)(self, r[0], g[1], b[2]) }
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
            (self.vtable.rgb)(
                self,
                rgb.as_mut_ptr(),
                rgb.as_mut_ptr().add(1),
                rgb.as_mut_ptr().add(2),
            );

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
