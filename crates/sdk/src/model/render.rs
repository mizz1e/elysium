use super::{DrawModelState, ModelRenderInfo};
use crate::materials::Material;
use crate::vtable_validate;
use core::ptr;
use elysium_math::Matrix3x4;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<1>,
    override_material: unsafe extern "thiscall" fn(this: *const ModelRender, material: *const u8),
    _pad1: vtable::Pad<19>,
    draw_model: unsafe extern "thiscall" fn(
        this: *const ModelRender,
        context: *const u8,
        state: *const DrawModelState,
        info: *const ModelRenderInfo,
        bone_to_world: *const Matrix3x4,
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
    pub fn override_material(&self, material: &Material) {
        let material = <*const Material>::cast(material);

        unsafe { (self.vtable.override_material)(self, material) }
    }

    pub fn reset_material(&self) {
        unsafe { (self.vtable.override_material)(self, ptr::null()) }
    }

    pub fn draw_model(
        &self,
        context: *const u8,
        state: &DrawModelState,
        info: &ModelRenderInfo,
        bone_to_world: &Matrix3x4,
    ) {
        unsafe { (self.vtable.draw_model)(self, context, state, info, bone_to_world) }
    }

    pub fn draw_model_address(&self) -> *const u8 {
        let draw_model = &self.vtable.draw_model
            as *const unsafe extern "thiscall" fn(
                this: *const ModelRender,
                context: *const u8,
                state: *const DrawModelState,
                info: *const ModelRenderInfo,
                bone_to_world: *const Matrix3x4,
            );

        draw_model.cast()
    }
}
