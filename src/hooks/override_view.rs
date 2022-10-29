use crate::State;
use elysium_sdk::View;

/// `OverrideView` hook.
pub unsafe extern "C" fn override_view(this: *const u8, view: &mut View) {
    let state = State::get();
    let override_view_original = state.hooks.override_view.unwrap();

    view.angle = state.interfaces.as_ref().unwrap().engine.view_angle();
    //view.fov = 120.0;

    (override_view_original)(this, view);
}
