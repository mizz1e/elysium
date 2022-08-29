use crate::entity::{Entity, Player, PlayerRef};
use crate::state::DrawModel;
use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::entity::Team;
use elysium_sdk::material::{Material, MaterialFlag};
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::Interfaces;

unsafe fn draw_layer(
    this: &ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
    draw_model_original: &DrawModel,
    material: &Material,
) {
    this.override_material(material, 0, -1);
    (draw_model_original)(this, context, draw_state, info, bone_to_world);
    this.reset_material();
}

/// `DrawModelExecute` hook.
#[inline(never)]
pub unsafe extern "C" fn draw_model(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    let state = State::get();
    let draw_model_original = state.hooks.draw_model.unwrap();
    let local_vars = &state.local;
    let Interfaces {
        entity_list,
        model_info,
        ..
    } = state.interfaces.as_ref().unwrap();

    let info = info.as_ref().unwrap();
    let model = info.model.as_ref().unwrap();
    let model_name = model_info.model_name(model);

    if let Some(gold) = state.materials.gold {
        gold.set_rgba([1.0, 1.0, 1.0, 0.9]);
        gold.set_flag(MaterialFlag::IGNORE_Z, false);
        gold.set_flag(MaterialFlag::WIREFRAME, false);

        if model_name.starts_with("models/player") {
            let entity_index = (*info).entity_index;
            let entity = entity_list.entity(entity_index);
            let local = PlayerRef::from_raw(local_vars.player).unwrap();

            if !entity.is_null() {
                let entity = PlayerRef::from_raw(entity).unwrap();

                match entity.team() {
                    Team::Counter => gold.set_rgba([0.0, 0.5, 1.0, 0.9]),
                    Team::Terrorist => gold.set_rgba([1.0, 0.0, 1.0, 0.9]),
                    _ => {}
                }
            }

            if local.index() == entity_index {
                if local.is_scoped() && local_vars.thirdperson.0 {
                    gold.set_alpha(0.05);
                }

                if state.view_angle.x < 0.0 {
                    gold.set_alpha(0.05);
                }
            }

            gold.set_flag(MaterialFlag::IGNORE_Z, true);
            //gold.set_flag(MaterialFlag::WIREFRAME, true);

            draw_layer(
                &*this,
                context,
                draw_state,
                info,
                bone_to_world,
                &draw_model_original,
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
                &draw_model_original,
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
                &draw_model_original,
                gold,
            );

            return;
        }
    }

    (draw_model_original)(this, context, draw_state, info, bone_to_world);
}
