use super::{PlayerRef, WeaponRef};
use crate::{networked, networked_mut, Networked, State};
use cake::ffi::VTablePad;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, PlayerFlags, Renderable, Team};
use elysium_sdk::model::Model;
use elysium_sdk::{object_validate, vtable_validate, HitGroup, WeaponInfo};
use palette::{Srgb, Srgba, WithAlpha};
use std::mem::MaybeUninit;
use std::ops::RangeInclusive;
use std::ptr;

pub use exposure::Exposure;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<12>,
    origin: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const Vec3,
    _pad1: VTablePad<98>,
    set_model_index: unsafe extern "thiscall" fn(this: *mut EntityRepr, index: i32),
    _pad2: VTablePad<10>,
    attachment:
        unsafe extern "thiscall" fn(this: *const EntityRepr, index: i32, origin: *mut Vec3) -> bool,
    _pad3: VTablePad<5>,
    team: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Team,
    _pad4: VTablePad<38>,
    health: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad5: VTablePad<40>,
    is_alive: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad6: VTablePad<1>,
    is_player: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad7: VTablePad<7>,
    is_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad8: VTablePad<112>,
    active_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad9: VTablePad<16>,
    eye_pos: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad10: VTablePad<1>,
    weapon_sub_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad11: VTablePad<6>,
    observer_mode: unsafe extern "thiscall" fn(this: *const EntityRepr) -> ObserverMode,
    observer_target: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad12: VTablePad<50>,
    aim_punch: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad13: VTablePad<62>,
    draw_crosshair: unsafe extern "thiscall" fn(this: *const EntityRepr),
    _pad14: VTablePad<48>,
    spread: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    _pad15: VTablePad<1>,
    weapon_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad16: VTablePad<5>,
    weapon_data: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const WeaponInfo,
    _pad17: VTablePad<6>,
    muzzle_attachment_index_1st:
        unsafe extern "thiscall" fn(this: *const EntityRepr, view_model: *const EntityRepr) -> i32,
    muzzle_attachment_index_3rd: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad18: VTablePad<13>,
    inaccuracy: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    update_accuracy_penalty: unsafe extern "thiscall" fn(this: *mut EntityRepr),
}

vtable_validate! {
    origin => 12,
    set_model_index => 111,
    attachment => 122,
    team => 128,
    health => 167,
    is_alive => 208,
    is_player => 210,
    is_weapon => 218,
    active_weapon => 331,
    eye_pos => 348,
    weapon_sub_kind => 350,
    observer_mode => 357,
    observer_target => 358,
    aim_punch => 409,
    draw_crosshair => 472,
    spread => 521,
    weapon_kind => 523,
    weapon_data => 529,
    muzzle_attachment_index_1st => 536,
    muzzle_attachment_index_3rd => 537,
    inaccuracy => 551,
    update_accuracy_penalty => 552,
}

#[repr(C)]
pub(super) struct EntityRepr {
    vtable: &'static VTable,
    renderable: Renderable,
    networkable: Networkable,
}

object_validate! {
    EntityRepr;
    vtable => 0,
    renderable => 8,
    networkable => 16,
}

// generic
impl EntityRepr {
    networked!(render_mode_address: u8 = base_entity.render_mode);

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        ptr::addr_of!(*self).cast()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        ptr::addr_of_mut!(*self).cast()
    }

    #[inline]
    fn networked<T, F>(&self, f: F) -> *const T
    where
        F: Fn(&Networked) -> usize,
    {
        let state = State::get();
        let offset = f(&state.networked);

        unsafe { self.as_ptr().cast::<T>().byte_add(offset) }
    }

    #[inline]
    fn networked_mut<T, F>(&mut self, f: F) -> *mut T
    where
        F: Fn(&Networked) -> usize,
    {
        let state = State::get();
        let offset = f(&state.networked);

        unsafe { self.as_mut_ptr().cast::<T>().byte_add(offset) }
    }

    #[inline]
    pub fn attachment(&self, index: i32) -> Option<Vec3> {
        let mut origin = MaybeUninit::uninit();

        unsafe {
            (self.vtable.attachment)(self, index, origin.as_mut_ptr())
                .then(|| MaybeUninit::assume_init(origin))
        }
    }

    /// The entity's class.
    #[inline]
    pub fn client_class(&self) -> Option<&Class> {
        unsafe { self.networkable.client_class().cast::<Class>().as_ref() }
    }

    /// The entity's health.
    #[inline]
    pub fn health(&self) -> i32 {
        unsafe { (self.vtable.health)(self) }
    }

    /// Is this entity alive?
    #[inline]
    pub fn is_alive(&self) -> bool {
        unsafe { (self.vtable.is_alive)(self) }
    }

    /// Is the entity dormant?
    #[inline]
    pub fn is_dormant(&self) -> bool {
        self.networkable.is_dormant()
    }

    /// Is this entity a player?
    #[inline]
    pub fn is_player(&self) -> bool {
        unsafe { (self.vtable.is_player)(self) }
    }

    /// Is this entity a weapon?
    #[inline]
    pub fn is_weapon(&self) -> bool {
        unsafe { (self.vtable.is_weapon)(self) }
    }

    /// The entity's index within the entity list.
    #[inline]
    pub fn index(&self) -> i32 {
        self.networkable.index()
    }

    /// The entity's model.
    #[inline]
    pub fn model(&self) -> Option<&Model> {
        unsafe { self.renderable.model().cast::<Model>().as_ref() }
    }

    /// The entity's origin.
    #[inline]
    pub fn origin(&self) -> Vec3 {
        unsafe { *(self.vtable.origin)(self) }
    }

    #[inline]
    pub fn set_model_index(&mut self, index: i32) {
        unsafe { (self.vtable.set_model_index)(self, index) }
    }

    /// Setup bones for this entity.
    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        self.renderable.setup_bones(bones, mask, time)
    }

    #[inline]
    pub fn should_draw(&self) -> bool {
        self.renderable.should_draw()
    }
}

// fog
impl EntityRepr {
    networked!(rgb: i32 = fog.color_primary);
    networked_mut!(rgb_mut: i32 = fog.color_primary);

    networked!(density: f32 = fog.density);
    networked_mut!(density_mut: f32 = fog.density);

    networked!(far_z: f32 = fog.far_z);
    networked_mut!(far_z_mut: f32 = fog.far_z);

    networked!(is_enabled: bool = fog.is_enabled);
    networked_mut!(is_enabled_mut: bool = fog.is_enabled);

    networked!(start: f32 = fog.start);
    networked_mut!(start_mut: f32 = fog.start);

    networked!(end: f32 = fog.end);
    networked_mut!(end_mut: f32 = fog.end);

    /// Returns the clip distance (far-Z).
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn clip_distance(&self) -> f32 {
        unsafe { self.far_z().read_unaligned() }
    }

    /// Returns the distance range (start and end distance).
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn range(&self) -> Option<RangeInclusive<f32>> {
        unsafe {
            self.is_enabled().read_unaligned().then(|| {
                let start = self.start().read_unaligned();
                let end = self.end().read_unaligned();

                start..=end
            })
        }
    }

    /// Returns the color (rgb) and density (alpha).
    #[inline]
    pub fn rgba(&self) -> Srgba {
        let rgb = unsafe { self.rgb().read_unaligned() as u32 };
        let alpha = unsafe { self.density().read_unaligned() };

        // why is this bgr??
        let [b, g, r, _] = rgb.to_ne_bytes();
        let rgb = u32::from_ne_bytes([r, g, b, 0]);

        let srgb: Srgb<f32> = Srgb::from(rgb).into_format();

        srgb.with_alpha(alpha)
    }

    /// Set the clip distance (far-Z).
    ///
    /// A non-finite, negative or zero value will disable the clip distance.
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn set_clip_distance(&mut self, distance: f32) {
        unsafe { self.far_z_mut().write_unaligned(distance) }
    }

    /// Set the distance range (start and end distance).
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn set_range(&mut self, range: Option<RangeInclusive<f32>>) {
        unsafe {
            let enabled = range
                .inspect(|range| {
                    let start = elysium_math::to_finite(*range.start(), 0.0).max(0.0);
                    let end = elysium_math::to_finite(*range.end(), 0.0).max(start);

                    self.start_mut().write_unaligned(start);
                    self.end_mut().write_unaligned(end);
                })
                .is_some();

            self.is_enabled_mut().write_unaligned(enabled);
        }
    }

    /// Set the color (rgb) and density (alpha).
    ///
    /// Non-finite or negative alpha will be treated as 0.0.
    #[inline]
    pub fn set_rgba(&mut self, srgba: impl Into<Srgba>) {
        let (rgb, alpha) = srgba.into().split();
        let rgb: Srgb<u8> = rgb.into_format();
        let rgb: u32 = rgb.into();

        // why is this bgr??
        let [b, g, r, _] = rgb.to_ne_bytes();
        let rgb = u32::from_ne_bytes([r, g, b, 0]);

        unsafe {
            self.rgb_mut().write_unaligned(rgb as i32);
            self.density_mut().write_unaligned(alpha);
        }
    }
}

// player
impl EntityRepr {
    networked!(armor_value_ref: i32 = player.armor);
    networked!(flags_ref: i32 = player.flags);
    networked!(has_helmet_ref: bool = player.has_helmet);
    networked!(is_dead_address: u8 = base_player.is_dead);
    networked!(is_defusing_ref: bool = player.is_defusing);
    networked!(is_scoped_ref: bool = player.is_scoped);
    networked!(lower_body_yaw_ref: f32 = player.lower_body_yaw);
    networked!(tick_base_ref: u32 = base_player.tick_base);
    networked!(view_offset_ref: Vec3 = base_player.view_offset);
    networked!(velocity_ref: Vec3 = base_player.velocity);

    /// The player's active weapon.
    #[inline]
    pub fn active_weapon(&self) -> Option<WeaponRef<'_>> {
        unsafe {
            let weapon = (self.vtable.active_weapon)(self);

            WeaponRef::from_raw(weapon as _)
        }
    }

    /// The player's aim punch.
    #[inline]
    pub fn aim_punch(&self) -> Vec3 {
        unsafe { (self.vtable.aim_punch)(self) }
    }

    /// The player's armor value.
    #[inline]
    pub fn armor_value(&self) -> i32 {
        unsafe { self.armor_value_ref().read_unaligned() }
    }

    /// Returns the damage modifier for the provided hit group and ratio.
    #[inline]
    pub fn damage_modifier(&self, group: HitGroup, weapon_armor_ratio: f32) -> f32 {
        let mut modifier = group.damage_modifier();

        if self.armor_value() > 0 {
            if group.is_head() && self.has_helmet() {
                modifier *= weapon_armor_ratio * 0.5;
            }
        }

        modifier
    }

    /// The player's eye offset (from the player's origin).
    #[inline]
    pub fn eye_offset(&self) -> Vec3 {
        let view_offset = unsafe { self.view_offset_ref().read_unaligned() };

        // zero view offset fix
        if view_offset.is_zero() {
            let z = if self.flags().ducking() { 46.0 } else { 64.0 };

            Vec3::from_xyz(0.0, 0.0, z)
        } else {
            view_offset
        }
    }

    /// The player's eye origin.
    #[inline]
    pub fn eye_origin(&self) -> Vec3 {
        self.origin() + self.eye_offset()
    }

    // TODO: check if this is better than above
    /// The player's eye origin.
    #[inline]
    pub fn eye_origin_alt(&self) -> Vec3 {
        unsafe { (self.vtable.eye_pos)(self) }
    }

    /// The player's state flags.
    #[inline]
    pub fn flags(&self) -> PlayerFlags {
        unsafe {
            let flags = self.flags_ref().read_unaligned();

            PlayerFlags::new(flags)
        }
    }

    /// Whether the player has a helmet.
    #[inline]
    pub fn has_helmet(&self) -> bool {
        unsafe { self.has_helmet_ref().read_unaligned() }
    }

    /// Whether the player is defusing a bomb.
    #[inline]
    pub fn is_defusing(&self) -> bool {
        unsafe { self.is_defusing_ref().read_unaligned() }
    }

    /// Whether the player is scoped.
    #[inline]
    pub fn is_scoped(&self) -> bool {
        unsafe { self.is_scoped_ref().read_unaligned() }
    }

    /// The player's lower body yaw.
    #[inline]
    pub fn lower_body_yaw(&self) -> f32 {
        unsafe { self.lower_body_yaw_ref().read_unaligned() }
    }

    /// The player's movement type.
    #[inline]
    pub fn move_kind(&self) -> MoveKind {
        unsafe {
            let kind = self
                .render_mode_address()
                .byte_add(1)
                .cast::<i32>()
                .read_unaligned();

            MoveKind::from_raw(kind)
        }
    }

    /// The player's observing mode.
    #[inline]
    pub fn observer_mode(&self) -> ObserverMode {
        unsafe { (self.vtable.observer_mode)(self) }
    }

    /// The player's observer target player.
    #[inline]
    pub fn observer_target(&self) -> Option<PlayerRef> {
        unsafe {
            let observer = (self.vtable.observer_target)(self);

            PlayerRef::from_raw(observer as _)
        }
    }

    /// Set the player's view angle.
    ///
    /// # Safety
    ///
    /// Modifying the view angle of a player via networked variables may have unintended side
    /// effects! Be sure to reset it to the original value during
    /// [`Frame::RenderEnd`](elysium_sdk::Frame::RenderEnd).
    #[inline]
    pub unsafe fn set_view_angle(&mut self, angle: Vec3) {
        self.is_dead_address()
            .byte_add(4)
            .cast::<Vec3>()
            .cast_mut()
            .write_unaligned(angle)
    }

    /// The player's team.
    #[inline]
    pub fn team(&self) -> Team {
        unsafe { (self.vtable.team)(self) }
    }

    /// The player's tick base.
    #[inline]
    pub fn tick_base(&self) -> u32 {
        unsafe { self.tick_base_ref().read_unaligned() }
    }

    /// The player's velocity.
    #[inline]
    pub fn velocity(&self) -> Vec3 {
        unsafe { self.velocity_ref().read_unaligned() }
    }

    /// The player's view angle.
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            self.is_dead_address()
                .byte_add(4)
                .cast::<Vec3>()
                .read_unaligned()
        }
    }
}

mod exposure {
    use std::mem;
    use std::ops::{Bound, RangeBounds};

    /// Tonemap exposure.
    #[derive(Clone, Copy)]
    pub struct Exposure {
        start: Bound<f32>,
        end: Bound<f32>,
    }

    impl Exposure {
        /// Returns sanitized bounds.
        #[inline]
        pub(super) fn to_tuple(self) -> (f32, f32) {
            const MIN_EXPOSURE: f32 = 0.0001;

            fn map(bound: Bound<f32>) -> f32 {
                let value = match bound {
                    Bound::Included(value) => value,
                    Bound::Excluded(value) => value,
                    Bound::Unbounded => MIN_EXPOSURE,
                };

                let value = elysium_math::to_finite(value, MIN_EXPOSURE);
                let value = value.max(MIN_EXPOSURE);

                value
            }

            let start = map(self.start);
            let end = map(self.end).max(start);

            (start, end)
        }
    }

    impl Default for Exposure {
        #[inline]
        fn default() -> Self {
            Self::from(0.5..=0.5)
        }
    }

    impl<R> From<R> for Exposure
    where
        R: RangeBounds<f32>,
    {
        #[inline]
        fn from(range: R) -> Self {
            let start = range.start_bound().map(mem::copy);
            let end = range.end_bound().map(mem::copy);

            Self { start, end }
        }
    }
}

// tonemap
impl EntityRepr {
    networked!(bloom_scale: f32 = tonemap.bloom_scale);
    networked_mut!(bloom_scale_mut: f32 = tonemap.bloom_scale);

    networked!(enable_bloom_scale: bool = tonemap.enable_bloom_scale);
    networked_mut!(enable_bloom_scale_mut: bool = tonemap.enable_bloom_scale);

    networked!(enable_max_exposure: bool = tonemap.enable_max_exposure);
    networked_mut!(enable_max_exposure_mut: bool = tonemap.enable_max_exposure);

    networked!(enable_min_exposure: bool = tonemap.enable_min_exposure);
    networked_mut!(enable_min_exposure_mut: bool = tonemap.enable_min_exposure);

    networked!(max_exposure: f32 = tonemap.max_exposure);
    networked_mut!(max_exposure_mut: f32 = tonemap.max_exposure);

    networked!(min_exposure: f32 = tonemap.min_exposure);
    networked_mut!(min_exposure_mut: f32 = tonemap.min_exposure);

    /// Returns the bloom effect scale.
    #[inline]
    pub fn bloom(&self) -> f32 {
        unsafe {
            self.enable_bloom_scale()
                .read_unaligned()
                .then(|| self.bloom_scale().read_unaligned())
                .unwrap_or_default()
        }
    }

    /// Returns the exposure range.
    #[inline]
    pub fn exposure(&self) -> Option<Exposure> {
        unsafe {
            let min_enabled = self.enable_min_exposure().read_unaligned();
            let max_enabled = self.enable_max_exposure().read_unaligned();
            let min = self.min_exposure().read_unaligned();
            let max = self.max_exposure().read_unaligned();

            match (min_enabled, max_enabled) {
                (true, true) => Some(Exposure::from(min..=max)),
                (true, false) => Some(Exposure::from(min..)),
                (false, true) => Some(Exposure::from(..=min)),
                (false, false) => None,
            }
        }
    }

    /// Set the bloom effect scale.
    ///
    /// Non-finite or negative scale will be treated as 0.0.
    #[inline]
    pub fn set_bloom(&mut self, scale: f32) {
        unsafe {
            let scale = elysium_math::to_finite(scale, 0.0).max(0.0);

            self.bloom_scale_mut().write_unaligned(scale);
            self.enable_bloom_scale_mut().write_unaligned(scale != 0.0);
        }
    }

    /// Set the exposure range.
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    #[inline]
    pub fn set_exposure<E: Into<Exposure>>(&mut self, exposure: Option<E>) {
        let (start, end) = exposure.map(Into::into).unwrap_or_default().to_tuple();

        unsafe {
            self.min_exposure_mut().write_unaligned(start);
            self.max_exposure_mut().write_unaligned(end);

            self.enable_min_exposure_mut().write(true);
            self.enable_max_exposure_mut().write(true);
        }
    }
}

// weapon
impl EntityRepr {
    networked!(magazine_ref: i32 = base_weapon.magazine);
    networked!(next_attack_time_ref: f32 = base_weapon.next_attack_time);
    networked!(revolver_cock_time_ref: f32 = weapon.revolver_cock_time);

    #[inline]
    pub fn magazine(&self) -> Option<u32> {
        let magazine: i32 = unsafe { self.magazine_ref().read_unaligned() };

        if magazine < 0 {
            None
        } else {
            Some(magazine as u32)
        }
    }

    #[inline]
    pub fn next_attack_time(&self) -> f32 {
        unsafe { self.next_attack_time_ref().read_unaligned() }
    }

    #[inline]
    pub fn revolver_cock_time(&self) -> Option<f32> {
        let time: f32 = unsafe { self.revolver_cock_time_ref().read_unaligned() };

        if time > 3.4028235e+38 {
            None
        } else {
            Some(time)
        }
    }

    #[inline]
    pub fn draw_crosshair(&self) {
        unsafe { (self.vtable.draw_crosshair)(self) }
    }

    #[inline]
    pub fn spread(&self) -> f32 {
        unsafe { (self.vtable.spread)(self) }
    }

    #[inline]
    pub fn weapon_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_kind)(self) }
    }

    #[inline]
    pub fn weapon_data(&self) -> Option<&WeaponInfo> {
        unsafe { (self.vtable.weapon_data)(self).as_ref() }
    }

    #[inline]
    pub fn muzzle_attachment_index_1st(&self, view_model: *const EntityRepr) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_1st)(self, view_model) }
    }

    #[inline]
    pub fn muzzle_attachment_index_3rd(&self) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_3rd)(self) }
    }

    #[inline]
    pub fn inaccuracy(&self) -> f32 {
        unsafe { (self.vtable.inaccuracy)(self) }
    }

    #[inline]
    pub fn update_accuracy_penalty(&mut self) {
        unsafe { (self.vtable.update_accuracy_penalty)(self) }
    }

    #[inline]
    pub fn weapon_sub_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_sub_kind)(self) }
    }
}
