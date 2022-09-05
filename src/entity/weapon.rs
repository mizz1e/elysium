use super::Entity;

/// Weapon methods.
pub trait Weapon<'a>: Entity<'a> {
    fn next_attack_time(&self) -> f32;
}
