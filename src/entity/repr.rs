use super::{PlayerRef, WeaponRef};
use core::time::Duration;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, PlayerFlags, Renderable, Team};
use elysium_sdk::model::Model;
use elysium_sdk::{networked, HitGroup, WeaponInfo};
use palette::{Srgb, Srgba, WithAlpha};
use std::ffi::OsStr;
use std::mem::MaybeUninit;
use std::ops::RangeInclusive;
use std::ptr;

pub use exposure::Exposure;

#[repr(C)]
struct VTable {
    _pad0: MaybeUninit<[unsafe extern "C" fn(); 12]>,
    origin: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const Vec3,
    _pad1: MaybeUninit<[unsafe extern "C" fn(); 98]>,
    set_model_index: unsafe extern "thiscall" fn(this: *mut EntityRepr, index: i32),
    _pad2: MaybeUninit<[unsafe extern "C" fn(); 10]>,
    attachment:
        unsafe extern "thiscall" fn(this: *const EntityRepr, index: i32, origin: *mut Vec3) -> bool,
    _pad3: MaybeUninit<[unsafe extern "C" fn(); 5]>,
    team: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Team,
    _pad4: MaybeUninit<[unsafe extern "C" fn(); 38]>,
    health: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad5: MaybeUninit<[unsafe extern "C" fn(); 40]>,
    is_alive: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad6: MaybeUninit<[unsafe extern "C" fn(); 1]>,
    is_player: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad7: MaybeUninit<[unsafe extern "C" fn(); 7]>,
    is_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad8: MaybeUninit<[unsafe extern "C" fn(); 112]>,
    active_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad9: MaybeUninit<[unsafe extern "C" fn(); 16]>,
    eye_pos: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad10: MaybeUninit<[unsafe extern "C" fn(); 1]>,
    weapon_sub_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad11: MaybeUninit<[unsafe extern "C" fn(); 6]>,
    observer_mode: unsafe extern "thiscall" fn(this: *const EntityRepr) -> ObserverMode,
    observer_target: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad12: MaybeUninit<[unsafe extern "C" fn(); 50]>,
    aim_punch: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad13: MaybeUninit<[unsafe extern "C" fn(); 62]>,
    draw_crosshair: unsafe extern "thiscall" fn(this: *const EntityRepr),
    _pad14: MaybeUninit<[unsafe extern "C" fn(); 48]>,
    spread: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    _pad15: MaybeUninit<[unsafe extern "C" fn(); 1]>,
    weapon_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad16: MaybeUninit<[unsafe extern "C" fn(); 5]>,
    weapon_data: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const WeaponInfo,
    _pad17: MaybeUninit<[unsafe extern "C" fn(); 6]>,
    muzzle_attachment_index_1st:
        unsafe extern "thiscall" fn(this: *const EntityRepr, view_model: *const EntityRepr) -> i32,
    muzzle_attachment_index_3rd: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad18: MaybeUninit<[unsafe extern "C" fn(); 13]>,
    inaccuracy: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    update_accuracy_penalty: unsafe extern "thiscall" fn(this: *mut EntityRepr),
}

#[repr(C)]
pub(super) struct EntityRepr {
    vtable: &'static VTable,
    renderable: Renderable,
    networkable: Networkable,
}

// generic
impl EntityRepr {
    #[inline]
    fn as_ptr(&self) -> *const u8 {
        ptr::addr_of!(*self).cast()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        ptr::addr_of_mut!(*self).cast()
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
    /// Returns the clip distance (far-Z).
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn clip_distance(&self) -> f32 {
        networked::read!(self, fog.clip_distance)
    }

    /// Returns the distance range (start and end distance).
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn range(&self) -> Option<RangeInclusive<f32>> {
        networked::read!(self, fog.is_enabled).then(|| {
            let start = networked::read!(self, fog.start);
            let end = networked::read!(self, fog.end);

            start..=end
        })
    }

    /// Returns the color (rgb) and density (alpha).
    #[inline]
    pub fn rgba(&self) -> Srgba {
        let rgb = 0x00FF0000_u32; // networked::read!(self, fog.rgb);
        let alpha = networked::read!(self, fog.alpha);

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
        networked::write!(self, fog.clip_distance, distance)
    }

    /// Set the distance range (start and end distance).
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    ///
    /// Distance is relative to the local players position.
    #[inline]
    pub fn set_range(&mut self, range: Option<RangeInclusive<f32>>) {
        let enabled = range
            .inspect(|range| {
                let start = (*range.start()).max(0.0);
                let end = (*range.end()).max(start);

                networked::write!(self, fog.start, start);
                networked::write!(self, fog.end, end);
            })
            .is_some();

        networked::write!(self, fog.is_enabled, enabled);
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
        let _rgb = u32::from_ne_bytes([r, g, b, 0]);

        //networked::write!(self, fog.rgb, rgb);
        networked::write!(self, fog.alpha, alpha);
    }
}

// player
impl EntityRepr {
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
        networked::read!(self, player.armor_value)
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
        let offset = networked::read!(self, base_player.eye_offset);

        // zero view offset fix
        if offset == Vec3::splat(0.0) {
            let z = if self.flags().ducking() { 46.0 } else { 64.0 };

            Vec3::from_array([0.0, 0.0, z])
        } else {
            offset
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
        networked::read!(self, base_player.flags)
    }

    /// Whether the player has a helmet.
    #[inline]
    pub fn has_helmet(&self) -> bool {
        networked::read!(self, player.has_helmet)
    }

    /// Whether the player is defusing a bomb.
    #[inline]
    pub fn is_defusing(&self) -> bool {
        todo!()
        //networked::read!(self, player.is_defusing)
    }

    /// Whether the player is immune to damage.
    #[inline]
    pub fn is_immune(&self) -> bool {
        networked::read!(self, player.is_immune)
    }

    /// Whether the player is scoped.
    #[inline]
    pub fn is_scoped(&self) -> bool {
        networked::read!(self, player.is_scoped)
    }

    #[inline]
    pub fn location_name(&self) -> Box<OsStr> {
        networked::read!(self, base_player.location_name)
    }

    /// The player's lower body yaw.
    #[inline]
    pub fn lower_body_yaw(&self) -> f32 {
        networked::read!(self, player.lower_body_yaw)
    }

    /// The player's movement type.
    #[inline]
    pub fn move_kind(&self) -> MoveKind {
        unsafe {
            let kind = networked::addr!(self, base_entity.render_mode)
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
        networked::addr!(self, base_player.is_dead)
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
        networked::read!(self, base_player.tick_base)
    }

    /// The player's velocity.
    #[inline]
    pub fn velocity(&self) -> Vec3 {
        networked::read!(self, base_player.velocity)
    }

    /// The player's view angle.
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            networked::addr!(self, base_player.is_dead)
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
    /// Returns the bloom effect scale.
    #[inline]
    pub fn bloom(&self) -> f32 {
        networked::read!(self, tonemap.bloom_scale)
    }

    /// Returns the exposure range.
    #[inline]
    pub fn exposure(&self) -> Option<Exposure> {
        let start = networked::read!(self, tonemap.exposure_start);
        let end = networked::read!(self, tonemap.exposure_end);

        let start_enabled = networked::read!(self, tonemap.exposure_start_enabled);
        let end_enabled = networked::read!(self, tonemap.exposure_end_enabled);

        match (start_enabled, end_enabled) {
            (true, true) => Some(Exposure::from(start..=end)),
            (true, false) => Some(Exposure::from(start..)),
            (false, true) => Some(Exposure::from(..=end)),
            (false, false) => None,
        }
    }

    /// Set the bloom effect scale.
    ///
    /// Non-finite or negative scale will be treated as 0.0.
    #[inline]
    pub fn set_bloom(&mut self, scale: f32) {
        let scale = scale.max(0.0);

        networked::write!(self, tonemap.bloom_scale, scale);
        networked::write!(self, tonemap.bloom_scale_enabled, scale != 0.0);
    }

    /// Set the exposure range.
    ///
    /// Non-finite or negative bounds will be treated as 0.0.
    #[inline]
    pub fn set_exposure<E: Into<Exposure>>(&mut self, exposure: Option<E>) {
        let (start, end) = exposure.map(Into::into).unwrap_or_default().to_tuple();

        networked::write!(self, tonemap.exposure_start, start);
        networked::write!(self, tonemap.exposure_end, end);

        networked::write!(self, tonemap.exposure_start_enabled, true);
        networked::write!(self, tonemap.exposure_end_enabled, true);
    }
}

// weapon
impl EntityRepr {
    #[inline]
    pub fn magazine(&self) -> Option<u32> {
        let magazine = networked::read!(self, base_combat_weapon.magazine);

        if magazine < 0 {
            None
        } else {
            Some(magazine as u32)
        }
    }

    #[inline]
    pub fn next_attack_time(&self) -> Duration {
        networked::read!(self, base_combat_weapon.next_primary_attack)
    }

    #[inline]
    pub fn revolver_cock_time(&self) -> Option<Duration> {
        let time = networked::read!(self, weapon_cs_base.revolver_cock_time);

        if time > Duration::from_secs_f32(3.4028235e+38) {
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
