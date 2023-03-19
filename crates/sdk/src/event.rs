use crate::{global, Ptr};
use bevy::prelude::*;

#[derive(Resource)]
pub struct FireGameEvent(pub(crate) unsafe extern "C" fn(this: *mut u8, event: *mut u8));

#[derive(Resource)]
pub struct EventManager {
    pub(crate) ptr: Ptr,
}

impl EventManager {
    pub(crate) unsafe fn setup(&self) {
        info!("setup event manager hooks");

        global::with_app_mut(|app| {
            app.insert_resource(FireGameEvent(self.ptr.vtable_replace(0, fire_game_event)));
        });
    }
}

unsafe extern "C" fn fire_game_event(this: *mut u8, event: *mut u8) {
    global::with_app_mut(|app| {
        trace!("event");

        let fire_game_event = app.world.resource::<FireGameEvent>().0;

        (fire_game_event)(this, event)
    })
}
