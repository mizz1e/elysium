use crate::State;
use elysium_sdk::Interfaces;

pub unsafe extern "C" fn draw_model(
    this: *const u8,
    context: *mut u8,
    state: *const u8,
    info: *const u8,
    bone_to_world: *const u8,
) {
    let gstate = State::get();
    let hooks = gstate.hooks.as_mut().unwrap_unchecked();
    let Interfaces {
        material_system,
        model_render,
        ..
    } = gstate.interfaces.as_ref().unwrap_unchecked();

    if let Some(gold) = gstate.materials.gold {
        model_render.override_material(gold);
        (hooks.draw_model)(this, context, state, info, bone_to_world);
        model_render.reset_material();
    } else {
        (hooks.draw_model)(this, context, state, info, bone_to_world);
    }
}
