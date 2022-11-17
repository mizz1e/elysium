use crate::{INetChannel, Ptr};
use bevy::prelude::*;
use std::ffi;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;

/// `engine/cdll_engine_int.cpp`.
#[derive(Resource)]
pub struct IVEngineClient {
    pub(crate) ptr: Ptr,
}

impl IVEngineClient {
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        let method: unsafe extern "C" fn(this: *mut u8, view_angle: *mut Vec3) =
            unsafe { self.ptr.vtable_entry(18) };

        let mut view_angle = Vec3::ZERO;

        unsafe {
            (method)(self.ptr.as_ptr(), &mut view_angle);
        }

        view_angle
    }

    #[inline]
    pub fn set_view_angle(&self, view_angle: Vec3) {
        let method: unsafe extern "C" fn(this: *mut u8, view_angle: *const Vec3) =
            unsafe { self.ptr.vtable_entry(19) };

        unsafe {
            (method)(self.ptr.as_ptr(), &view_angle);
        }
    }

    #[inline]
    pub fn level_name(&self) -> Box<OsStr> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(53) };

        let level_name = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!level_name.is_null());

        let level_name = unsafe { CStr::from_ptr(level_name).to_bytes() };

        Box::from(OsStr::from_bytes(level_name))
    }

    #[inline]
    pub fn local_player_index(&self) -> i32 {
        let method: unsafe extern "C" fn(this: *mut u8) -> i32 =
            unsafe { self.ptr.vtable_entry(12) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    #[inline]
    pub fn net_channel(&self) -> Option<INetChannel> {
        let method: unsafe extern "C" fn(this: *mut u8) -> *mut u8 =
            unsafe { self.ptr.vtable_entry(78) };

        let network_channel = unsafe { (method)(self.ptr.as_ptr()) };

        if network_channel.is_null() {
            None
        } else {
            let ptr = Ptr::new("INetChannel", network_channel)?;

            Some(INetChannel { ptr })
        }
    }
}
