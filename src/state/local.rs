use elysium_math::Vec3;
use elysium_model::Bones;
use std::ptr;

pub struct Thirdperson {
    pub enabled: bool,
    pub toggle: bool,
    pub toggle_lock: bool,
}

impl Thirdperson {
    #[inline]
    pub const fn new() -> Self {
        Self {
            enabled: true,
            toggle: false,
            toggle_lock: false,
        }
    }
}

/// Local player-related values.
pub struct Local {
    /// Local player's aim punch angle.
    pub aim_punch_angle: Vec3,

    /// Local player's bones.
    pub bones: Bones,

    pub fake_bones: Bones,

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

    /// Local player thridperson state.
    pub thirdperson: Thirdperson,

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
    pub was_jumping: bool,

    pub time: f32,
    pub shift: i32,
}

const NEW: Local = Local {
    aim_punch_angle: Vec3::zero(),
    bones: Bones::zero(),
    fake_bones: Bones::zero(),
    health: 0,
    magazine_ammo: 0,
    old_yaw: 0.0,
    player: ptr::null(),
    shot_view_angle: Vec3::zero(),
    thirdperson: Thirdperson::new(),
    total_ammo: 0,
    view_angle: Vec3::zero(),
    view_punch_angle: Vec3::zero(),
    visualize_shot: 0.0,
    weapon: ptr::null(),
    was_attacking: false,
    was_jumping: false,
    time: 0.0,
    shift: 0,
};

impl Local {
    /// Initailize local structurr.
    #[inline]
    pub(crate) const fn new() -> Self {
        NEW
    }

    /// Reset local player values.
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Toggle thirdperson.
    #[inline]
    pub fn toggle_thirdperson(&mut self) {
        if !self.thirdperson.toggle_lock {
            self.thirdperson.toggle ^= true;
            self.thirdperson.toggle_lock = true;
        }
    }

    /// Release thirdperson lock.
    #[inline]
    pub fn release_thirdperson_toggle(&mut self) {
        self.thirdperson.toggle_lock = false;
    }
}
