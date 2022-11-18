use crate::{global, networked, pattern, ClientClass, Mat4x3, Ptr, Tick, Time};
use bevy::prelude::{Resource, Vec3};
use std::time::Duration;
use std::{ffi, mem};

pub use id::EntityId;

mod id;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum WeaponSound {
    Empty = 0,
    Single = 1,
    SingleNpc = 2,
    Double = 3,
    DoubleNpc = 4,
    Burst = 5,
    Reload = 6,
    ReloadNpc = 7,
    MeleeMiss = 8,
    MeleeHit = 9,
    MeleeHitWorld = 10,
    Special1 = 11,
    Special2 = 12,
    Special3 = 13,
    Taunt = 14,
    FastReload = 15,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum MoveKind {
    None = 0,
    Isometric = 1,
    Walk = 2,
    Step = 3,
    Fly = 4,
    FlyGravity = 5,
    VPhysics = 6,
    Push = 7,
    NoClip = 8,
    Ladder = 9,
    Observer = 10,
    Custom = 11,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Team {
    None = 0,
    Spectators = 1,
    Terrorist = 2,
    Counter = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ObserverMode {
    None = 0,
    Deathcam = 1,
    Freezecam = 2,
    Fixed = 3,
    InEye = 4,
    Chase = 5,
    Roaming = 6,
}

impl ObserverMode {
    /// if the observer mode breaks thirdperson
    #[inline]
    pub const fn breaks_thirdperson(&self) -> bool {
        matches!(
            self,
            ObserverMode::InEye | ObserverMode::Chase | ObserverMode::Roaming
        )
    }
}

bitflags::bitflags! {
    /// `public/const.h`.
    #[repr(transparent)]
    pub struct PlayerFlag: u32 {
        const ON_GROUND = 1 << 0;
        const DUCKING = 1 << 1;
        const ANIM_DUCKING = 1 << 2;
        const WATER_JUMP = 1 << 3;
        const ON_TRAIN = 1 << 4;
        const IN_RAIN = 1 << 5;
        const FROZEN = 1 << 6;
        const AT_CONTROLS = 1 << 7;
        const IS_CLIENT = 1 << 8;
        const IS_BOT = 1 << 9;
        const IN_WATER = 1 << 10;
        const FLY = 1 << 11;
        const SWIM = 1 << 12;
        const CONVEYOR = 1 << 13;
        const NPC = 1 << 14;
        const GOD_MODE = 1 << 15;
        const NO_TARGET = 1 << 16;
        const AIM_TARGET = 1 << 17;
        const PARTIAL_GROUND = 1 << 18;
        const STATIC_PROP = 1 << 19;
        const GRAPHED = 1 << 20;
        const GRENADE = 1 << 21;
        const STEP_MOVEMENT = 1 << 22;
        const DONT_TOUCH = 1 << 23;
        const BASE_VELOCITY = 1 << 24;
        const WORLD_BRUSH = 1 << 25;
        const OBJECT = 1 << 26;
        const KILL_ME = 1 << 27;
        const ON_FIRE = 1 << 28;
        const DISSOLVING = 1 << 29;
        const TRANS_RAGDOLL = 1 << 30;
        const UNBLOCKABLE_BY_PLAYER = 1 << 31;
    }
}

pub struct IClientEntity {
    pub(crate) ptr: Ptr,
}

impl IClientEntity {
    #[inline]
    unsafe fn client_renderable(&self) -> Ptr {
        let ptr = self.ptr.byte_offset(mem::size_of::<*mut u8>());
        let ptr = Ptr::new("IClientRenderable", ptr);

        ptr.unwrap_unchecked()
    }

    #[inline]
    unsafe fn client_networkable(&self) -> Ptr {
        let ptr = self.ptr.byte_offset(mem::size_of::<*mut u8>() * 2);
        let ptr = Ptr::new("IClientNetworkable", ptr);

        ptr.unwrap_unchecked()
    }

    #[inline]
    pub fn client_class(&self) -> *const ClientClass {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ClientClass =
            unsafe { networkable.vtable_entry(2) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    #[inline]
    pub fn index(&self) -> i32 {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> i32 =
            unsafe { networkable.vtable_entry(10) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    #[inline]
    pub fn is_dormant(&self) -> bool {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { networkable.vtable_entry(9) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    #[inline]
    pub fn setup_bones(&self, bones: &mut [Mat4x3; 256], mask: ffi::c_int, time: f32) {
        let renderable = unsafe { self.client_renderable() };
        let method: unsafe extern "C" fn(
            this: *mut u8,
            bones: *mut Mat4x3,
            bones_len: ffi::c_int,
            mask: ffi::c_int,
            time: f32,
        ) = unsafe { renderable.vtable_entry(13) };

        unsafe {
            (method)(
                renderable.as_ptr(),
                bones.as_mut_ptr(),
                bones.len() as i32,
                mask,
                time,
            )
        }
    }

    /// Set the player's view angle.
    ///
    /// # Safety
    ///
    /// Modifying the view angle of a player via networked variables may have unintended side
    /// effects! Be sure to reset it to the original value during
    #[inline]
    pub unsafe fn set_view_angle(&self, angle: Vec3) {
        networked::addr!(self.ptr.as_ptr(), base_player.is_dead)
            .byte_add(4)
            .cast::<Vec3>()
            .write_unaligned(angle)
    }

    /// The player's view angle.
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            networked::addr!(self.ptr.as_ptr(), base_player.is_dead)
                .byte_add(4)
                .cast::<Vec3>()
                .read_unaligned()
        }
    }

    #[inline]
    pub fn anim_state(&self) -> Option<AnimState> {
        unsafe {
            let offset = global::with_app(|app| app.world.resource::<AnimStateOffset>().0);
            let ptr = self.ptr.as_ptr().byte_add(offset);
            let ptr = ptr as *const *mut u8;
            let ptr = Ptr::new("AnimState", *ptr)?;

            Some(AnimState { ptr })
        }
    }

    #[inline]
    pub fn max_desync_angle(&self) -> f32 {
        unsafe {
            let Some(anim_state) = self.anim_state() else {
                return 0.0;
            };

            let mut yaw_modifier = (anim_state.running_accel_progress() * -3.0 - 0.2)
                * anim_state.feet_shuffle_speed().clamp(0.0, 1.0)
                + 1.0;

            if anim_state.duck_progress() > 0.0 {
                yaw_modifier += anim_state.duck_progress()
                    * anim_state.feet_shuffle_speed_2().clamp(0.0, 1.0)
                    * (0.5 - yaw_modifier);
            }

            anim_state.velocity_subtract_y() * yaw_modifier
        }
    }

    #[inline]
    pub fn is_lby_updating(&self) -> bool {
        unsafe {
            let Some(anim_state) = self.anim_state() else {
                return false;
            };

            global::with_app_mut(|app| {
                let mut lby_update_time = app
                    .world
                    .get_resource::<LbyUpdateTime>()
                    .map(|time| time.0)
                    .unwrap_or(Time(Duration::ZERO));

                let current_time = self.tick_base().to_time();
                let is_lby_updating = if anim_state.vertical_velocity() > 0.1
                    || anim_state.horizontal_velocity().abs() > 100.0
                {
                    *lby_update_time = *current_time + Duration::from_secs_f32(0.22);

                    false
                } else if current_time > lby_update_time {
                    *lby_update_time = *current_time + Duration::from_secs_f32(1.1);

                    true
                } else {
                    false
                };

                app.insert_resource(LbyUpdateTime(lby_update_time));

                is_lby_updating
            })
        }
    }

    #[inline]
    pub fn tick_base(&self) -> Tick {
        networked::read!(self.ptr.as_ptr(), base_player.tick_base)
    }
}

#[derive(Resource)]
pub struct LbyUpdateTime(pub(crate) Time);

#[derive(Resource)]
pub struct AnimStateOffset(pub(crate) usize);

#[repr(C)]
pub struct AnimState {
    ptr: Ptr,
}

impl AnimState {
    pub unsafe fn setup() {
        tracing::trace!("obtain CSPlayer::Spawn");

        let module = link::load_module("client_client.so").unwrap();
        let bytes = module.bytes();
        let opcode = &pattern::CSPLAYER_SPAWN.find(bytes).unwrap().1[..56];

        tracing::trace!("CSPlayer::Spawn = {opcode:02X?}");
        tracing::trace!("obtain AnimState offset");

        let ip = opcode.as_ptr().byte_add(52);
        let offset = ip.cast::<u32>().read() as usize;

        tracing::trace!("AnimState offset = {offset:?}");

        global::with_app_mut(|app| {
            app.insert_resource(AnimStateOffset(offset));
        });
    }

    unsafe fn read<T>(&self, offset: usize) -> T {
        self.ptr.byte_offset::<T>(offset).read_unaligned()
    }

    unsafe fn write<T>(&self, offset: usize, value: T) {
        self.ptr.byte_offset::<T>(offset).write_unaligned(value);
    }

    unsafe fn duck_progress(&self) -> f32 {
        self.read(0xB8)
    }

    unsafe fn horizontal_velocity(&self) -> f32 {
        self.read(0x100)
    }

    unsafe fn vertical_velocity(&self) -> f32 {
        self.read(0x104)
    }

    unsafe fn feet_shuffle_speed(&self) -> f32 {
        self.read(0x10C)
    }

    unsafe fn feet_shuffle_speed_2(&self) -> f32 {
        self.read(0x110)
    }

    unsafe fn running_accel_progress(&self) -> f32 {
        self.read(0x130)
    }

    unsafe fn velocity_subtract_y(&self) -> f32 {
        self.read(0x3A4)
    }
}
