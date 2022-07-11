use crate::state::Hooks;
use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::material::Material;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::Interfaces;

unsafe fn draw_layer(
    this: &ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
    hooks: &Hooks,
    material: &Material,
) {
    this.override_material(material, 0, -1);
    (hooks.draw_model)(this, context, draw_state, info, bone_to_world);
    this.reset_material();
}

#[inline(never)]
pub unsafe extern "C" fn draw_model(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    let state = State::get();
    let hooks = state.hooks.as_ref().unwrap_unchecked();
    let interfaces = state.interfaces.as_ref().unwrap_unchecked();
    let local_vars = &state.local;
    let Interfaces {
        entity_list,
        model_info,
        ..
    } = interfaces;

    let model_name = model_info.model_name(&*(*info).model);

    if let Some(gold) = state.materials.gold {
        if model_name.starts_with("models/player") {
            let entity_index = (*info).entity_index;
            let _entity = entity_list.entity(entity_index);
            let local = &*local_vars.player;

            use elysium_sdk::material::MaterialFlag;

            gold.set_rgba([1.0, 1.0, 1.0, 0.9]);

            if local.index() == entity_index {
                if local.is_scoped() && local_vars.thirdperson.0 {
                    gold.set_alpha(0.05);
                }
            }

            //gold.set_flag(MaterialFlag::NO_CULL, true);
            gold.set_flag(MaterialFlag::IGNORE_Z, true);
            gold.set_flag(MaterialFlag::WIREFRAME, true);

            draw_layer(
                &*this,
                context,
                draw_state,
                info,
                bone_to_world,
                hooks,
                gold,
            );

            return;
        } else if model_name.starts_with("models/weapons/v_") {
            draw_layer(
                &*this,
                context,
                draw_state,
                info,
                bone_to_world,
                hooks,
                gold,
            );

            return;
        } else {
            draw_layer(
                &*this,
                context,
                draw_state,
                info,
                bone_to_world,
                hooks,
                gold,
            );

            return;
        }
    }

    (hooks.draw_model)(this, context, draw_state, info, bone_to_world);
}
