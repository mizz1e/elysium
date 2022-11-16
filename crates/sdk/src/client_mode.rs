use crate::{global, CUserCmd, CViewSetup, Ptr};
use bevy::prelude::*;

#[derive(Resource)]
pub struct OverrideView(pub(crate) unsafe extern "C" fn(this: *mut u8, setup: *const CViewSetup));

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct IClientMode {
    pub(crate) ptr: Ptr,
}

impl IClientMode {
    pub(crate) unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(OverrideView(self.ptr.vtable_replace(19, override_view)));

            self.ptr.vtable_replace(25, create_move);
        });
    }
}

unsafe extern "C" fn override_view(this: *mut u8, setup: *const CViewSetup) {
    let method = global::with_app(|app| app.world.resource::<OverrideView>().0);

    (method)(this, setup)
}

unsafe extern "C" fn create_move(
    this: *mut u8,
    input_sample_time: f32,
    command: *mut CUserCmd,
) -> bool {
    false
}
