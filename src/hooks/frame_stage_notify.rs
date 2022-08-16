use crate::state::Local;
use crate::{Entity, EntityRef, State};
use elysium_math::Vec3;
use elysium_sdk::client::Class;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::EntityId;
use elysium_sdk::{Engine, EntityList, Frame, Globals, Input, Interfaces};

fn update_vars(vars: &Vars, engine: &Engine) {
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
}

/// Override fog controller properties.
fn update_fog(entity: EntityRef) {
    *entity.is_enabled() = true;
    *entity.start_distance() = 150.0;
    *entity.end_distance() = 350.0;
    *entity.far_z() = 10000.0;
    *entity.density() = 0.1;
    *entity.color_primary() = 0x0000FF;
    *entity.color_secondary() = 0xFFFF00;
    *entity.direction() = Vec3::from_xyz(1.0, 0.0, 0.0);
}

/// Override tonemap controller properties.
fn update_tonemap(entity: EntityRef) {
    *entity.enable_bloom_scale() = true;
    *entity.enable_min_exposure() = true;
    *entity.enable_max_exposure() = true;
    *entity.min_exposure() = 0.5;
    *entity.max_exposure() = 0.5;
    *entity.bloom_scale() = 2.0;
}

/// Thirdperson handling.
fn update_thirdperson(globals: &Globals, input: &Input, local_vars: &mut Local, local: &Entity) {
    let state = State::get();

    if input.thirdperson {
        // fix the local player's view_angle when in thirdperson
        *local.view_angle() = local_vars.view_angle;
        // other players can't see roll, so why should we?
        local.view_angle().z = 0.0;
    } else {
        // in cooperation with override_view, this will change the view model's position.
        if local_vars.visualize_shot != 0.0 {
            if local_vars.visualize_shot > globals.current_time {
                *local.view_angle() = local_vars.shot_view_angle;
            } else {
                *local.view_angle() = state.view_angle;
                local_vars.visualize_shot = 0.0;
            }
        }

        // rotate view model
        local.view_angle().z = -35.0;
    }
}

/// Iterate entities and update entity specific things.
#[inline]
unsafe fn update_entities(entity_list: &EntityList) {
    let state = State::get();
    let players = &mut state.players;
    let globals = state.globals.as_ref().unwrap();
    let time = globals.current_time;

    for index in entity_list.player_range() {
        let entity = entity_list.entity(index);

        if entity.is_null() {
            continue;
        }

        let entity = EntityRef::from_raw(entity.cast::<Entity>());
        let mut bones = players[index as usize - 1].bones;

        entity.setup_bones(&mut bones[..128], 0x00000100, time);
        entity.setup_bones(&mut bones[..128], 0x000FFF00, time);
    }

    for index in entity_list.non_player_range() {
        let entity = entity_list.entity(index);

        if entity.is_null() {
            continue;
        }

        let entity = EntityRef::from_raw(entity.cast::<Entity>());
        let class = entity.client_class();

        if class.is_null() {
            continue;
        }

        let class = &*class.cast::<Class>();

        match class.entity_id {
            EntityId::CFogController => update_fog(entity),
            EntityId::CEnvTonemapController => update_tonemap(entity),
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
    let vars = state.vars.as_ref().unwrap();
    let local_vars = &mut state.local;
    let is_menu_open = state.menu_open.0;
    let frame = Frame::from_raw_unchecked(frame);

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
        let local = &*local_vars.player;

        input.thirdperson = !local.observer_mode().breaks_thirdperson() && local_vars.thirdperson.0;

        match frame {
            Frame::RenderStart => {
                update_thirdperson(globals, input, local_vars, local);
                update_entities(entity_list);
            }
            _ => {}
        }
    }

    (frame_stage_notify_original)(this, frame.into_raw());
}
