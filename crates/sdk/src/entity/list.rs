use crate::{vtable_validate, Engine};
use core::ops::RangeInclusive;

#[repr(C)]
pub struct VTable {
    networkable: unsafe extern "thiscall" fn(this: *const EntityList, index: i32) -> *const u8,
    networkable_from_handle:
        unsafe extern "thiscall" fn(this: *const EntityList, handle: *const u8) -> *const u8,
    unknown_from_handle:
        unsafe extern "thiscall" fn(this: *const EntityList, handle: *const u8) -> *const u8,
    entity: unsafe extern "thiscall" fn(this: *const EntityList, index: i32) -> *const u8,
    entity_from_handle:
        unsafe extern "thiscall" fn(this: *const EntityList, handle: *const u8) -> *const u8,
    number_of_entities:
        unsafe extern "thiscall" fn(this: *const EntityList, include_non_networked: bool) -> i32,
    highest_entity_index: unsafe extern "thiscall" fn(this: *const EntityList) -> i32,
    set_max_entities: unsafe extern "thiscall" fn(this: *const EntityList, max: i32),
    max_entities: unsafe extern "thiscall" fn(this: *const EntityList) -> i32,
}

vtable_validate! {
    networkable => 0,
    networkable_from_handle => 1,
    unknown_from_handle => 2,
    entity => 3,
    entity_from_handle => 4,
    number_of_entities => 5,
    highest_entity_index => 6,
    set_max_entities => 7,
    max_entities => 8,
}

/// Entity list interface.
///
/// NOTE: Using this in `create_move` seems to crash the game.
#[repr(C)]
pub struct EntityList {
    vtable: &'static VTable,
}

impl EntityList {
    #[inline]
    pub fn networkable(&self, index: i32) -> *const u8 {
        unsafe { (self.vtable.networkable)(self, index) }
    }

    #[inline]
    pub fn networkable_from_handle(&self, handle: *const u8) -> *const u8 {
        unsafe { (self.vtable.networkable_from_handle)(self, handle) }
    }

    #[inline]
    pub fn unknown_from_handle(&self, handle: *const u8) -> *const u8 {
        unsafe { (self.vtable.unknown_from_handle)(self, handle) }
    }

    #[inline]
    pub fn entity(&self, index: i32) -> *const u8 {
        unsafe { (self.vtable.entity)(self, index) }
    }

    #[inline]
    pub fn entity_from_handle(&self, handle: *const u8) -> *const u8 {
        unsafe { (self.vtable.entity_from_handle)(self, handle) }
    }

    #[inline]
    pub fn number_of_entities(&self, include_non_networked: bool) -> i32 {
        unsafe { (self.vtable.number_of_entities)(self, include_non_networked) }
    }

    #[inline]
    pub fn highest_entity_index(&self) -> i32 {
        unsafe { (self.vtable.highest_entity_index)(self) }
    }

    #[inline]
    pub unsafe fn set_max_entities(&self, max: i32) {
        (self.vtable.set_max_entities)(self, max)
    }

    #[inline]
    pub fn max_entities(&self) -> i32 {
        unsafe { (self.vtable.max_entities)(self) }
    }

    #[inline]
    pub fn local_player(&self, engine: &Engine) -> *const u8 {
        self.entity(engine.local_player_index())
    }

    #[inline]
    pub fn player_range(&self) -> RangeInclusive<i32> {
        1..=64
    }

    #[inline]
    pub fn non_player_range(&self) -> RangeInclusive<i32> {
        65..=self.highest_entity_index()
    }
}
