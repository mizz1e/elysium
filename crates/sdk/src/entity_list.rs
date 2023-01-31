use crate::{global, IClientEntity, Ptr};
use bevy::prelude::*;
use std::{ffi, ops};

#[derive(Resource)]
pub struct IClientEntityList {
    pub(crate) ptr: Ptr,
}

impl IClientEntityList {
    pub fn get(&self, index: ffi::c_int) -> Option<IClientEntity> {
        let method: unsafe extern "C" fn(this: *mut u8, index: ffi::c_int) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(3) };

        let ptr = unsafe { (method)(self.ptr.as_ptr(), index) };
        let ptr = Ptr::new("IClientEntity", ptr)?;

        Some(IClientEntity { ptr })
    }

    pub fn highest_index(&self) -> ffi::c_int {
        let method: unsafe extern "C" fn(this: *mut u8) -> ffi::c_int =
            unsafe { self.ptr.vtable_entry(6) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn players(&self) -> Entities<'_> {
        let mut entities = Vec::with_capacity(63);

        // 1 is the world
        for i in 1..=64 {
            let Some(player) = self.get(i) else {
                continue;
            };

            entities.push(player);
        }

        Entities {
            _entity_list: self,
            entities,
        }
    }
}

pub struct Entities<'a> {
    _entity_list: &'a IClientEntityList,
    entities: Vec<IClientEntity>,
}

impl<'a> ops::Deref for Entities<'a> {
    type Target = [IClientEntity];

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}
