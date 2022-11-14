//! Input interace.

use crate::vtable_validate;
use cake::ffi::{BytePad, VTablePad};
use core::{fmt, ptr};
use elysium_math::Vec3;

pub use joystick::Joystick;
pub use mouse::Mouse;
pub use state::State;

mod joystick;
mod mouse;
mod state;

bitflags::bitflags! {
    /// Movement state flags.
    ///
    /// [`game/shared/in_buttons.h`](https://github.com/alliedmodders/hl2sdk/blob/csgo/game/shared/in_buttons.h).
    #[repr(transparent)]
    pub struct Button: i32 {
        /// Attack with the current weapon.
        ///
        /// You cannot attack if you do not have any weapons.
        const ATTACK = 1 << 0;

        /// Jump.
        ///
        /// You can only jump if you are on the ground.
        const JUMP = 1 << 1;

        /// Duck (Go into a crouching position).
        const DUCK = 1 << 2;

        /// Move forward.
        ///
        /// Used for animations.
        const MOVE_FORWARD = 1 << 3;

        /// Move backward.
        ///
        /// Used for animations.
        const MOVE_BACKWARD = 1 << 4;

        /// Interact with something.
        ///
        /// Plant a bomb, defuse a bomb, open a door, so on.
        const USE = 1 << 5;

        /// TODO.
        const CANCEL = 1 << 6;

        /// TODO.
        const LEFT = 1 << 7;

        /// TODO.
        const RIGHT = 1 << 8;

        /// Move to the left.
        ///
        /// Used for animations.
        const MOVE_LEFT = 1 << 9;

        /// Move to the right.
        ///
        /// Used for animations.
        const MOVE_RIGHT = 1 << 10;

        /// Secondary attack with the current weapon.
        ///
        /// Switch firing mode, quick fire a revolver, etc.
        ///
        /// You cannot attack if you do not have any weapons.
        const ATTACK_SECONDARY = 1 << 11;

        /// TODO.
        const RUN = 1 << 12;

        /// Reload the current weapon.
        ///
        /// You cannot reload if you do not have any weapons.
        const RELOAD = 1 << 13;

        /// TODO.
        const ALT = 1 << 14;

        /// TODO.
        const ALT_SECONDARY = 1 << 15;

        /// TODO.
        const SCOREBOARD = 1 << 16;

        /// TODO.
        const SPEED = 1 << 17;

        /// TODO.
        const WALK = 1 << 18;

        /// TODO.
        const ZOOM = 1 << 19;

        /// TODO.
        const WEAPON = 1 << 20;

        /// TODO.
        const WEAPON_SECONDARY = 1 << 21;

        /// Enables fast duck.
        ///
        /// Must be enabled for the duration of ducking normally.
        ///
        /// See [`CCSGameMovement::CheckParameters` in `game/shared/cstrike15/cs_gamemovement.cpp`](https://github.com/elysian6969/cstrike/blob/master/game/shared/cstrike15/cs_gamemovement.cpp#L169) for why this works.
        const FAST_DUCK = 1 << 22;

        /// TODO.
        const GRENADE = 1 << 23;

        /// TODO.
        const GRENADE_SECONDARY = 1 << 24;

        /// TODO.
        const LOOK_SPIN = 1 << 25;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    pub vtable: *const (),
    pub command: i32,
    pub tick_count: i32,
    pub view_angle: Vec3,
    pub aim_direction: Vec3,
    pub movement: Vec3,
    pub buttons: Button,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Vec3,
    pub head_offset: Vec3,
}

#[repr(C)]
struct VTable {
    _pad0: VTablePad<8>,
    get_user_command:
        unsafe extern "thiscall" fn(this: *const Input, slot: i32, sequence: i32) -> *const Command,
    _pad1: VTablePad<13>,
    activate_mouse: unsafe extern "thiscall" fn(this: *const Input),
    deactivate_mouse: unsafe extern "thiscall" fn(this: *const Input),
}

vtable_validate! {
    get_user_command => 8,
    activate_mouse => 22,
    deactivate_mouse => 23,
}

#[repr(C)]
pub struct Input {
    vtable: &'static VTable,
    _pad0: BytePad<8>,
    pub is_track_ir_available: bool,
    pub is_mouse_initialized: bool,
    pub is_mouse_active: bool,
    _pad1: BytePad<162>,
    pub thirdperson: bool,
    pub camera_moving_with_mouse: bool,
    pub offset: Vec3,
}

impl Input {
    #[inline]
    pub fn get_user_command(&self, slot: i32, sequence: i32) -> *const Command {
        unsafe { (self.vtable.get_user_command)(self, slot, sequence) }
    }

    /// hides the cursor and starts re-centering
    #[inline]
    pub fn activate_mouse(&self) {
        unsafe { (self.vtable.activate_mouse)(self) }
    }

    /// gives back the cursor and stops centering the mouse
    #[inline]
    pub fn deactivate_mouse(&self) {
        unsafe { (self.vtable.deactivate_mouse)(self) }
    }
}

impl fmt::Debug for Input {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Input")
            .field("vtable", &self.vtable)
            .field("_pad0", &self._pad0)
            .field("is_track_ir_available", &self.is_track_ir_available)
            .field("is_mouse_initialized", &self.is_mouse_initialized)
            .field("is_mouse_active", &self.is_mouse_active)
            .field("_pad1", &self._pad1)
            .field("thirdperson", &self.thirdperson)
            .field("camera_moving_with_mouse", &self.camera_moving_with_mouse)
            .field("offset", &self.offset)
            .finish()
    }
}

impl fmt::Debug for VTable {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VTable")
            .field("_pad0", &self._pad0)
            .field("get_user_command", &ptr::addr_of!(self.get_user_command))
            .finish()
    }
}
