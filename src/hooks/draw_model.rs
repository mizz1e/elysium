use crate::entity::{Entity, Player, PlayerRef};
use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::entity::Team;
use elysium_sdk::material::{Material, MaterialFlag, MaterialKind};
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::MaterialSystem;
use elysium_sdk::Vdf;

#[inline]
unsafe fn draw_model_inner(
    model_render: &mut ModelRender,
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
    let material_system = &interfaces.material_system;

    let flat = state
        .materials
        .flat
        .get_or_insert_with(|| material_system.from_kind(MaterialKind::Flat).unwrap());

    let glow = state
        .materials
        .glow
        .get_or_insert_with(|| material_system.from_kind(MaterialKind::Glow).unwrap());

    let info = info.as_ref()?;
    let name = info.name(&model_info)?;

    if name.starts_with("models/player") {
        let index = info.entity_index;
        let player = PlayerRef::from_raw(entity_list.entity(index))?;
        let local = PlayerRef::from_raw(state.local.player)?;

        if index == local.index() {
            model_render.reset_material();

            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        } else {
            let (rgba, ignore_z) = match player.team() {
                Team::Counter => ([0.0, 1.0, 1.0, 0.5], false),
                Team::Terrorist => ([1.0, 0.0, 0.0, 0.5], true),
                _ => ([1.0, 1.0, 1.0, 0.5], false),
            };

            flat.set_rgba([0.0, 0.0, 0.0, 1.0]);
            glow.set_rgba(rgba);

            flat.set_flag(MaterialFlag::IGNORE_Z, ignore_z);
            glow.set_flag(MaterialFlag::IGNORE_Z, ignore_z);

            model_render.override_material(flat);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.override_material(glow);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.reset_material();
        }
    } else if name.starts_with("models/weapons/v_") {
        flat.set_rgba([0.0, 0.0, 0.0, 1.0]);
        glow.set_rgba([0.7, 0.0, 1.0, 0.4]);

        flat.set_flag(MaterialFlag::IGNORE_Z, false);
        glow.set_flag(MaterialFlag::IGNORE_Z, false);

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    } else {
        flat.set_rgba([0.0, 0.0, 0.0, 1.0]);
        glow.set_rgba([0.7, 0.0, 1.0, 0.4]);

        flat.set_flag(MaterialFlag::IGNORE_Z, false);
        glow.set_flag(MaterialFlag::IGNORE_Z, false);

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    }

    Some(())
}

/// `DrawModelExecute` hook.
pub unsafe extern "C" fn draw_model(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    draw_model_inner(model_render, context, draw_state, info, bone_to_world);
}
