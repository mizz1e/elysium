//! Networked variables
// TODO: move to elysium_sdk and hide implementation(?)

use core::mem::MaybeUninit;
use elysium_sdk::client::{Client, Table};
use elysium_sdk::{Class, Entry};

#[derive(Debug)]
pub struct BaseAnimating {
    pub client_side_animation: usize,
}

#[derive(Debug)]
pub struct BaseEntity {
    pub render_mode: usize,
    pub team: usize,
}

#[derive(Debug)]
pub struct BasePlayer {
    pub aim_punch_angle: usize,
    pub health: usize,
    pub is_dead: usize,
    pub tick_base: usize,
    pub velocity: usize,
    pub view_offset: usize,
    pub view_punch_angle: usize,
}

#[derive(Debug)]
pub struct BaseWeapon {
    pub next_attack_time: usize,
    pub magazine: usize,
}

#[derive(Debug)]
pub struct Fog {
    pub color_primary: usize,
    pub color_secondary: usize,
    pub density: usize,
    pub direction: usize,
    pub end: usize,
    pub far_z: usize,
    pub is_enabled: usize,
    pub hdr_scale: usize,
    pub start: usize,
}

#[derive(Debug)]
pub struct Item {
    pub index: usize,
}

#[derive(Debug)]
pub struct Player {
    pub armor: usize,
    pub eye_angle: usize,
    pub flags: usize,
    pub has_defuse_kit: usize,
    pub has_helmet: usize,
    pub in_bomb_zone: usize,
    pub is_defusing: usize,
    pub is_immune: usize,
    pub is_scoped: usize,
    pub lower_body_yaw: usize,
    pub max_flash_alpha: usize,
    pub money: usize,
    pub observer: usize,
    pub patch_indicies: usize,
    pub ragdoll: usize,
    pub shots_fired: usize,
    pub wait_for_no_attack: usize,
    pub weapon: usize,
}

#[derive(Debug)]
pub struct Weapon {
    pub revolver_cock_time: usize,
}

#[derive(Debug)]
pub struct Tonemap {
    pub bloom_scale: usize,
    pub enable_bloom_scale: usize,
    pub enable_min_exposure: usize,
    pub enable_max_exposure: usize,
    pub min_exposure: usize,
    pub max_exposure: usize,
}

/// Networked variable manager.
#[derive(Debug)]
pub struct Networked {
    pub base_animating: BaseAnimating,
    pub base_entity: BaseEntity,
    pub base_player: BasePlayer,
    pub base_weapon: BaseWeapon,
    pub fog: Fog,
    pub item: Item,
    pub player: Player,
    pub weapon: Weapon,
    pub tonemap: Tonemap,
}

const NEW: Networked = unsafe { MaybeUninit::zeroed().assume_init() };

impl Networked {
    #[inline]
    pub const fn new() -> Self {
        NEW
    }

    #[inline]
    pub fn update(&mut self, client: &Client) {
        let top_level = client.class_iter();

        // Iterate classes.
        for class in top_level {
            if let Some(table) = class.table {
                // Skip classes we are not interested in.
                if let Some(class) = Class::from_str(&*table.name()) {
                    iterate_table(self, table, class, 0);
                }
            }
        }

        println!("{self:?}");
    }
}

/// Insert an entry we have interest in into our map.
#[inline]
fn insert_entry(this: &mut Networked, class: Class, entry: Entry, offset: usize) {
    match (class, entry) {
        // base_animating
        (Class::BaseAnimating, Entry::ClientSideAnimation) => {
            this.base_animating.client_side_animation = offset
        }

        // base_entity
        (Class::BaseEntity, Entry::RenderMode) => this.base_entity.render_mode = offset,
        (Class::BaseEntity, Entry::Team) => this.base_entity.team = offset,

        // base_player
        (Class::BasePlayer, Entry::AimPunchAngle) => this.base_player.aim_punch_angle = offset,
        (Class::BasePlayer, Entry::Health) => this.base_player.health = offset,
        (Class::BasePlayer, Entry::IsDead) => this.base_player.is_dead = offset,
        (Class::BasePlayer, Entry::TickBase) => this.base_player.tick_base = offset,
        (Class::BasePlayer, Entry::Velocity) => this.base_player.velocity = offset,
        (Class::BasePlayer, Entry::ViewOffset) => this.base_player.view_offset = offset,
        (Class::BasePlayer, Entry::ViewPunchAngle) => this.base_player.view_punch_angle = offset,

        // base_weapon
        (Class::BaseWeapon, Entry::NextAttackTime) => this.base_weapon.next_attack_time = offset,
        (Class::BaseWeapon, Entry::Magazine) => this.base_weapon.magazine = offset,

        // fog
        (Class::Fog, Entry::FogColorPrimary) => this.fog.color_primary = offset,
        (Class::Fog, Entry::FogColorSecondary) => this.fog.color_secondary = offset,
        (Class::Fog, Entry::FogDensity) => this.fog.density = offset,
        (Class::Fog, Entry::FogDirection) => this.fog.direction = offset,
        (Class::Fog, Entry::FogEnd) => this.fog.end = offset,
        (Class::Fog, Entry::FogFarZ) => this.fog.far_z = offset,
        (Class::Fog, Entry::FogIsEnabled) => this.fog.is_enabled = offset,
        (Class::Fog, Entry::FogHDRScale) => this.fog.hdr_scale = offset,
        (Class::Fog, Entry::FogStart) => this.fog.start = offset,

        // item
        (Class::Item, Entry::ItemIndex) => this.item.index = offset,

        // player
        (Class::Player, Entry::Armor) => this.player.armor = offset,
        (Class::Player, Entry::EyeAngle) => this.player.eye_angle = offset,
        (Class::Player, Entry::Flags) => this.player.flags = offset,
        (Class::Player, Entry::HasDefuseKit) => this.player.has_defuse_kit = offset,
        (Class::Player, Entry::HasHelmet) => this.player.has_helmet = offset,
        (Class::Player, Entry::InBombZone) => this.player.in_bomb_zone = offset,
        (Class::Player, Entry::IsDefusing) => this.player.is_defusing = offset,
        (Class::Player, Entry::IsImmune) => this.player.is_immune = offset,
        (Class::Player, Entry::IsScoped) => this.player.is_scoped = offset,
        (Class::Player, Entry::LowerBodyYaw) => this.player.lower_body_yaw = offset,
        (Class::Player, Entry::MaxFlashAlpha) => this.player.max_flash_alpha = offset,
        (Class::Player, Entry::Money) => this.player.money = offset,
        (Class::Player, Entry::Observer) => this.player.observer = offset,
        (Class::Player, Entry::PatchIndicies) => this.player.patch_indicies = offset,
        (Class::Player, Entry::Ragdoll) => this.player.ragdoll = offset,
        (Class::Player, Entry::ShotsFired) => this.player.shots_fired = offset,
        (Class::Player, Entry::WaitForNoAttack) => this.player.wait_for_no_attack = offset,
        (Class::Player, Entry::Weapon) => this.player.weapon = offset,

        // weapon
        (Class::Weapon, Entry::RevolverCockTime) => this.weapon.revolver_cock_time = offset,

        // tonemap
        (Class::Tonemap, Entry::BloomScale) => this.tonemap.bloom_scale = offset,
        (Class::Tonemap, Entry::EnableBloomScale) => this.tonemap.enable_bloom_scale = offset,
        (Class::Tonemap, Entry::EnableMaxExposure) => this.tonemap.enable_max_exposure = offset,
        (Class::Tonemap, Entry::EnableMinExposure) => this.tonemap.enable_min_exposure = offset,
        (Class::Tonemap, Entry::MaxExposure) => this.tonemap.max_exposure = offset,
        (Class::Tonemap, Entry::MinExposure) => this.tonemap.min_exposure = offset,
        _ => {}
    }
}

/// Iterate the networked tables.
#[inline]
fn iterate_table(this: &mut Networked, table: &'static Table, class: Class, base_offset: usize) {
    // TODO: impl iterator for ISlice
    for property in table.properties().iter() {
        if let Some(sub_table) = property.data_table() {
            // Recurse sub-tables.
            iterate_table(
                this,
                sub_table,
                class,
                base_offset + property.offset as usize,
            );
        }

        // Skip entries we are not interested in.
        if let Some(entry) = Entry::from_str(&*property.name()) {
            let offset = base_offset + property.offset as usize;

            insert_entry(this, class, entry, offset);
        }
    }
}

/// Generates a function to obtain a pointer to a networked variable.
#[macro_export]
macro_rules! networked {
    ($ident:ident: $ty:ty = $class:ident.$entry:ident) => {
        #[inline]
        fn $ident(&self) -> *const $ty {
            self.networked(|networked| networked.$class.$entry)
        }
    };
}

/// Generates a function to obtain a mutable pointer to a networked variable.
#[macro_export]
macro_rules! networked_mut {
    ($ident:ident: $ty:ty = $class:ident.$entry:ident) => {
        #[inline]
        fn $ident(&mut self) -> *mut $ty {
            self.networked_mut(|networked| networked.$class.$entry)
        }
    };
}
