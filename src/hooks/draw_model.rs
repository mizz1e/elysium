use crate::entity::{Entity, Player, PlayerRef};
use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::entity::Team;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};

#[inline]
unsafe fn draw_model_inner(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) -> Option<()> {
    let state = State::get();
    let draw_model_original = state.hooks.draw_model?;
    let interfaces = state.interfaces.as_ref()?;
    let entity_list = &interfaces.entity_list;
    let model_info = &interfaces.model_info;
    let model_render = this.as_ref()?;
    let material = state.materials.gold?;

    let info = info.as_ref()?;
    let name = info.name(&model_info)?;

    if name.starts_with("models/player") {
        let index = info.entity_index;
        let player = PlayerRef::from_raw(entity_list.entity(index))?;
        let local = PlayerRef::from_raw(state.local.player)?;

        if index == local.index() {
            model_render.reset_material();
            (draw_model_original)(this, context, draw_state, info, state.local.bones.as_ptr());
        } else {
            let rgba = match player.team() {
                Team::Counter => [0.0, 1.0, 1.0, 0.5],
                Team::Terrorist => [1.0, 0.0, 0.0, 0.5],
                _ => [1.0, 1.0, 1.0, 0.5],
            };

            material.set_rgba(rgba);

            model_render.override_material(material);
            (draw_model_original)(this, context, draw_state, info, bone_to_world);
            model_render.reset_material();
        }
    } else if name.starts_with("models/weapons/v_") {
        (draw_model_original)(this, context, draw_state, info, bone_to_world);
    } else {
        (draw_model_original)(this, context, draw_state, info, bone_to_world);
    }

    Some(())
}

/// `DrawModelExecute` hook.
pub unsafe extern "C" fn draw_model(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    draw_model_inner(this, context, draw_state, info, bone_to_world);
}
