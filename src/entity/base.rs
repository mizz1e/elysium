use super::{EntityRef, FogRef, PlayerRef, TonemapRef, WeaponRef};
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::model::Model;

pub(super) use sealed::Sealed;

mod sealed {
    pub trait Sealed {}
}

/// Entity methods.
pub trait Entity<'a>: Sealed + 'a {
    fn attachment(&self, index: i32) -> Option<Vec3>;

    /// Cast this reference to an entity reference.
    unsafe fn cast_entity(self) -> EntityRef<'a>;

    /// Cast this reference to a fog reference.
    unsafe fn cast_fog(self) -> FogRef<'a>;

    /// Cast this reference to a player reference.
    unsafe fn cast_player(self) -> PlayerRef<'a>;

    /// Cast this reference to a tonemap reference.
    unsafe fn cast_tonemap(self) -> TonemapRef<'a>;

    /// Cast this reference to a weapon reference.
    unsafe fn cast_weapon(self) -> WeaponRef<'a>;

    /// The entity's class.
    fn client_class(&self) -> Option<&'a Class>;

    /// The entity's health.
    fn health(&self) -> i32;

    /// Is this entity alive?
    fn is_alive(&self) -> bool;

    /// Is the entity dormant?
    fn is_dormant(&self) -> bool;

    /// Is this entity a player?
    fn is_player(&self) -> bool;

    /// Is this entity a weapon?
    fn is_weapon(&self) -> bool;

    /// The entity's index within the entity list.
    fn index(&self) -> i32;

    /// The entity's model.
    fn model(&self) -> Option<&'a Model>;

    /// The entity's origin.
    fn origin(&self) -> Vec3;

    /// Setup bones for this entity.
    fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool;
}
