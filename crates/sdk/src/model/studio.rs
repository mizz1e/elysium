//! [`studio.h`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/sp/src/public/studio.h)

use std::ffi::{CStr, OsStr};
use std::io::Read;
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStrExt;
use std::{io, mem, slice};

const HEADER_SIZE: usize = mem::size_of::<StudioHeader>();

#[derive(Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug)]
#[repr(C)]
pub struct OffsetSlice {
    pub len: i32,
    pub offset: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct StudioHeader {
    pub id: i32,
    pub version: i32,
    pub checksum: u32,
    pub name: [u8; 64],
    pub len: i32,

    pub eye_origin: Vec3,
    pub illumination_origin: Vec3,
    pub hull_box: BoundingBox,
    pub view_box: BoundingBox,

    pub flags: i32,
    pub bones: OffsetSlice,

    pub bone_controllers: OffsetSlice,
    pub hitbox_sets: OffsetSlice,

    pub local_animations: OffsetSlice,
    pub local_sequences: OffsetSlice,

    pub activity_list_version: i32,
    pub events_indexed: i32,

    pub textures: OffsetSlice,

    pub cdtextures: OffsetSlice,

    pub skin_reference_count: i32,
    pub skin_family_count: i32,
    pub skin_index: i32,

    pub body_parts: OffsetSlice,

    pub local_attachments: OffsetSlice,

    pub local_nodes: OffsetSlice,
    pub local_node_name_index: i32,

    pub flex_descriptors: OffsetSlice,
    pub flex_controllers: OffsetSlice,
    pub flex_rules: OffsetSlice,
    pub kchains: OffsetSlice,
    pub mouths: OffsetSlice,

    pub pose_parameters: OffsetSlice,

    pub surface_prop_index: i32,

    pub key_values_index: i32,
    pub key_values_count: i32,

    pub autoplay_locks: OffsetSlice,

    pub mass: f32,
    pub contents: i32,

    pub include_models: OffsetSlice,

    // originally a pointer
    pub virtual_model: u64,

    pub animation_block_name_index: i32,

    pub animation_blocks: OffsetSlice,

    // originally a pointer
    pub animation_block_model: u64,

    pub bone_table_by_name_index: i32,

    // originally a pointer
    pub vertex_base: u64,

    // originally a pointer
    pub index_base: u64,

    pub directional_light_dot: u8,

    pub root_lod: u8,

    pub allow_root_lod_count: u8,

    pub _unused0: [u8; 5],

    pub flex_controller_ui: OffsetSlice,

    pub _unused1: [u8; 8],

    pub studio_hdr_to_index: i32,

    pub _unused2: [u8; 4],
}

impl StudioHeader {
    #[inline]
    pub fn from_reader<R>(reader: &mut R) -> io::Result<Self>
    where
        R: Read,
    {
        let mut header = MaybeUninit::<StudioHeader>::uninit();
        let bytes = unsafe { slice::from_raw_parts_mut(header.as_mut_ptr().cast(), HEADER_SIZE) };

        reader.read_exact(bytes)?;

        Ok(unsafe { MaybeUninit::assume_init(header) })
    }

    #[inline]
    pub fn name(&self) -> Option<&OsStr> {
        let cstr = CStr::from_bytes_until_nul(&self.name).ok()?;
        let bytes = cstr.to_bytes();
        let os_str = OsStr::from_bytes(bytes);

        Some(os_str)
    }
}
