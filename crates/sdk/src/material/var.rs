use cake::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<12>,
    set_tint:
        unsafe extern "thiscall" fn(var: *const Var, r: *const f32, g: *const f32, b: *const f32),
}

#[repr(C)]
pub struct Var {
    vtable: &'static VTable,
}

impl Var {
    #[inline]
    pub fn set_tint(&self, rgb: [f32; 3]) {
        let [r, g, b] = rgb.each_ref();

        unsafe { (self.vtable.set_tint)(self, r, g, b) }
    }
}
