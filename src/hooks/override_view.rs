use crate::State;
use elysium_sdk::View;

/// `OverrideView` hook.
pub unsafe extern "C" fn override_view(this: *const u8, view: &mut View) {
    let state = State::get();
    let hooks = state.hooks.as_ref().unwrap_unchecked();

    view.angle = state.view_angle;

    (hooks.override_view)(this, view);
}
