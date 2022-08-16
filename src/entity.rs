use crate::{Networked, State};
use cake::ffi::VTablePad;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, Renderable, Team};
use elysium_sdk::{object_validate, vtable_validate, WeaponInfo};
use std::marker::PhantomData;
use std::ops;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<12>,
    origin: unsafe extern "thiscall" fn(this: *const Entity) -> *const Vec3,
    _pad1: VTablePad<98>,
    set_model_index: unsafe extern "thiscall" fn(this: *const Entity, index: i32),
    _pad2: VTablePad<10>,
    attachment:
        unsafe extern "thiscall" fn(this: *const Entity, index: i32, origin: *const Vec3) -> bool,
    _pad3: VTablePad<5>,
    team: unsafe extern "thiscall" fn(this: *const Entity) -> Team,
    _pad4: VTablePad<38>,
    health: unsafe extern "thiscall" fn(this: *const Entity) -> i32,
    _pad5: VTablePad<40>,
    is_alive: unsafe extern "thiscall" fn(this: *const Entity) -> bool,
    _pad6: VTablePad<1>,
    is_player: unsafe extern "thiscall" fn(this: *const Entity) -> bool,
    _pad7: VTablePad<7>,
    is_weapon: unsafe extern "thiscall" fn(this: *const Entity) -> bool,
    _pad8: VTablePad<112>,
    active_weapon: unsafe extern "thiscall" fn(this: *const Entity) -> *const Entity,
    _pad9: VTablePad<16>,
    eye_pos: unsafe extern "thiscall" fn(this: *const Entity) -> Vec3,
    _pad10: VTablePad<1>,
    weapon_sub_kind: unsafe extern "thiscall" fn(this: *const Entity) -> i32,
    _pad11: VTablePad<6>,
    observer_mode: unsafe extern "thiscall" fn(this: *const Entity) -> ObserverMode,
    observer_target: unsafe extern "thiscall" fn(this: *const Entity) -> *const Entity,
    _pad12: VTablePad<50>,
    aim_punch: unsafe extern "thiscall" fn(this: *const Entity) -> Vec3,
    _pad13: VTablePad<62>,
    draw_crosshair: unsafe extern "thiscall" fn(this: *const Entity),
    _pad14: VTablePad<48>,
    spread: unsafe extern "thiscall" fn(this: *const Entity) -> f32,
    _pad15: VTablePad<1>,
    weapon_kind: unsafe extern "thiscall" fn(this: *const Entity) -> i32,
    _pad16: VTablePad<5>,
    weapon_data: unsafe extern "thiscall" fn(this: *const Entity) -> *const WeaponInfo,
    _pad17: VTablePad<6>,
    muzzle_attachment_index_1st:
        unsafe extern "thiscall" fn(this: *const Entity, view_model: *const Entity) -> i32,
    muzzle_attachment_index_3rd: unsafe extern "thiscall" fn(this: *const Entity) -> i32,
    _pad18: VTablePad<13>,
    inaccuracy: unsafe extern "thiscall" fn(this: *const Entity) -> f32,
    update_accuracy_penalty: unsafe extern "thiscall" fn(this: *const Entity),
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
pub struct Entity {
    vtable: &'static VTable,
    pub renderable: Renderable,
    pub networkable: Networkable,
}

#[derive(Clone, Copy)]
pub struct EntityRef<'a> {
    entity: *const Entity,
    _phantom: PhantomData<&'a Entity>,
}

impl<'a> EntityRef<'a> {
    #[inline]
    pub unsafe fn from_raw(entity: *const Entity) -> Self {
        let _phantom = PhantomData;

        Self { entity, _phantom }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const Entity {
        self.entity
    }

    #[inline]
    pub fn as_entity(&self) -> &'a Entity {
        unsafe { &*self.entity }
    }
}

impl<'a> ops::Deref for EntityRef<'a> {
    type Target = Entity;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_entity()
    }
}

object_validate! {
    Entity;
    vtable => 0,
    renderable => 8,
    networkable => 16,
}

impl Entity {
    /// the entity's class
    #[inline]
    pub fn client_class(&self) -> *const u8 {
        self.networkable.client_class()
    }

    /// is the entity dormant
    #[inline]
    pub fn is_dormant(&self) -> bool {
        self.networkable.is_dormant()
    }

    /// the entity's index
    #[inline]
    pub fn index(&self) -> i32 {
        self.networkable.index()
    }

    /// the entity's model
    #[inline]
    pub fn model(&self) -> *const u8 {
        self.renderable.model()
    }

    /// setup bones
    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        self.renderable.setup_bones(bones, mask, time)
    }

    /// should draw?
    #[inline]
    pub fn should_draw(&self) -> bool {
        self.renderable.should_draw()
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        unsafe { *(self.vtable.origin)(self) }
    }

    #[inline]
    pub fn set_model_index(&self, index: i32) {
        unsafe { (self.vtable.set_model_index)(self, index) }
    }

    #[inline]
    pub fn attachment(&self, index: i32, origin: *const Vec3) -> bool {
        unsafe { (self.vtable.attachment)(self, index, origin) }
    }

    /// only for base_players
    #[inline]
    pub fn team(&self) -> Team {
        unsafe { (self.vtable.team)(self) }
    }

    #[inline]
    pub fn health(&self) -> i32 {
        unsafe { (self.vtable.health)(self) }
    }

    #[inline]
    pub fn is_alive(&self) -> bool {
        unsafe { (self.vtable.is_alive)(self) }
    }

    #[inline]
    pub fn is_player(&self) -> bool {
        unsafe { (self.vtable.is_player)(self) }
    }

    #[inline]
    pub fn is_weapon(&self) -> bool {
        unsafe { (self.vtable.is_weapon)(self) }
    }

    /// only for base_players
    #[inline]
    pub fn active_weapon(&self) -> *const Entity {
        unsafe { (self.vtable.active_weapon)(self) }
    }

    #[inline]
    pub fn eye_pos(&self) -> Vec3 {
        unsafe { (self.vtable.eye_pos)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn weapon_sub_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_sub_kind)(self) }
    }

    #[inline]
    pub fn observer_mode(&self) -> ObserverMode {
        unsafe { (self.vtable.observer_mode)(self) }
    }

    #[inline]
    pub fn observer_target(&self) -> *const Entity {
        unsafe { (self.vtable.observer_target)(self) }
    }

    #[inline]
    pub fn aim_punch(&self) -> Vec3 {
        unsafe { (self.vtable.aim_punch)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn draw_crosshair(&self) {
        unsafe { (self.vtable.draw_crosshair)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn spread(&self) -> f32 {
        unsafe { (self.vtable.spread)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn weapon_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_kind)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn weapon_data(&self) -> *const WeaponInfo {
        unsafe { (self.vtable.weapon_data)(self) }
    }

    #[inline]
    pub fn muzzle_attachment_index_1st(&self, view_model: *const Entity) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_1st)(self, view_model) }
    }

    #[inline]
    pub fn muzzle_attachment_index_3rd(&self) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_3rd)(self) }
    }

    /// only for base_weapon
    #[inline]
    pub fn inaccuracy(&self) -> f32 {
        unsafe { (self.vtable.inaccuracy)(self) }
    }

    #[inline]
    pub fn update_accuracy_penalty(&self) {
        unsafe { (self.vtable.update_accuracy_penalty)(self) }
    }

    /// networked variable
    #[inline]
    fn networked<T, F>(&self, f: F) -> &mut T
    where
        F: Fn(&Networked) -> usize,
    {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let state = State::get();
            let offset = f(&state.networked);

            &mut *(this.byte_add(offset) as *mut T)
        }
    }

    /// only for base_entity
    #[inline]
    fn render_mode_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_entity.render_mode)
    }

    /// only for base_player
    #[inline]
    pub fn move_kind(&self) -> MoveKind {
        unsafe { *self.render_mode_address().byte_add(1).cast() }
    }

    /// only for base_players=
    #[inline]
    fn is_dead_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_player.is_dead)
    }

    /// only for base_player
    #[inline]
    pub fn view_angle(&self) -> &mut Vec3 {
        unsafe {
            let view_angle_address = self.is_dead_address().byte_add(4) as *mut Vec3;

            &mut *view_angle_address
        }
    }

    /// only for base_player
    #[inline]
    pub fn velocity(&self) -> Vec3 {
        *self.networked(|networked| networked.base_player.velocity)
    }

    /// base player view offset
    #[inline]
    pub fn view_offset(&self) -> Vec3 {
        *self.networked(|networked| networked.base_player.view_offset)
    }

    /// player armor value
    #[inline]
    pub fn armor(&self) -> i32 {
        *self.networked(|networked| networked.player.armor)
    }

    /// player flags
    #[inline]
    pub fn flags(&self) -> i32 {
        *self.networked(|networked| networked.player.flags)
    }

    /// does player have a helmet
    #[inline]
    pub fn has_helmet(&self) -> bool {
        *self.networked(|networked| networked.player.has_helmet)
    }

    /// is player defusing
    #[inline]
    pub fn is_defusing(&self) -> bool {
        *self.networked(|networked| networked.player.is_defusing)
    }

    /// is player scoped
    #[inline]
    pub fn is_scoped(&self) -> bool {
        *self.networked(|networked| networked.player.is_scoped)
    }

    /// player lower body yaw
    #[inline]
    pub fn lby(&self) -> i32 {
        *self.networked(|networked| networked.player.lower_body_yaw)
    }

    /// only for base player
    #[inline]
    pub fn eye_origin(&self) -> Vec3 {
        let origin = self.origin();
        let view_offset = self.view_offset();

        let z = if self.flags() & (1 << 1) != 0 {
            46.0
        } else {
            64.0
        };

        let view_offset = if view_offset == Vec3::zero() {
            Vec3::from_xyz(0.0, 0.0, z)
        } else {
            view_offset
        };

        origin + view_offset
    }

    /// only for fog
    #[inline]
    pub fn is_enabled(&self) -> &mut bool {
        self.networked(|networked| networked.fog.is_enabled)
    }

    /// only for fog
    #[inline]
    pub fn start_distance(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.start)
    }

    /// only for fog
    #[inline]
    pub fn end_distance(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.end)
    }

    /// only for fog
    #[inline]
    pub fn far_z(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.far_z)
    }

    /// only for fog
    #[inline]
    pub fn density(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.density)
    }

    /// only for fog
    #[inline]
    pub fn direction(&self) -> &mut Vec3 {
        self.networked(|networked| networked.fog.direction)
    }

    /// only for fog
    #[inline]
    pub fn color_primary(&self) -> &mut i32 {
        self.networked(|networked| networked.fog.color_primary)
    }

    /// only for fog
    #[inline]
    pub fn color_secondary(&self) -> &mut i32 {
        self.networked(|networked| networked.fog.color_secondary)
    }

    /// only for fog
    #[inline]
    pub fn hdr_scale(&self) -> &mut f32 {
        self.networked(|networked| networked.fog.hdr_scale)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_min_exposure(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_min_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_max_exposure(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_max_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn enable_bloom_scale(&self) -> &mut bool {
        self.networked(|networked| networked.tonemap.enable_bloom_scale)
    }

    /// only for tonemap
    #[inline]
    pub fn min_exposure(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.min_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn max_exposure(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.max_exposure)
    }

    /// only for tonemap
    #[inline]
    pub fn bloom_scale(&self) -> &mut f32 {
        self.networked(|networked| networked.tonemap.bloom_scale)
    }

    /// only for base_weapon
    #[inline]
    pub fn magazine(&self) -> Option<u16> {
        let magazine: i32 = *self.networked(|networked| networked.base_weapon.magazine);

        if magazine < 0 {
            None
        } else {
            Some(magazine as u16)
        }
    }

    /// only for base_weapon
    #[inline]
    pub fn next_attack_time(&self) -> &mut f32 {
        self.networked(|networked| networked.base_weapon.next_attack_time)
    }

    /// only for weapon
    #[inline]
    pub fn revolver_cock_time(&self) -> Option<f32> {
        let time: f32 = *self.networked(|networked| networked.weapon.revolver_cock_time);

        if time > 3.4028235e+38 {
            None
        } else {
            Some(time)
        }
    }
}
