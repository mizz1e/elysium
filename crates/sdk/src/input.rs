//! Input interace.

use crate::vtable_validate;
use cake::ffi::{BytePad, VTablePad};
use core::{fmt, ptr};
use elysium_math::Vec3;

pub use button::Button;
pub use joystick::Joystick;
pub use mouse::Mouse;
pub use state::State;

mod button;
mod joystick;
mod mouse;
mod state;

pub const IN_ATTACK: i32 = 1 << 0;
pub const IN_JUMP: i32 = 1 << 1;
pub const IN_DUCK: i32 = 1 << 2;
pub const IN_BULLRUSH: i32 = 1 << 22;
pub const IN_LEFT: i32 = 1 << 9;
pub const IN_RIGHT: i32 = 1 << 10;

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    pub vtable: *const (),
    pub command: i32,
    pub tick_count: i32,
    pub view_angle: Vec3,
    pub aim_direction: Vec3,
    pub movement: Vec3,
    pub state: i32,
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

impl Command {
    #[inline]
    const fn has(&self, flag: i32) -> bool {
        (self.state & flag) != 0
    }

    #[inline]
    const fn set(&mut self, flag: i32, value: bool) {
        if value {
            self.state |= flag;
        } else {
            self.state &= !flag;
        }
    }

    #[inline]
    pub const fn in_attack(&self) -> bool {
        self.has(IN_ATTACK)
    }

    #[inline]
    pub const fn in_jump(&self) -> bool {
        self.has(IN_JUMP)
    }

    #[inline]
    pub const fn in_duck(&self) -> bool {
        self.has(IN_DUCK)
    }

    #[inline]
    pub const fn in_fast_duck(&self) -> bool {
        self.has(IN_BULLRUSH)
    }

    #[inline]
    pub const fn attack(&mut self, value: bool) {
        self.set(IN_ATTACK, value)
    }

    #[inline]
    pub const fn jump(&mut self, value: bool) {
        self.set(IN_JUMP, value)
    }

    #[inline]
    pub const fn duck(&mut self, value: bool) {
        self.set(IN_DUCK, value)
    }

    #[inline]
    pub const fn fast_duck(&mut self, value: bool) {
        self.set(IN_BULLRUSH, value);
    }
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
