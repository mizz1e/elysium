use super::{DrawModelState, ModelRenderInfo};
use crate::material::Material;
use crate::{vtable_validate, Mat4x3};
use cake::ffi::VTablePad;
use core::ptr;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<1>,
    override_material: unsafe extern "C" fn(
        this: *const ModelRender,
        material: *const Material,
        override_kind: i32,
        material_index: i32,
    ),
    _pad1: VTablePad<19>,
    draw_model: unsafe extern "C" fn(
        this: *const ModelRender,
        context: *mut u8,
        state: *mut DrawModelState,
        info: *const ModelRenderInfo,
        bone_to_world: *const [Mat4x3; 256],
    ),
}

vtable_validate! {
    override_material => 1,
    draw_model => 21,
}

/// Model renderer.
#[repr(C)]
pub struct ModelRender {
    vtable: &'static VTable,
}

impl ModelRender {
    #[inline]
    fn _override_material(&mut self, material: *const Material) {
        let override_kind = 0;
        let material_index = -1;

        unsafe { (self.vtable.override_material)(self, material, override_kind, material_index) }
    }

    #[inline]
    pub fn override_material(&mut self, material: &Material) {
        self._override_material(material)
    }

    #[inline]
    pub fn reset_material(&mut self) {
        self._override_material(ptr::null())
    }

    #[inline]
    pub fn draw_model(
        &self,
        context: *mut u8,
        state: &mut DrawModelState,
        info: &ModelRenderInfo,
        bone_to_world: &[Mat4x3; 256],
    ) {
        unsafe { (self.vtable.draw_model)(self, context, state, info, bone_to_world) }
    }

    #[inline]
    pub fn draw_model_address(&self) -> *const u8 {
        &self.vtable.draw_model as *const _ as *const u8
    }
}

use crate::{global, Ptr};

pub struct IVModelRender(pub(crate) Ptr);
