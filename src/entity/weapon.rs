use super::Entity;
use core::time::Duration;
use elysium_sdk::WeaponInfo;

/// Weapon methods.
pub trait Weapon<'a>: Entity<'a> {
    fn next_attack_time(&self) -> Duration;
    fn info(&self) -> Option<&WeaponInfo>;
}
