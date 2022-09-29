use super::Entity;
use elysium_sdk::WeaponInfo;

/// Weapon methods.
pub trait Weapon<'a>: Entity<'a> {
    fn next_attack_time(&self) -> f32;
    fn info(&self) -> Option<&WeaponInfo>;
}
