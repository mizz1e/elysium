use crate::entity::{Entity, EntityRef, Fog, FogRef, Player, PlayerRef, Tonemap, TonemapRef};
use crate::state::Local;
use crate::{state, State};
use elysium_sdk::entity::EntityId;
use elysium_sdk::{material, Engine, EntityList, Frame, Globals, Input, Interfaces, Vars};
use std::ffi;

fn update_vars(vars: &mut Vars, _engine: &Engine) {
    let state = State::get();

    state.ffa = vars.ffa.read();

    vars.cheats.write(true);
    vars.hud.write(false);
    vars.vgui.write(false);

    // useful
    vars.show_grenade_path.write(true);

    // annoying
    vars.auto_help.write(false);
    vars.show_help.write(false);

    // these disable when true
    vars.engine_sleep.write(true);
    vars.html_motd.write(true);
    vars.freeze_cam.write(true);
    vars.panorama_blur.write(true);
}

/// Override fog controller properties.
fn update_fog(mut fog: FogRef<'_>) {
    let state = State::get();

    fog.set_clip_distance(state.fog_clip);
    fog.set_range(Some(state.fog_start..=state.fog_end));
    fog.set_rgba(state.fog);
}

/// Override tonemap controller properties.
fn update_tonemap(mut tonemap: TonemapRef<'_>) {
    let state = State::get();

    tonemap.set_bloom(state.bloom);
    tonemap.set_exposure(Some(state.exposure_min..=state.exposure_max));
}

/// Thirdperson handling.
fn update_thirdperson(
    globals: &Globals,
    input: &Input,
    local_vars: &mut Local,
    local: &mut PlayerRef<'_>,
) {
    let state = State::get();

    state.original_view_angle = local.view_angle();

    let engine_view_angle = state.interfaces.as_ref().unwrap().engine.view_angle();

    if input.thirdperson {
        let mut view_angle = local_vars.view_angle;

        // roll isnt networked to others, so don't visualize it, either
        view_angle.z = 0.0;

        // fix the local player's view_angle when in thirdperson
        unsafe {
            local.set_view_angle(view_angle);
        }
    } else {
        // in cooperation with override_view, this will change the view model's position.
        if local_vars.visualize_shot != 0.0 {
            if local_vars.visualize_shot > globals.current_time {
                unsafe {
                    local.set_view_angle(local_vars.shot_view_angle);
                }
            } else {
                unsafe {
                    local.set_view_angle(state.view_angle);
                }
                local_vars.visualize_shot = 0.0;
            }
        }

        let mut view_angle = engine_view_angle;

        // rotate view model
        view_angle.z = -35.0;

        unsafe {
            local.set_view_angle(view_angle);
        }
    }
}

/*unsafe fn update_precipitation() {
    let state = State::get();
    let Interfaces {
        entity_list,
        ..
    } = state.interfaces.as_ref().unwrap();

    let precipitation_networkable = precpitation_class.new(MAX_EDICTS - 1, 0);

    let entity = entity_list.entity(MAX_EDICTS - 1);
    let entity = EntityRef::from_raw(entity.cast());

    entity.networkable.pre_data_update(DataUpdateKind::Created);
    entity.networkable.on_pre_data_changed(DataUpdateKind::Created);

    *entity.mins() = Vec3::splat(-32767.0);
    *entity.maxs() = Vec3::splat(32767.0);

    entity.networkable.on_data_changed(DataUpdateKind::Created);
    entity.networkable.post_data_update(DataUpdateKind::Created);
}*/

/// Iterate entities and update entity specific things.
#[inline]
unsafe fn update_entities(entity_list: &EntityList) {
    let state = State::get();
    let globals = state.globals.as_ref().unwrap();
    let time = globals.current_time;
    let local_vars = &state.local;

    let player_iter = entity_list
        .player_range()
        .flat_map(|index| Some((index, PlayerRef::from_raw(entity_list.entity(index))?)));

    for (index, player) in player_iter {
        let local = PlayerRef::from_raw(local_vars.player).unwrap();

        if local.index() == index {
            continue;
        }

        if player.is_dormant() {
            continue;
        }

        if !player.is_alive() {
            continue;
        }

        if !player.is_enemy() {
            continue;
        }

        const BONE_USED_BY_HITBOX: i32 = 0x00000100;

        player.setup_bones(
            &mut state.bones[(index as usize) - 1][..128],
            BONE_USED_BY_HITBOX,
            time,
        );
    }

    let entity_iter = entity_list
        .non_player_range()
        .flat_map(|index| EntityRef::from_raw(entity_list.entity(index)));

    for entity in entity_iter {
        let class = match entity.client_class() {
            Some(class) => class,
            None => continue,
        };

        match class.entity_id {
            EntityId::CEnvTonemapController => update_tonemap(entity.cast_tonemap()),
            EntityId::CFogController => update_fog(entity.cast_fog()),
            EntityId::CPrecipitation => (), //println!("got rain"),
            _ => {}
        }
    }
}

const FRAME_NET_UPDATE_END: ffi::c_int = 4;
const FRAME_RENDER_START: ffi::c_int = 5;

#[derive(Clone, Copy)]
pub struct FrameStageNotify(pub(crate) unsafe extern "C" fn(*const u8, ffi::c_int));

/// `FrameStageNotify` hook.
pub unsafe extern "C" fn frame_stage_notify(this: *const u8, frame: ffi::c_int) {
    debug_assert!(!this.is_null());

    let frame_stage_notify = elysium_sdk::with_app_mut(|app| {
        match frame {
            FRAME_RENDER_START => app.update(),
            _ => {}
        }

        *app.world.resource::<FrameStageNotify>()
    });

    let state = State::get();
    let Interfaces {
        engine,
        entity_list,
        ..
    } = state.interfaces.as_ref().unwrap();

    if engine.is_in_game() && !state.new_game {
        state.new_game = false;
        state.update_materials = true;
    } else {
        state.new_game = true;
        state.world.as_mut().unwrap().clear();
        state.blur.as_mut().unwrap().clear();
    }

    for material in state.world.as_ref().unwrap().iter() {
        let material = material.get();

        material.set_rgba([0.2, 0.2, 0.2, 1.0]);
    }

    for material in state.blur_static.as_ref().unwrap().iter() {
        let material = material.get();

        material.set_flag(material::Flag::NO_DRAW, true);
    }

    for material in state.blur.as_ref().unwrap().iter() {
        let material = material.get();

        material.set_flag(material::Flag::NO_DRAW, true);
    }

    if let Some(material) = state::material::BLOOD.load() {
        material.set_flag(material::Flag::WIREFRAME, true);
        material.set_rgba([1.0, 0.0, 0.0, 1.0]);
    }

    if let Some(material) = state::material::MUZZLE_FLASH.load() {
        material.set_flag(material::Flag::WIREFRAME, true);
        material.set_rgba([1.0, 1.0, 1.0, 1.0]);
    }

    if let Some(material) = state::material::DECAL.load() {
        material.set_flag(material::Flag::NO_DRAW, true);
    }

    if let Some(material) = state::material::SMOKE.load() {
        material.set_flag(material::Flag::WIREFRAME, true);
        material.set_rgba([1.0, 0.0, 1.0, 1.0]);
    }

    if let Some(material) = state::material::FIRE.load() {
        material.set_flag(material::Flag::WIREFRAME, true);
        material.set_rgba([1.0, 1.0, 0.0, 1.0]);
    }

    let globals = state.globals.as_mut().unwrap();
    let input = state.input.as_mut().unwrap();
    let vars = state.vars.as_mut().unwrap();
    let local_vars = &mut state.local;
    let frame = match Frame::from_raw(frame) {
        Some(frame) => frame,
        None => panic!("unexpected frame variant: {frame:?}"),
    };

    state.view_angle = engine.view_angle();

    // force vars
    update_vars(vars, engine);

    local_vars.player = entity_list.local_player(engine).cast();

    if local_vars.player.is_null() {
        local_vars.reset();
    } else {
        let mut local_player = PlayerRef::from_raw(local_vars.player).unwrap();

        state.location = Some(local_player.location_name());

        // is it even enabled
        let mut thirdperson = local_vars.thirdperson.enabled;

        // certain spectator modes alongside thirdperson break the camera.
        thirdperson &= !local_player.observer_mode().breaks_thirdperson();

        // use first person while scoped.
        thirdperson &= !local_player.is_scoped();

        // apply current toggle state.
        thirdperson &= local_vars.thirdperson.toggle;

        input.thirdperson = thirdperson;

        match frame {
            Frame::RenderStart => {
                update_thirdperson(globals, input, local_vars, &mut local_player);
                update_entities(entity_list);
            }
            _ => {
                local_player.set_view_angle(state.original_view_angle);
            }
        }
    }

    (frame_stage_notify.0)(this, frame.to_i32());
}
