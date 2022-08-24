use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::entity::{MoveKind, ObserverMode, PlayerFlags, Team};
use elysium_sdk::model::Model;
use elysium_sdk::HitGroup;
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
pub trait Entity<'a>: sealed::Sealed + 'a {
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

    /// is the entity dormant
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

/// Fog methods.
pub trait Fog<'a>: Entity<'a> {
    /// Returns the fog's clip distance (far-Z).
    fn clip_distance(&self) -> f32;

    /// Returns the fog's range (start and end distance).
    fn range(&self) -> Option<RangeInclusive<f32>>;

    /// Returns the fog's color (rgb) and density (alpha).
    fn rgba(&self) -> (u8, u8, u8, f32);

    /// Set the fog's clip distance (far-Z).
    ///
    /// Non-finite, negative or zero will disable the clip distance.
    fn set_clip_distance(&mut self, distance: f32);

    /// Set the fog's range (start and end distance).
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    fn set_range(&mut self, distance: Option<RangeInclusive<f32>>);

    /// Set the fog's color (rgb) and density (alpha).
    ///
    /// Non-finite or negative alpha will be treated as 0.0.
    fn set_rgba(&mut self, rgba: (u8, u8, u8, f32));
}

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

    /// The player's lower body vaw.
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

/// Tonemap methods.
pub trait Tonemap<'a>: Entity<'a> {
    /// Returns the tonemap's bloom effect setting.
    fn bloom(&self) -> f32;

    /// Returns the tonemap's exposure effect setting.
    fn exposure(&self) -> Option<Exposure>;

    /// Returns the tonemap's bloom effect setting.
    ///
    /// Non-finite or negative scale will be treated as 0.0.
    fn set_bloom(&mut self, scale: f32);

    /// Sets the tonemap's exposure effect setting.
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    fn set_exposure<B: Into<Exposure>>(&mut self, exposure: Option<E>);
}

/// Weapon methods.
pub trait Weapon<'a>: Entity<'a> {}

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
                    let entity = NonNull::new(entity as _)?;
                    let _phantom = PhantomData;

                    Some(Self { entity, _phantom })
                }

                #[inline]
                pub unsafe fn from_raw_unchecked(entity: *const u8) -> Self {
                    let entity = NonNull::new_unchecked(entity as _);
                    let _phantom = PhantomData;

                    Self { entity, _phantom }
                }

                #[inline]
                fn as_repr(&self) -> &'a EntityRepr {
                    unsafe { self.entity.as_ref() }
                }

                #[inline]
                fn as_repr_mut(&mut self) -> &'a mut EntityRepr {
                    unsafe { self.entity.as_mut() }
                }
            }

            impl<'a> sealed::Sealed for $ident<'a> {}

            // all entities implement entity
            impl<'a> Entity<'a> for $ident<'a> {
                #[inline]
                fn attachment(&self, index: i32) -> Option<Vec3> {
                    self.as_repr().attachment(index)
                }

                #[inline]
                unsafe fn cast_entity(self) -> EntityRef<'a> {
                    EntityRef::from_raw_unchecked(self.entity.as_ptr() as _)
                }

                #[inline]
                unsafe fn cast_fog(self) -> FogRef<'a> {
                    FogRef::from_raw_unchecked(self.entity.as_ptr() as _)
                }

                #[inline]
                unsafe fn cast_player(self) -> PlayerRef<'a> {
                    PlayerRef::from_raw_unchecked(self.entity.as_ptr() as _)
                }

                #[inline]
                unsafe fn cast_tonemap(self) -> TonemapRef<'a> {
                    TonemapRef::from_raw_unchecked(self.entity.as_ptr() as _)
                }

                #[inline]
                unsafe fn cast_weapon(self) -> WeaponRef<'a> {
                    WeaponRef::from_raw_unchecked(self.entity.as_ptr() as _)
                }

                #[inline]
                fn client_class(&self) -> Option<&'a Class> {
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
                fn model(&self) -> Option<&'a Model> {
                    self.as_repr().model()
                }

                #[inline]
                fn origin(&self) -> Vec3 {
                    self.as_repr().origin()
                }

                #[inline]
                fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
                    self.as_repr().setup_bones(bones, mask, time)
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

impl<'a> Fog<'a> for FogRef<'a> {
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

impl<'a> Player<'a> for PlayerRef<'a> {
    #[inline]
    fn active_weapon(&self) -> Option<WeaponRef<'a>> {
        self.as_repr().active_weapon()
    }

    #[inline]
    fn aim_punch(&self) -> Vec3 {
        self.as_repr().aim_punch()
    }

    #[inline]
    fn armor_value(&self) -> i32 {
        self.as_repr().armor_value()
    }

    #[inline]
    fn damage_modifier(&self, group: HitGroup, ratio: f32) -> f32 {
        self.as_repr().damage_modifier(group, ratio)
    }

    #[inline]
    fn eye_offset(&self) -> Vec3 {
        self.as_repr().eye_offset()
    }

    #[inline]
    fn eye_origin(&self) -> Vec3 {
        self.as_repr().eye_origin()
    }

    #[inline]
    fn flags(&self) -> PlayerFlags {
        self.as_repr().flags()
    }

    #[inline]
    fn has_helmet(&self) -> bool {
        self.as_repr().has_helmet()
    }

    #[inline]
    fn is_defusing(&self) -> bool {
        self.as_repr().is_defusing()
    }

    #[inline]
    fn is_scoped(&self) -> bool {
        self.as_repr().is_scoped()
    }

    #[inline]
    fn lower_body_yaw(&self) -> i32 {
        self.as_repr().lower_body_yaw()
    }

    #[inline]
    fn move_kind(&self) -> MoveKind {
        self.as_repr().move_kind()
    }

    #[inline]
    fn observer_mode(&self) -> ObserverMode {
        self.as_repr().observer_mode()
    }

    #[inline]
    fn observer_target(&self) -> Option<PlayerRef<'a>> {
        self.as_repr().observer_target()
    }

    #[inline]
    unsafe fn set_view_angle(&mut self, angle: Vec3) {
        self.as_repr_mut().set_view_angle(angle)
    }

    #[inline]
    fn team(&self) -> Team {
        self.as_repr().team()
    }

    #[inline]
    fn velocity(&self) -> Vec3 {
        self.as_repr().velocity()
    }

    #[inline]
    fn velocity_magnitude(&self) -> f32 {
        self.velocity().magnitude()
    }

    #[inline]
    fn view_angle(&self) -> Vec3 {
        self.as_repr().view_angle()
    }
}

impl<'a> Tonemap<'a> for TonemapRef<'a> {
    #[inline]
    fn bloom(&self) -> f32 {
        self.as_repr().bloom()
    }

    #[inline]
    fn exposure(&self) -> Option<Exposure> {
        self.as_repr().exposure()
    }

    #[inline]
    fn set_bloom(&mut self, bloom: f32) {
        self.as_repr_mut().set_bloom(bloom);
    }

    #[inline]
    fn set_exposure<E: Into<Exposure>>(&mut self, exposure: Option<E>) {
        self.as_repr_mut().set_exposure(exposure);
    }
}
