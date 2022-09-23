use crate::entity::{Entity, EntityRef, Fog, FogRef, Player, PlayerRef, Tonemap, TonemapRef};
use crate::state::Local;
use crate::State;
use elysium_sdk::entity::EntityId;
use elysium_sdk::{Engine, EntityList, Frame, Globals, Input, Interfaces, Vars};

fn update_vars(vars: &mut Vars, engine: &Engine) {
    // misc
    vars.allow_developer.write(true);
    vars.fast_render.write(!engine.is_in_game());
    vars.cheats.write(true);
    vars.developer.write(true);

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

    // p100
    //vars.hud.write(false);

    // shadows
    //vars.csm.write(false);
    vars.csm_shadows.write(false);
    vars.feet_shadows.write(false);
    vars.prop_shadows.write(false);
    vars.rope_shadows.write(false);
    vars.shadows.write(false);
    vars.skybox3d.write(false);
    vars.viewmodel_shadows.write(false);
    vars.world_shadows.write(false);

    // useless objects
    vars.ropes.write(false);
    vars.sprites.write(false);

    // translucent things
    vars.water_fog.write(false);

    // overlay
    vars.underwater_overlay.write(false);

    // effects
    vars.alien_blood.write(false);
    vars.human_blood.write(false);
    vars.decals.write(false);
    vars.jiggle_bones.write(false);
    //vars.rain.write(false);

    // phsyics
    vars.physics_timescale.write(0.5);

    // meme
    vars.interpolate.write(false);
    vars.lag_comp.write(0.0);
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
    mut local: PlayerRef<'_>,
) {
    let state = State::get();

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

        let mut view_angle = local.view_angle();

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
    let players = &mut state.players;
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

        let mut bones = players[(index as usize) - 1].bones;

        player.setup_bones(&mut bones[..128], 0x00000100, time);
        player.setup_bones(&mut bones[..128], 0x000FFF00, time);
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
            EntityId::CPrecipitation => println!("got rain"),
            _ => {}
        }
    }
}

/// `FrameStageNotify` hook.
pub unsafe extern "C" fn frame_stage_notify(this: *const u8, frame: i32) {
    let state = State::get();
    let Interfaces {
        engine,
        entity_list,
        input_system,
        surface,
        ..
    } = state.interfaces.as_ref().unwrap();

    let frame_stage_notify_original = state.hooks.frame_stage_notify.unwrap();
    let globals = state.globals.as_mut().unwrap();
    let input = state.input.as_mut().unwrap();
    let vars = state.vars.as_mut().unwrap();
    let local_vars = &mut state.local;
    let is_menu_open = state.menu_open.0;
    let frame = match Frame::from_raw(frame) {
        Some(frame) => frame,
        None => panic!("unexpected frame variant: {frame:?}"),
    };

    state.view_angle = engine.view_angle();

    // force vars
    update_vars(vars, engine);

    if engine.is_in_game() {
        input_system.enable_input(is_menu_open);

        if is_menu_open {
            surface.unlock_cursor();
            input.deactivate_mouse();
        } else {
            input.activate_mouse();
        }
    } else {
        // apparently needs to be enabled as you're enterting a map
        input_system.enable_input(true);
    }

    local_vars.player = entity_list.local_player(engine).cast();

    if local_vars.player.is_null() {
        local_vars.reset();
    } else {
        let local_player = PlayerRef::from_raw(local_vars.player).unwrap();

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
                update_thirdperson(globals, input, local_vars, local_player);
                update_entities(entity_list);
            }
            _ => {}
        }
    }

    (frame_stage_notify_original)(this, frame.to_i32());
}
