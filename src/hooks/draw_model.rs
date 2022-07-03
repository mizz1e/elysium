use elysium_sdk::Interfaces;
use elysium_sdk::model::ModelRender;
use crate::state;

pub unsafe extern "C" fn draw_model(
    this: *const u8,
    context: *const u8,
    state: *const u8,
    info: *const u8,
    bone_to_world: *const u8,
) {
    let interfaces = &*state::interfaces().cast::<Interfaces>();
    let model_render = &*interfaces.model_render.cast::<ModelRender>();

    model_render.reset_material();
    state::hooks::draw_model(this, context, state, info, bone_to_world)
}
