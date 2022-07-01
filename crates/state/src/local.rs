//! Local player-related values.

use core::cell::SyncUnsafeCell;
use core::ptr;
use elysium_math::Vec3;
use providence_model::Bones;

#[repr(transparent)]
struct LocalWrapper(Local);

unsafe impl Sync for LocalWrapper {}

static LOCAL: SyncUnsafeCell<LocalWrapper> = SyncUnsafeCell::new(LocalWrapper(Local::new()));

/// Local player-related values.
pub struct Local {
    /// Local player's aim punch angle.
    pub aim_punch_angle: Vec3,
    /// Local player's bones.
    pub bones: Bones,
    /// Local player's current health.
    pub health: i32,
    /// Local player's current magazine ammo.
    pub magazine_ammo: i32,
    /// Local player's old yaw.
    pub old_yaw: f32,
    /// Reference to the local player.
    pub player: *const u8,
    /// Local player's shot angle (used for ragebot).
    pub shot_view_angle: Vec3,
    /// Whether the local player is in thirdperson or not.
    pub thirdperson: bool,
    /// Prevent SDL_PollEvent key duplication (thus "breaking" toggling).
    pub thirdperson_lock: bool,
    /// Total ammo the local player has.
    pub total_ammo: i32,
    /// Local player's view angle.
    pub view_angle: Vec3,
    /// Local player's view punch angle.
    pub view_punch_angle: Vec3,
    /// Whether to visualise the shot angle (used for ragebot).
    pub visualize_shot: f32,
    /// Local player's current weapon.
    pub weapon: *const u8,
    /// If the local player was attacking last tick.
    pub was_attacking: bool,
    /// If the local player was on the ground last tick.
    pub was_on_ground: bool,
}

impl Local {
    const INIT: Self = Self {
        aim_punch_angle: Vec3::zero(),
        bones: Bones::zero(),
        health: 0,
        magazine_ammo: 0,
        old_yaw: 0.0,
        player: ptr::null(),
        shot_view_angle: Vec3::zero(),
        thirdperson: false,
        thirdperson_lock: false,
        total_ammo: 0,
        view_angle: Vec3::zero(),
        view_punch_angle: Vec3::zero(),
        visualize_shot: 0.0,
        weapon: ptr::null(),
        was_attacking: false,
        was_on_ground: false,
    };

    /// Initailize local structurr.
    #[inline]
    pub(crate) const fn new() -> Self {
        Self::INIT
    }

    /// Obtain a mutable reference to shared local player-related variables.
    #[inline]
    pub fn get() -> &'static mut Self {
        // SAFETY: LocalWrapper is repr(transparent)
        unsafe { &mut *SyncUnsafeCell::raw_get(&LOCAL).cast() }
    }

    /// Reset local player values.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::INIT;
    }

    /// Toggle thirdperson.
    #[inline]
    pub fn toggle_thirdperson(&mut self) {
        if !self.thirdperson_lock {
            self.thirdperson ^= true;
            self.thirdperson_lock = true;
        }
    }

    /// Release thirdperson lock.
    #[inline]
    pub fn release_thirdperson_lock(&mut self) {
        self.thirdperson_lock = false;
    }
}
