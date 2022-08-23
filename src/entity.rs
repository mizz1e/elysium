use crate::{Networked, State};
use cake::ffi::VTablePad;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::entity::{MoveKind, ObserverMode, PlayerFlags, Team};
use elysium_sdk::model::Model;
use elysium_sdk::{object_validate, vtable_validate, HitGroup, WeaponInfo};
use repr::EntityRepr;
use std::marker::PhantomData;
use std::ops::{RangeBounds, RangeInclusive};
use std::ptr::NonNull;

pub use repr::Exposure;

mod repr;

mod sealed {
    pub trait Sealed {}
}

/// Entity methods.
pub trait Entity: sealed::Sealed {
    fn attachment(&self, index: i32) -> Option<Vec3>;

    /// the entity's class
    fn client_class(&self) -> Option<&Class>;

    /// the entity's health
    fn health(&self) -> i32;

    /// is this entity alive
    fn is_alive(&self) -> bool;

    /// is the entity dormant
    fn is_dormant(&self) -> bool;

    /// is this entity a player
    fn is_player(&self) -> bool;

    /// is this entity a weapon
    fn is_weapon(&self) -> bool;

    /// the entity's index
    fn index(&self) -> i32;

    /// the entity's model
    fn model(&self) -> Option<&Model>;

    /// the entity's origin
    fn origin(&self) -> Vec3;
}

/// Fog methods.
pub trait Fog: Entity {
    /// Returns the fog's clip distance (far-Z).
    fn clip_distance(&self) -> f32;

    /// Returns the fog's range (start and end distance).
    fn range(&self) -> Option<RangeInclusive<f32>>;

    /// Returns the fog's color (rgb) and density (alpha).
    fn rgba(&self) -> (u8, u8, u8, f32);

    /// Set the fog's clip distance (far-Z).
    fn set_clip_distance(&mut self, distance: f32);

    /// Set the fog's range (start and end distance).
    fn set_range(&mut self, distance: Option<RangeInclusive<f32>>);

    /// Set the fog's color (rgb) and density (alpha).
    fn set_rgba(&mut self, rgba: (u8, u8, u8, f32));
}

/// Player methods.
pub trait Player: Entity {
    /// The player's active weapon.
    fn active_weapon(&self) -> Option<WeaponRef>;

    /// The player's aim punch.
    fn aim_punch(&self) -> Vec3;

    /// The player's armor value.
    fn armor_value(&self) -> i32;

    /// Returns the damage modifier for the provided hit group and ratio.
    fn damage_modifier(&self, group: HitGroup, ratio: f32) -> f32;

    /// The player's eye origin.
    fn eye_origin(&self) -> Vec3;

    /// The player's eye position.
    fn eye_pos(&self) -> Vec3;

    /// The player's state flags.
    fn flags(&self) -> PlayerFlags;

    /// Whether the player has a helmet.
    fn has_helmet(&self) -> bool;

    /// Whether the player is defusing a bomb.
    fn is_defusing(&self) -> bool;

    /// Whether the player is scoped.
    fn is_scoped(&self) -> bool;

    /// The player's lower body vaw.
    fn lower_body_yaw(&self) -> i32;

    /// The player's movement type.
    fn move_kind(&self) -> MoveKind;

    /// The player's observing mode.
    fn observer_mode(&self) -> ObserverMode;

    /// The player's observer target.
    fn observer_target(&self) -> Option<EntityRef>;

    /// Set the player's view angle.
    fn set_view_angle(&mut self, angle: Vec3);

    /// The player's team.
    fn team(&self) -> Team;

    /// The player's velocity.
    fn velocity(&self) -> Vec3;

    /// The player's view angle.
    fn view_angle(&self) -> Vec3;

    /// The player's view offset.
    fn view_offset(&self) -> Vec3;
}

/// Tonemap methods.
pub trait Tonemap: Entity {
    /// Returns the tonemap's bloom effect setting.
    fn bloom(&self) -> Option<f32>;

    /// Returns the tonemap's exposure effect setting.
    fn exposure(&self) -> Option<Exposure>;

    /// Returns the tonemap's bloom effect setting.
    fn set_bloom(&mut self, scale: Option<f32>);

    /// Sets the tonemap's exposure effect setting.
    fn set_exposure<R: RangeBounds<u16>>(&mut self, exposure: Option<R>);
}

/// Weapon methods.
pub trait Weapon: Entity {}

macro_rules! def_ent {
    (
        $(
            $(#[$meta:meta])*
            pub struct $ident:ident<'a>;
        )*
    ) => {
        $(
            $(#[$meta])*
            pub struct $ident<'a> {
                entity: NonNull<EntityRepr>,
                _phantom: PhantomData<&'a mut EntityRepr>,
            }

            impl<'a> $ident<'a> {
                #[inline]
                pub unsafe fn from_raw(entity: *const u8) -> Option<Self> {
                    let entity = NonNull::new(entity)?;
                    let _phantom = PhantomData;

                    Some(Self { entity, _phantom })
                }

                #[inline]
                fn as_repr(&self) -> &'a EntityRepr {
                    self.entity.as_ref()
                }

                #[inline]
                fn as_repr_mut(&mut self) -> &'a mut EntityRepr {
                    self.entity.as_mut()
                }
            }

            impl<'a> Entity for $ident<'a> {
                #[inline]
                fn attachment(&self, index: i32) -> Option<Vec3> {
                    self.as_repr().attachment(index)
                }

                #[inline]
                fn client_class(&self) -> Option<&Class> {
                    self.as_repr().client_class()
                }

                #[inline]
                fn health(&self) -> i32 {
                    self.as_repr().health()
                }

                #[inline]
                fn is_alive(&self) -> bool {
                    self.as_repr().is_alive()
                }

                #[inline]
                fn is_dormant(&self) -> bool {
                    self.as_repr().is_dormant()
                }

                #[inline]
                fn is_player(&self) -> bool {
                    self.as_repr().is_player()
                }

                #[inline]
                fn is_weapon(&self) -> bool {
                    self.as_repr().is_weapon()
                }

                #[inline]
                fn index(&self) -> i32 {
                    self.as_repr().index()
                }

                #[inline]
                fn model(&self) -> Option<&Model> {
                    self.as_repr().model()
                }

                #[inline]
                fn origin(&self) -> Vec3 {
                    self.as_repr().origin()
                }
            }
        )*
    };
}

def_ent! {
    /// A reference to an entity.
    pub struct EntityRef<'a>;

    /// A reference to a fog entity.
    pub struct FogRef<'a>;

    /// A reference to a player.
    pub struct PlayerRef<'a>;

    /// A reference to a tonemap entity.
    pub struct TonemapRef<'a>;

    /// A reference to a weapon.
    pub struct WeaponRef<'a>;
}

impl<'a> Fog for FogRef<'a> {
    #[inline]
    fn clip_distance(&self) -> f32 {
        self.as_repr().clip_distance()
    }

    #[inline]
    fn range(&self) -> Option<RangeInclusive<f32>> {
        self.as_repr().range()
    }

    #[inline]
    fn rgba(&self) -> (u8, u8, u8, f32) {
        self.as_repr().rgba()
    }

    #[inline]
    fn set_clip_distance(&mut self, distance: f32) {
        self.as_repr_mut().set_clip_distance(distance);
    }

    #[inline]
    fn set_range(&mut self, range: Option<RangeInclusive<f32>>) {
        self.as_repr_mut().set_range(range);
    }

    #[inline]
    fn set_rgba(&mut self, rgba: (u8, u8, u8, f32)) {
        self.as_repr_mut().set_rgba(rgba);
    }
}

impl<'a> Tonemap for TonemapRef<'a> {
    #[inline]
    fn bloom(&self) -> Option<f32> {
        self.as_repr().bloom()
    }

    #[inline]
    fn exposure(&self) -> Option<Exposure> {
        self.as_repr().exposure()
    }

    #[inline]
    fn set_bloom(&mut self, bloom: Option<f32>) {
        self.as_repr_mut().set_bloom(bloom);
    }

    #[inline]
    fn set_exposure<R: RangeBounds<u16>>(&mut self, exposure: Option<R>) {
        self.as_repr_mut().set_exposure(exposure);
    }
}
