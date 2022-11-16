use crate::Mat4x3;
use bevy::math::Vec3;
use cake::ffi::BytePad;

pub use info::ModelInfo;
pub use studio::StudioHeader;

mod info;
mod studio;

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct DrawModelState {
    pub studio: *const (),
    pub hardware_data: *const (),
    pub renderable: *const (),
    pub model_to_world: *const [Mat4x3; 256],
    pub decals: *const (),
    pub draw_flags: i32,
    pub lod: i32,
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Model {
    pub name: [u8; 255],
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct ModelRenderInfo {
    pub origin: Vec3,
    pub angles: Vec3,
    _pad0: BytePad<4>,
    pub renderable: *const *const (),
    pub model: *const Model,
    pub model_to_world: *const [Mat4x3; 256],
    pub lighting_offset: *const Mat4x3,
    pub lighting_origin: *const Vec3,
    pub flags: i32,
    pub entity_index: i32,
    pub skin: i32,
    pub body: i32,
    pub hitboxset: i32,
    pub instance: *const (),
}

impl ModelRenderInfo {
    #[inline]
    pub fn name(&self, model_info: &ModelInfo) -> Option<Box<str>> {
        model_info.name_from_info(self)
    }
}
