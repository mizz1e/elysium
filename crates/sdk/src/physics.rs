use crate::Ptr;
use bevy::prelude::Resource;
use std::ffi;

/// `https://github.com/SteamDatabase/GameTracking-CSGO/blob/master/csgo/scripts/surfaceproperties_cs.txt`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SurfaceKind {
    Antlion = b'A',
    BloodyFlesh = b'B',
    Concrete = b'C',
    Dirt = b'D',
    EggShell = b'E',
    Flesh = b'F',
    Grate = b'G',
    AlienFlesh = b'H',
    Clip = b'I',
    Grass = b'J',
    Snow = b'K',
    Plastic = b'L',
    Metal = b'M',
    Sand = b'N',
    Foliage = b'O',
    Computer = b'P',
    Asphalt = b'Q',
    Brick = b'R',
    Slosh = b'S',
    Tile = b'T',
    Cardboard = b'U',
    Vent = b'V',
    Wood = b'W',
    Glass = b'Y',
    WarpShield = b'Z',
    Clay = 1,
    Plaster = 2,
    Rock = 3,
    Rubber = 4,
    Sheetrock = 5,
    Cloth = 6,
    Carpet = 7,
    Paper = 8,
    Upholstery = 9,
    Puddle = 10,
    Mud = 11,
    SandBarrelNoPenetration = 12,
    SandBarrel = 13,
    MetalShield = 14,
}

#[derive(Clone, Copy, Debug)]
pub struct SurfaceData {
    pub kind: SurfaceKind,
    pub damage_modifier: f32,
    pub penetration_modifier: f32,
}

#[derive(Resource)]
pub struct IPhysicsSurfaceProps {
    pub(crate) ptr: Ptr,
}

impl IPhysicsSurfaceProps {
    #[inline]
    pub fn data(&self, index: ffi::c_int) -> Option<SurfaceData> {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            index: ffi::c_int,
        ) -> *const internal::surfacedata_t = unsafe { self.ptr.vtable_entry(5) };

        tracing::trace!("obtain surface data");

        let ptr = unsafe { (method)(self.ptr.as_ptr(), index) };
        let internal::surfacedata_t {
            game:
                internal::surfacegameprops_t {
                    penetration_modifier,
                    damage_modifier,
                    material,
                    ..
                },
            ..
        } = unsafe { ptr.as_ref()? };

        tracing::trace!("we never make it");

        Some(SurfaceData {
            kind: unsafe { std::mem::transmute(*material as u8) },
            damage_modifier: *damage_modifier,
            penetration_modifier: *penetration_modifier,
        })
    }
}

/// `public/vphysics_interface.h`.
mod internal {
    use std::ffi;

    /// `public/SoundEmitterSystem/isoundemittersystembase.h`.
    type SoundScriptHash = ffi::c_uint;

    #[repr(C)]
    pub struct surfacephysicsparams_t {
        pub friction: f32,
        pub elasticity: f32,
        pub density: f32,
        pub thickness: f32,
        pub dampening: f32,
    }

    #[repr(C)]
    pub struct surfaceaudioparams_t {
        pub reflectivity: f32,
        pub hardness_factor: f32,
        pub roughness_factor: f32,
        pub rough_threshold: f32,
        pub hard_threshold: f32,
        pub hard_velocity_threshold: f32,
    }

    #[repr(C)]
    pub struct surfacesoundnames_t {
        pub walk_step_left: ffi::c_ushort,
        pub walk_step_right: ffi::c_ushort,
        pub run_step_left: ffi::c_ushort,
        pub run_step_right: ffi::c_ushort,
        pub impact_soft: ffi::c_ushort,
        pub impact_hard: ffi::c_ushort,
        pub scrape_smooth: ffi::c_ushort,
        pub scrape_rough: ffi::c_ushort,
        pub bullet_impact: ffi::c_ushort,
        pub rolling: ffi::c_ushort,
        pub break_sound: ffi::c_ushort,
        pub strain_sound: ffi::c_ushort,
    }

    #[repr(C)]
    pub struct surfacesoundhandles_t {
        pub walk_step_left: SoundScriptHash,
        pub walk_step_right: SoundScriptHash,
        pub run_step_left: SoundScriptHash,
        pub run_step_right: SoundScriptHash,
        pub impact_soft: SoundScriptHash,
        pub impact_hard: SoundScriptHash,
        pub scrape_smooth: SoundScriptHash,
        pub scrape_rough: SoundScriptHash,
        pub bullet_impact: SoundScriptHash,
        pub rolling: SoundScriptHash,
        pub break_sound: SoundScriptHash,
        pub strain_sound: SoundScriptHash,
    }

    #[repr(C)]
    pub struct surfacegameprops_t {
        pub max_speed_factor: f32,
        pub jump_factor: f32,
        pub penetration_modifier: f32,
        pub damage_modifier: f32,
        pub material: ffi::c_ushort,
        pub climbable: ffi::c_uchar,
        pub pad: ffi::c_uchar,
    }

    #[repr(C)]
    pub struct surfacedata_t {
        pub physics: surfacephysicsparams_t,
        pub audio: surfaceaudioparams_t,
        pub sounds: surfacesoundnames_t,
        pub game: surfacegameprops_t,
        pub soundhandles: surfacesoundhandles_t,
    }
}
