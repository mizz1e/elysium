//! Local player-related values.

use super::{Shared, SharedOption, STATE};
use core::ptr::NonNull;
use elysium_math::Vec3;
use providence_model::Bones;

macro_rules! local {
    ($(($get:ident, $set:ident): $shared:ident<$ty:ty> = $expr:expr;)*) => {
        pub(crate) struct Local {
            $($get: $shared<$ty>,)*
        }

        impl Local {
            #[inline]
            pub const fn new() -> Self {
                Self {
                    $($get: $expr,)*
                }
            }
        }

        $(
            #[inline]
            pub unsafe fn $get() -> $ty {
                *STATE.local.$get.as_mut()
            }

            #[inline]
            pub fn $set(value: $ty) {
                unsafe {
                    STATE.local.$get.write(value);
                }
            }
        )*
    };
}

local! {
    (aim_punch_angle, set_aim_punch_angle): Shared<Vec3> = Shared::new(Vec3::zero());
    (player, set_player): SharedOption<NonNull<u8>> = SharedOption::none();
    (bones, set_bones): Shared<Bones> = Shared::new(Bones::zero());
    (weapon, set_weapon): SharedOption<NonNull<u8>> = SharedOption::none();
    (magazine_ammo, set_magazine_ammo): Shared<i32> = Shared::new(0);
    (total_ammo, set_total_ammo): Shared<i32> = Shared::new(0);
    (health, set_health): Shared<i32> = Shared::new(0);
    (old_yaw, set_old_yaw): Shared<f32> = Shared::new(0.0);
    (shot_view_angle, set_shot_view_angle): Shared<Vec3> = Shared::new(Vec3::zero());
    (use_shot_view_angle, set_use_shot_view_angle): Shared<f32> = Shared::new(0.0);
    (view_angle, set_view_angle): Shared<Vec3> = Shared::new(Vec3::zero());
    (view_punch_angle, set_view_punch_angle): Shared<Vec3> = Shared::new(Vec3::zero());
}

/// Is the local player uninitialized.
#[inline]
pub fn is_player_none() -> bool {
    STATE.local.player.is_none()
}

/// Reset local player
#[inline]
pub fn set_player_none() {
    unsafe {
        STATE.local.player.take();
    }
}
