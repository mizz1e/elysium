use crate::State;
use elysium_sdk::View;

/// `OverrideView` hook.
pub unsafe extern "C" fn override_view(this: *const u8, view: &mut View) {
    let state = State::get();
    let engine = &state.interfaces.as_ref().unwrap().engine;
    let override_view_original = state.hooks.override_view.unwrap();

    view.angle = engine.view_angle();

    (override_view_original)(this, view);
}
