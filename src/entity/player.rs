use super::{Entity, PlayerRef, WeaponRef};
use elysium_math::Vec3;
use elysium_sdk::entity::{MoveKind, ObserverMode, PlayerFlags, Team};
use elysium_sdk::HitGroup;

/// Player methods.
pub trait Player<'a>: Entity<'a> {
    /// The player's active weapon.
    fn active_weapon(&self) -> Option<WeaponRef<'a>>;

    /// The player's aim punch.
    fn aim_punch(&self) -> Vec3;

    /// The player's armor value.
    fn armor_value(&self) -> i32;

    /// Returns the damage modifier for the provided hit group and ratio.
    fn damage_modifier(&self, group: HitGroup, ratio: f32) -> f32;

    /// The player's eye offset (from the player's origin).
    fn eye_offset(&self) -> Vec3;

    /// The player's eye origin.
    fn eye_origin(&self) -> Vec3;

    /// The player's state flags.
    fn flags(&self) -> PlayerFlags;

    /// Whether the player has a helmet.
    fn has_helmet(&self) -> bool;

    /// Whether the player is defusing a bomb.
    fn is_defusing(&self) -> bool;

    /// Whether the player is scoped.
    fn is_scoped(&self) -> bool;

    /// The player's lower body yaw.
    fn lower_body_yaw(&self) -> i32;

    /// The player's movement type.
    fn move_kind(&self) -> MoveKind;

    /// The player's observing mode.
    fn observer_mode(&self) -> ObserverMode;

    /// The player's observer target player.
    fn observer_target(&self) -> Option<PlayerRef<'a>>;

    /// Set the player's view angle.
    ///
    /// # Safety
    ///
    /// Modifying the view angle of a player via networked variables may have unintended side
    /// effects! Be sure to reset it to the original value during
    /// [`Frame::RenderEnd`](elysium_sdk::Frame::RenderEnd).
    unsafe fn set_view_angle(&mut self, angle: Vec3);

    /// The player's team.
    fn team(&self) -> Team;

    /// The player's velocity.
    fn velocity(&self) -> Vec3;

    /// The magnitude of the player's velocity.
    fn velocity_magnitude(&self) -> f32;

    /// The player's view angle.
    fn view_angle(&self) -> Vec3;
}
