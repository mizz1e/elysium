use crate::Ptr;
use bevy::prelude::*;
use std::ffi;

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

#[derive(Resource)]
#[repr(C)]
pub struct CUserCmd {
    vtable: *const (),
    pub number: ffi::c_int,
    pub tick_count: ffi::c_int,
    pub view_angle: Vec3,
    pub aim_direction: Vec3,
    pub movement: Vec3,
    pub buttons: Button,
    pub impulse: u8,
    pub weapon_select: ffi::c_int,
    pub weapon_subtype: ffi::c_int,
    pub random_seed: ffi::c_int,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Vec3,
    pub head_offset: Vec3,
}

unsafe impl Send for CUserCmd {}
unsafe impl Sync for CUserCmd {}

#[derive(Resource)]
pub struct CInput {
    pub(crate) ptr: Ptr,
}

impl CInput {
    unsafe fn internal(&self) -> &mut internal::CInput {
        &mut *self.ptr.as_ptr().cast::<internal::CInput>()
    }

    pub fn in_thirdperson(&self) -> bool {
        unsafe { self.internal().in_thirdperson }
    }

    pub fn set_in_thirdperson(&self, enabled: bool) {
        unsafe {
            self.internal().in_thirdperson = enabled;
        }
    }
}

mod internal {
    use std::mem::MaybeUninit;

    #[repr(C)]
    pub struct CInput {
        _pad0: MaybeUninit<[u8; 18]>,
        pub mouse_active: bool,
        _pad1: MaybeUninit<[u8; 162]>,
        pub in_thirdperson: bool,
    }
}
