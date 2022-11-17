use crate::{global, CUserCmd, CViewSetup, Ptr};
use bevy::prelude::*;
use std::ptr;

#[derive(Resource)]
pub struct OverrideView(pub(crate) unsafe extern "C" fn(this: *mut u8, setup: *const CViewSetup));

// rustfmt breaks with "trailing space left behind" after `pub(crate)`
type CreateMoveFn =
    unsafe extern "C" fn(this: *mut u8, input_sample_time: f32, command: *mut CUserCmd) -> bool;

#[derive(Resource)]
pub struct CreateMove(pub(crate) CreateMoveFn);

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct IClientMode {
    pub(crate) ptr: Ptr,
}

impl IClientMode {
    #[inline]
    pub(crate) unsafe fn setup(&self) {
        tracing::trace!("setup IClientMode");

        global::with_app_mut(|app| {
            app.insert_resource(OverrideView(self.ptr.vtable_replace(19, override_view)));

            // rustc apparently is a little too overzealous with it's optimization, and
            // deletes this hook if the result is unused?
            app.insert_resource(CreateMove(self.ptr.vtable_replace(25, create_move)));
        });
    }
}

unsafe extern "C" fn override_view(this: *mut u8, setup: *const CViewSetup) {
    debug_assert!(!this.is_null());
    debug_assert!(!setup.is_null());

    tracing::trace!("override view");

    let method = global::with_app(|app| app.world.resource::<OverrideView>().0);

    (method)(this, setup)
}

unsafe extern "C" fn create_move(
    this: *mut u8,
    input_sample_time: f32,
    command: *mut CUserCmd,
) -> bool {
    debug_assert!(!this.is_null());
    debug_assert!(!command.is_null());

    let command = &mut *command;

    // ignore input sampling
    if command.number == 0 {
        return false;
    }

    tracing::trace!("create move");

    false
}
