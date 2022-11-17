use crate::{global, Ptr};
use bevy::prelude::*;
use std::collections::HashMap;
use std::ffi;

type SourceIndex = ffi::c_int;

#[derive(Resource)]
pub struct IClientEntityList {
    pub(crate) ptr: Ptr,
}

impl IClientEntityList {
    fn get(&self, index: SourceIndex) -> *mut u8 {
        let method: unsafe extern "C" fn(this: *mut u8, index: SourceIndex) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(3) };

        unsafe { (method)(self.ptr.as_ptr(), index) }
    }

    fn highest_index(&self) -> SourceIndex {
        let method: unsafe extern "C" fn(this: *mut u8) -> SourceIndex =
            unsafe { self.ptr.vtable_entry(6) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }
}

#[derive(Resource)]
pub struct EntityMap(pub(crate) HashMap<ffi::c_int, *mut u8>);

unsafe impl Send for EntityMap {}
unsafe impl Sync for EntityMap {}

pub unsafe fn sync_entity_list() {
    global::with_app_mut(|app| {
        let world = app.world.cell();
        let entity_list = world.resource::<IClientEntityList>();
        let mut entity_map = world.resource_mut::<EntityMap>();
        let highest_index = entity_list.highest_index();

        for index in 0..=highest_index {
            let pointer = entity_list.get(index);

            if pointer.is_null() {
                entity_map.0.remove_entry(&index);
            } else {
                entity_map.0.insert(index, pointer);
            }
        }
    });
}
