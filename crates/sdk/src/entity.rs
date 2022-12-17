use crate::{
    convar, global, networked, pattern, ClientClass, IClientEntityList, IVEngineClient, Mat4x3,
    Ptr, Tick, Time,
};
use bevy::prelude::{Resource, Vec3};
use std::ffi::OsStr;
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
pub enum WeaponKind {
    Knife = 0,
    Pistol = 1,
    SMG = 2,
    Rifle = 3,
    Shotgun = 4,
    SniperRifle = 5,
    Machinegun = 6,
    C4 = 7,
    Placeholder = 8,
    Grenade = 9,
    Unknown = 10,
    StackableItem = 11,
    Fists = 12,
    BreachCharge = 13,
    BumpMine = 14,
    Tablet = 15,
    Melee = 16,
}

#[derive(Clone, Copy, Debug)]
pub struct WeaponInfo {
    pub max_clip: u32,
    pub kind: WeaponKind,
    pub price: u32,
    pub cycle_time: f32,
    pub full_auto: bool,
    pub damage: f32,
    pub headshot_multiplier: f32,
    pub armor_ratio: f32,
    pub bullets: u32,
    pub penetration_modifier: f32,
    pub range: f32,
    pub range_modifier: f32,
    pub has_silencer: bool,
    pub max_speed: f32,
    pub max_speed_alt: f32,
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
    OnLadder = 9,
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

bitflags::bitflags! {
    pub struct EntityFlag: u32 {
        const ALIVE = 1 << 0;
        const DORMANT = 1 << 1;
        const ENEMY = 1 << 2;
        const IN_AIR = 1 << 3;
        const NO_CLIP = 1 << 4;
        const ON_LADDER = 1 << 5;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IClientEntity {
    pub(crate) ptr: Ptr,
}

impl IClientEntity {
    /// Obtain an IClientEntity from an index.
    pub fn from_index(index: i32) -> Option<Self> {
        global::with_resource::<IClientEntityList, _>(|entity_list| entity_list.get(index))
    }

    /// Obtain an IClientEntity for the local player.
    pub fn local_player() -> Option<Self> {
        let index =
            global::with_resource::<IVEngineClient, _>(|engine| engine.local_player_index());

        IClientEntity::from_index(index)
    }

    unsafe fn client_renderable(&self) -> Ptr {
        let ptr = self.ptr.byte_offset(mem::size_of::<*mut u8>());
        let ptr = Ptr::new("IClientRenderable", ptr);

        ptr.unwrap_unchecked()
    }

    unsafe fn client_networkable(&self) -> Ptr {
        let ptr = self.ptr.byte_offset(mem::size_of::<*mut u8>() * 2);
        let ptr = Ptr::new("IClientNetworkable", ptr);

        ptr.unwrap_unchecked()
    }

    pub fn client_class(&self) -> *const ClientClass {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ClientClass =
            unsafe { networkable.vtable_entry(2) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    pub fn index(&self) -> i32 {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> i32 =
            unsafe { networkable.vtable_entry(10) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    pub fn setup_bones(&self, bones: &mut [Mat4x3; 256], mask: ffi::c_int, time: Time) {
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
                time.0.as_secs_f32(),
            )
        }
    }

    fn is_local_player(&self) -> bool {
        IClientEntity::local_player()
            .map(|local_player| local_player.index() == self.index())
            .unwrap_or_default()
    }

    /// Set the player's view angle.
    ///
    /// # Safety
    ///
    /// Modifying the view angle of a player via networked variables may have unintended side
    /// effects! Be sure to reset it to the original value during
    pub unsafe fn set_view_angle(&self, angle: Vec3) {
        networked::addr!(self.ptr.as_ptr(), base_player.is_dead)
            .byte_add(4)
            .cast::<Vec3>()
            .write_unaligned(angle)
    }

    /// The player's view angle.
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            networked::addr!(self.ptr.as_ptr(), base_player.is_dead)
                .byte_add(4)
                .cast::<Vec3>()
                .read_unaligned()
        }
    }

    pub fn anim_state(&self) -> Option<AnimState> {
        unsafe {
            let offset = global::with_resource::<AnimStateOffset, _>(|offset| offset.0);
            let ptr = self.ptr.as_ptr().byte_add(offset);
            let ptr = ptr as *const *mut u8;
            let ptr = Ptr::new("AnimState", *ptr)?;

            Some(AnimState { ptr })
        }
    }

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

    pub fn is_lby_updating(&self) -> bool {
        unsafe {
            let Some(anim_state) = self.anim_state() else {
                return false;
            };

            global::with_resource_or_init::<LbyUpdateTime, _>(
                |mut lby_update_time| {
                    let lby_update_time = &mut lby_update_time.0;

                    let current_time = self.tick_base().to_time();
                    let is_lby_updating = if anim_state.vertical_velocity() > 0.1
                        || anim_state.horizontal_velocity().abs() > 100.0
                    {
                        **lby_update_time = *current_time + Duration::from_secs_f32(0.22);

                        false
                    } else if current_time > *lby_update_time {
                        **lby_update_time = *current_time + Duration::from_secs_f32(1.1);

                        true
                    } else {
                        false
                    };

                    is_lby_updating
                },
                || LbyUpdateTime(Time(Duration::ZERO)),
            )
        }
    }

    /// The player's tick base.
    pub fn tick_base(&self) -> Tick {
        networked::read!(self.ptr.as_ptr(), base_player.tick_base)
    }

    /// The player's origin.
    pub fn origin(&self) -> Vec3 {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const Vec3 =
            unsafe { self.ptr.vtable_entry(12) };

        unsafe { *(method)(self.ptr.as_ptr()) }
    }

    /// The player's eye origin.
    pub fn eye_origin(&self) -> Vec3 {
        let mut origin = self.origin();

        origin.z += if self.player_flags().contains(PlayerFlag::DUCKING) {
            46.0
        } else {
            64.0
        };

        origin
    }

    /// The player's observing mode.
    pub fn observer_mode(&self) -> ObserverMode {
        let method: unsafe extern "C" fn(this: *mut u8) -> ObserverMode =
            unsafe { self.ptr.vtable_entry(357) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    /// Player's aim punch.
    pub fn aim_punch(&self) -> Vec3 {
        networked::read!(self.ptr.as_ptr(), cs_player.aim_punch)
            * global::with_resource::<convar::RecoilScale, _>(|scale| scale.read())
    }

    /// Player's active weapon.
    pub fn active_weapon(&self) -> Option<Self> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(331) };

        let ptr = unsafe { (method)(self.ptr.as_ptr()) };
        let ptr = Ptr::new("IClientEntity", ptr)?;

        Some(IClientEntity { ptr })
    }

    /// Next primary attack time.
    pub fn next_primary_attack(&self) -> Time {
        networked::read!(self.ptr.as_ptr(), base_combat_weapon.next_primary_attack)
    }

    pub fn weapon_info(&self) -> WeaponInfo {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const internal::WeaponInfo =
            unsafe { self.ptr.vtable_entry(529) };

        let internal::WeaponInfo {
            max_clip,
            kind,
            price,
            cycle_time,
            full_auto,
            damage,
            headshot_multiplier,
            armor_ratio,
            bullets,
            penetration_modifier,
            range,
            range_modifier,
            has_silencer,
            max_speed,
            max_speed_alt,
            ..
        } = unsafe { *(method)(self.ptr.as_ptr()) };

        WeaponInfo {
            max_clip: max_clip as u32,
            kind,
            price: price as u32,
            cycle_time,
            full_auto,
            damage: damage as f32,
            headshot_multiplier,
            armor_ratio,
            bullets: bullets as u32,
            penetration_modifier,
            range,
            range_modifier,
            has_silencer,
            max_speed,
            max_speed_alt,
        }
    }

    /// Whether the player is scoped.
    pub fn armor_value(&self) -> i32 {
        networked::read!(self.ptr.as_ptr(), cs_player.armor_value)
    }

    /// Whether the player is scoped.
    pub fn is_scoped(&self) -> bool {
        networked::read!(self.ptr.as_ptr(), cs_player.is_scoped)
    }

    /// The entity's health.
    pub fn health(&self) -> i32 {
        networked::read!(self.ptr.as_ptr(), base_player.health) as i32
    }

    pub fn velocity(&self) -> Vec3 {
        networked::read!(self.ptr.as_ptr(), base_player.velocity)
    }

    pub fn location_name(&self) -> Option<Box<OsStr>> {
        networked::read!(self.ptr.as_ptr(), base_player.location_name)
    }

    /// Determine whether this entity is a player.
    pub fn is_player(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(210) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    /// Determine whether this entity is alive.
    fn is_alive(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(208) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    /// Determine whether this entity is dormant.
    fn is_dormant(&self) -> bool {
        let networkable = unsafe { self.client_networkable() };
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { networkable.vtable_entry(9) };

        unsafe { (method)(networkable.as_ptr()) }
    }

    /// Determine whether this entity is an enemy to the local player.
    fn is_enemy(&self) -> bool {
        (global::with_resource::<convar::Ffa, _>(|ffa| ffa.read()) && !self.is_local_player())
            || IClientEntity::local_player()
                .map(|local_player| self.team() != local_player.team())
                .unwrap_or_default()
    }

    /// Determine whether the player is immune to damage.
    fn is_immune(&self) -> bool {
        networked::read!(self.ptr.as_ptr(), cs_player.is_immune)
    }

    /// The entity's team.
    fn team(&self) -> i32 {
        networked::read!(self.ptr.as_ptr(), base_entity.team) as i32
    }

    fn player_flags(&self) -> PlayerFlag {
        networked::read!(self.ptr.as_ptr(), base_player.flags)
    }

    fn move_kind(&self) -> MoveKind {
        unsafe {
            networked::addr!(self.ptr.as_ptr(), base_entity.render_mode)
                .byte_add(1)
                .cast::<MoveKind>()
                .read_unaligned()
        }
    }

    /// Current state of the entity.
    pub fn flags(&self) -> EntityFlag {
        if self.is_dormant() {
            // No longer networked, don't read anything more.
            return EntityFlag::DORMANT;
        }

        if !self.is_alive() {
            // No longer alive, don't read anything more.
            return EntityFlag::empty();
        }

        let mut flags = EntityFlag::ALIVE;

        if self.is_enemy() {
            flags.insert(EntityFlag::ENEMY);
        }

        let move_kind = self.move_kind();

        match move_kind {
            MoveKind::OnLadder => flags.insert(EntityFlag::ON_LADDER),
            MoveKind::NoClip => flags.insert(EntityFlag::NO_CLIP),
            _ => {}
        }

        let player_flags = self.player_flags();

        if !player_flags.contains(PlayerFlag::ON_GROUND) {
            flags.insert(EntityFlag::IN_AIR);
        }

        flags
    }

    pub fn remaining_ammo(&self) -> u32 {
        networked::read!(self.ptr.as_ptr(), base_combat_weapon.remaining_ammo) as u32
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

mod internal {
    use super::WeaponKind;
    use std::mem::MaybeUninit;

    impl WeaponKind {
        pub const fn as_i32(&self) -> i32 {
            *self as i32
        }
    }

    /// information about a weapon
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct WeaponInfo {
        _pad0: MaybeUninit<[u8; 32]>,
        pub max_clip: i32,
        _pad1: MaybeUninit<[u8; 204]>,
        pub name: *const u8,
        _pad2: MaybeUninit<[u8; 72]>,
        pub kind: WeaponKind,
        _pad3: MaybeUninit<[u8; 4]>,
        pub price: i32,
        _pad4: MaybeUninit<[u8; 12]>,
        pub cycle_time: f32,
        _pad5: MaybeUninit<[u8; 12]>,
        pub full_auto: bool,
        _pad6: MaybeUninit<[u8; 3]>,
        pub damage: i32,
        pub headshot_multiplier: f32,
        pub armor_ratio: f32,
        pub bullets: i32,
        pub penetration_modifier: f32,
        _pad7: MaybeUninit<[u8; 8]>,
        pub range: f32,
        pub range_modifier: f32,
        _pad8: MaybeUninit<[u8; 16]>,
        pub has_silencer: bool,
        _pad9: MaybeUninit<[u8; 23]>,
        pub max_speed: f32,
        pub max_speed_alt: f32,
        _pad10: MaybeUninit<[u8; 100]>,
        pub recoil_magnitude: f32,
        pub recoil_magnitude_alt: f32,
        _pad11: MaybeUninit<[u8; 16]>,
        pub recovery_time_stand: f32,
    }
}
