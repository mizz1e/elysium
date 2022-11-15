use crate::{vtable_validate, Mat4x3};
use cake::ffi::VTablePad;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<5>,
    should_draw: unsafe extern "C" fn(this: *const Renderable) -> bool,
    _pad1: VTablePad<2>,
    get_model: unsafe extern "C" fn(this: *const Renderable) -> *const u8,
    _pad2: VTablePad<4>,
    setup_bones: unsafe extern "C" fn(
        this: *const Renderable,
        bones: *mut Mat4x3,
        len: i32,
        mask: i32,
        time: f32,
    ) -> bool,
}

vtable_validate! {
    should_draw => 5,
    get_model => 8,
    setup_bones => 13,
}

#[repr(C)]
pub struct Renderable {
    vtable: &'static VTable,
}

impl Renderable {
    #[inline]
    pub fn should_draw(&self) -> bool {
        unsafe { (self.vtable.should_draw)(self) }
    }

    #[inline]
    pub fn model(&self) -> *const u8 {
        unsafe { (self.vtable.get_model)(self) }
    }

    #[inline]
    pub fn setup_bones(&self, bones: &mut [Mat4x3], mask: i32, time: f32) -> bool {
        unsafe {
            (self.vtable.setup_bones)(self, bones.as_mut_ptr(), bones.len() as i32, mask, time)
        }
    }
}
