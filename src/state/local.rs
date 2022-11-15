use bevy::math::Vec3;
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
    /// Reference to the local player.
    pub player: *const u8,

    /// Local player thridperson state.
    pub thirdperson: Thirdperson,

    /// Local player's view angle.
    pub view_angle: Vec3,
}

const NEW: Local = Local {
    player: ptr::null(),
    thirdperson: Thirdperson::new(),
    view_angle: Vec3::splat(0.0),
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
