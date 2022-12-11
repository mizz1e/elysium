use crate::{global, Config, IVEngineClient, Ptr};
use bevy::prelude::*;

#[derive(Resource)]
pub struct LockCursor(pub(crate) unsafe extern "C" fn(this: *mut u8));

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct Surface {
    pub(crate) ptr: Ptr,
}

impl Surface {
    pub(crate) unsafe fn setup(&self) {
        tracing::trace!("setup Surface");

        global::with_app_mut(|app| {
            app.insert_resource(LockCursor(self.ptr.vtable_replace(67, lock_cursor)));
        });
    }

    pub fn lock_cursor(&self) {
        let method = global::with_resource::<LockCursor, _>(|f| f.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn unlock_cursor(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(66) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }
}

unsafe extern "C" fn lock_cursor(this: *mut u8) {
    global::with_app(|app| {
        let config = app.world.resource::<Config>();
        let engine = app.world.resource::<IVEngineClient>();
        let surface = app.world.resource::<Surface>();

        if config.menu_open && !engine.is_in_game() {
            tracing::trace!("unlock");
            surface.unlock_cursor();
        }

        surface.lock_cursor();
    });
}
