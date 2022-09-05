use cake::ffi::BytePad;
use core::marker::PhantomData;
use core::ptr;
use elysium_math::{Matrix3x4, Vec3};

pub use info::ModelInfo;
pub use render::ModelRender;

mod info;
mod render;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(i32)]
#[rustfmt::skip] // rustfmt does an ugly
pub enum UsedBy {
    Anything    = 0x00_0F_FF_00,
    Hitbox      = 0x00_00_01_00, // bone (or child) used by a hitbox
    Attachment  = 0x00_00_02_00, // bone (or child) used by an attachment point
    VertexMask  = 0x00_03_FC_00,
    VertexLoD0  = 0x00_00_04_00, // bone (or child) used by the top-level model via skinned vertex
    VertexLoD1  = 0x00_00_08_00,
    VertexLoD2  = 0x00_00_10_00,
    VertexLoD3  = 0x00_00_20_00,
    VertexLoD4  = 0x00_00_40_00,
    VertexLoD5  = 0x00_00_80_00,
    VertexLoD6  = 0x00_01_00_00,
    VertexLoD7  = 0x00_02_00_00,
    BoneMerge   = 0x00_04_00_00, // bone is available for bone merge to occur
    AlwaysSetup = 0x00_08_00_00,
}

#[derive(Debug)]
#[repr(C)]
pub struct MagicArray<T> {
    len: i32,
    offset: i32,
    phantom: PhantomData<T>,
}

impl<T> MagicArray<T> {
    pub const fn len(&self) -> i32 {
        self.len
    }

    pub const fn offset(&self) -> i32 {
        self.offset
    }

    pub unsafe fn get_unchecked(&self, base_address: *const u8, index: i32) -> *const T {
        base_address
            .offset(self.offset as isize)
            .offset(index as isize)
            .cast()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Bone {
    pub name_offset: i32,
    pub parent: i32,
    pub bone_controller: [i32; 6],
    pub position: Vec3,
    pub quaternion: [f32; 4],
    pub rotation: [f32; 3],
    pub position_scale: Vec3,
    pub rotation_scale: Vec3,
    pub position_to_bone: Matrix3x4,
    pub quaternion_alignment: [f32; 4],
    pub flags: i32,
    pub procedural_kind: i32,
    pub procedural_offset: i32,
    pub physics_bone: i32,
    pub surface_prop_offset: i32,
    pub contents: i32,
    pub surface_prop_lookup: i32,
    _pad0: BytePad<28>,
}

impl Bone {
    pub const fn as_ptr(&self) -> *const u8 {
        ptr::addr_of!(self).cast()
    }

    pub unsafe fn name(&self) -> *const i8 {
        self.as_ptr().offset(self.name_offset as isize).cast()
    }

    pub unsafe fn procedural(&self) -> *const () {
        if self.procedural_offset == 0 {
            return ptr::null();
        }

        self.as_ptr().offset(self.procedural_offset as isize).cast()
    }

    pub unsafe fn get_surface_prop(&self) -> *const i8 {
        self.as_ptr()
            .offset(self.surface_prop_offset as isize)
            .cast()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct BoundingBox {
    pub bone: i32,
    pub group: i32,
    pub max: Vec3,
    pub min: Vec3,
    pub hitbox_name_offset: i32,
    _pad0: BytePad<12>,
    pub radius: f32,
    _pad1: BytePad<16>,
}

impl BoundingBox {
    pub const fn as_ptr(&self) -> *const u8 {
        ptr::addr_of!(self).cast()
    }

    pub unsafe fn name(&self) -> *const i8 {
        if self.hitbox_name_offset == 0 {
            return ptr::null();
        }

        self.as_ptr()
            .offset(self.hitbox_name_offset as isize)
            .cast()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct HitboxSet {
    pub name_offset: i32,
    pub hitboxes: MagicArray<BoundingBox>,
}

impl HitboxSet {
    pub const fn as_ptr(&self) -> *const u8 {
        ptr::addr_of!(self).cast()
    }

    pub unsafe fn name(&self) -> *const i8 {
        self.as_ptr().offset(self.name_offset as isize).cast()
    }

    pub unsafe fn hitbox_unchecked(&self, index: i32) -> *const BoundingBox {
        self.hitboxes.get_unchecked(self.as_ptr(), index)
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Hdr {
    pub id: i32,
    pub version: i32,
    pub checksum: i32,
    pub name: [i8; 64],
    pub length: i32,
    pub eye_position: Vec3,
    pub illumination_position: Vec3,
    pub hull_min: Vec3,
    pub hull_max: Vec3,
    pub view_bounding_box_min: Vec3,
    pub view_bounding_box_max: Vec3,
    pub flags: i32,
    pub bones: MagicArray<Bone>,
    pub bone_controllers: MagicArray<()>,
    pub hitbox_sets: MagicArray<HitboxSet>,
    pub local_anims: MagicArray<()>,
    pub local_seqs: MagicArray<()>,
    pub textures: MagicArray<()>,
    pub raw_textures: MagicArray<()>,
    pub replacable_textures: MagicArray<()>,
    pub body_parts: MagicArray<()>,
    pub local_attachments: MagicArray<()>,
    pub local_nodes: MagicArray<()>,
    pub flex_desc: MagicArray<()>,
    pub flex_controllers: MagicArray<()>,
    pub flex_rules: MagicArray<()>,
    pub ik_chains: MagicArray<()>,
    pub mouths: MagicArray<()>,
    pub local_pose_parameters: MagicArray<()>,
    pub surface_pos_offset: i32,
    pub key_values: MagicArray<()>,
    pub local_ik_autoplaylocks: MagicArray<()>,
    pub mass: f32,
    pub contents: i32,
    pub include_models: MagicArray<()>,
    pub virtual_model: *mut (),
    pub animation_block_name_offset: i32,
    pub animation_blocks: MagicArray<()>,
    pub bone_table_by_name_index: i32,
    pub vertex_base: *const (),
    pub index_base: *const (),
    pub constant_directional_light_dot: u8,
    pub root_lod: u8,
    pub allowed_root_lods: u8,
    _pad0: BytePad<5>,
    pub flex_controller_ui: MagicArray<()>,
    _pad1: BytePad<16>,
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct DrawModelState {
    pub studio: *const (),
    pub hardware_data: *const (),
    pub renderable: *const (),
    pub model_to_world: *const Matrix3x4,
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
    pub model_to_world: *const Matrix3x4,
    pub lighting_offset: *const Matrix3x4,
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
