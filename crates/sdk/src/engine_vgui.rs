use crate::{global, Ptr};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Paint(pub(crate) unsafe extern "C" fn(this: *mut u8, mode: i32));

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct EngineVGui {
    pub(crate) ptr: Ptr,
}

impl EngineVGui {
    pub(crate) unsafe fn setup(&self) {
        return;
        tracing::trace!("setup EngineVGui");

        global::with_app_mut(|app| {
            app.insert_resource(Paint(self.ptr.vtable_replace(15, paint)));
        });
    }
}

unsafe extern "C" fn paint(this: *mut u8, mode: i32) {
    // intentionally a no-op
}
