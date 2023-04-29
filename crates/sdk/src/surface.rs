use crate::{global, ptr, Config, IVEngineClient, InputStackSystem, Ptr};
use bevy::prelude::*;
use std::slice;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum Cursor {
    User,
    None,
    Arrow,
    IBeam,
    Hourglass,
    WaitArrow,
    Crosshair,
    Up,
    SizeNwse,
    SizeNesw,
    SizeWe,
    SizeNs,
    SizeAll,
    No,
    Hand,
    Blank,
}

#[derive(Resource)]
pub struct CalculateMouseVisible(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct SetCursor(pub(crate) unsafe extern "C" fn(this: *mut u8, cursor: Cursor));

#[derive(Resource)]
pub struct LockCursor(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct UnlockCursor(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct IsCursorLocked(pub(crate) unsafe extern "C" fn(this: *mut u8) -> bool);

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct Surface {
    pub(crate) ptr: Ptr,
}

impl Surface {
    pub(crate) unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(SetCursor(self.ptr.vtable_replace(57, set_cursor)));
            app.insert_resource(UnlockCursor(self.ptr.vtable_replace(66, unlock_cursor)));
            app.insert_resource(LockCursor(self.ptr.vtable_replace(67, lock_cursor)));
            app.insert_resource(IsCursorLocked(
                self.ptr.vtable_replace(108, is_cursor_locked),
            ));
        });
    }

    pub fn input_context(&self) -> *mut u8 {
        unsafe { *self.ptr.byte_offset::<*mut u8>(1000) }
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        let method = global::with_resource::<SetCursor, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr(), cursor) }
    }

    pub fn lock_cursor(&self) {
        let method = global::with_resource::<LockCursor, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn unlock_cursor(&self) {
        let method = global::with_resource::<UnlockCursor, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn is_cursor_locked(&self) -> bool {
        let method = global::with_resource::<IsCursorLocked, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }
}

unsafe extern "C" fn set_cursor(this: *mut u8, mut cursor: Cursor) {
    debug_assert!(!this.is_null());

    // Might need to do something extra here?

    global::with_app(|app| {
        let surface = app.world.resource::<Surface>();

        surface.set_cursor(cursor);
    });
}

unsafe extern "C" fn lock_cursor(this: *mut u8) {
    debug_assert!(!this.is_null());

    // Intentional no-op. Prevent uncontrolled control.
}

unsafe extern "C" fn unlock_cursor(this: *mut u8) {
    debug_assert!(!this.is_null());

    // Intentional no-op. Prevent uncontrolled control.
}

unsafe extern "C" fn is_cursor_locked(this: *mut u8) -> bool {
    debug_assert!(!this.is_null());

    global::with_app(|app| {
        let config = app.world.resource::<Config>();
        let engine = app.world.resource::<IVEngineClient>();

        if !engine.is_in_game() || config.menu_open {
            return false;
        }

        true
    })
}
