use crate::entity::{Entity, Player, PlayerRef};
use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::entity::Team;
use elysium_sdk::material::{Material, MaterialFlag, MaterialKind};
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};
use elysium_sdk::MaterialSystem;
use elysium_sdk::Vdf;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.4, 0.0, 1.0, 0.4];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.5];

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

    let flat = state.materials.get(MaterialKind::Flat, material_system);
    let glow = state.materials.get(MaterialKind::Glow, material_system);

    let info = info.as_ref()?;
    let name = info.name(&model_info)?;

    if name.starts_with("models/player") {
        let index = info.entity_index;
        let player = PlayerRef::from_raw(entity_list.entity(index))?;
        let local = PlayerRef::from_raw(state.local.player)?;

        if index == local.index() {
            flat.set_rgba(BLACK);
            glow.set_rgba(PURPLE);

            flat.set_flag(MaterialFlag::IGNORE_Z, false);
            glow.set_flag(MaterialFlag::IGNORE_Z, false);

            model_render.override_material(flat);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.override_material(glow);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.reset_material();
        } else {
            let (rgba, ignore_z) = match player.is_enemy() {
                false => (PURPLE, false),
                true => (RED, true),
                _ => ([1.0, 1.0, 1.0, 0.5], false),
            };

            flat.set_rgba(BLACK);
            glow.set_rgba(rgba);

            flat.set_flag(MaterialFlag::IGNORE_Z, ignore_z);
            glow.set_flag(MaterialFlag::IGNORE_Z, ignore_z);

            model_render.override_material(flat);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.override_material(glow);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.reset_material();

            // reset
            glow.set_rgba(PURPLE);
            glow.set_flag(MaterialFlag::IGNORE_Z, false);
        }
    } else if name.starts_with("models/weapons/v_") {
        flat.set_rgba(BLACK);
        glow.set_rgba(PURPLE);

        flat.set_flag(MaterialFlag::IGNORE_Z, false);
        glow.set_flag(MaterialFlag::IGNORE_Z, false);

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    } else {
        flat.set_rgba(BLACK);
        glow.set_rgba(PURPLE);

        flat.set_flag(MaterialFlag::IGNORE_Z, false);
        glow.set_flag(MaterialFlag::IGNORE_Z, false);

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    }

    use palette::{Hsl, Hue, IntoColor, Pixel, Srgb};

    let rgb: Hsl = Srgb::new(1.0, 0.0, 0.0).into_color();
    let rgb = rgb.shift_hue(state.init_time.unwrap().elapsed().as_secs_f32() * 100.0);
    let rgb: Srgb = rgb.into_color();
    let [r, g, b]: [f32; 3] = rgb.into_raw();

    glow.set_rgba([r, g, b, 1.0]);

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
