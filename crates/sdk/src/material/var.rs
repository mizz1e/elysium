use cake::ffi::VTablePad;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<12>,
    set_tint: unsafe extern "thiscall" fn(this: *mut Var, r: f32, g: f32, b: f32),
}

#[repr(C)]
pub struct Var {
    vtable: &'static VTable,
}

impl Var {
    #[inline]
    pub fn set_tint(&mut self, rgb: [f32; 3]) {
        let [r, g, b] = rgb;

        unsafe { (self.vtable.set_tint)(self, r, g, b) }
    }
}
