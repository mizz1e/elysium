//! Anti-aim implementation.

use core::ops::RangeInclusive;
use rand::Rng;

pub use pitch::Pitch;
pub use roll::{Roll, RollModifier, RollModifierKind};
pub use yaw::{Yaw, YawModifier, YawModifierKind};

mod pitch;
mod roll;
mod yaw;

#[inline]
pub(crate) fn random(range: RangeInclusive<f32>) -> f32 {
    let mut random = rand::thread_rng();

    random.gen_range(range)
}
