use crate::{state, Entity};
use core::mem;
use core::ptr::NonNull;
use elysium_math::Vec3;
use elysium_sdk::client::Class;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::EntityId;
use elysium_sdk::{Engine, EntityList, Frame, Globals, Input, InputSystem};

fn update_vars(vars: &Vars) {
    // misc
    vars.allow_developer.write(true);
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

fn update_fog(entity: &Entity) {
    *entity.is_enabled() = true;
    *entity.start_distance() = 150.0;
    *entity.end_distance() = 350.0;
    *entity.far_z() = 10000.0;
    *entity.density() = 0.1;
    *entity.color_primary() = 0x0000FF;
    *entity.color_secondary() = 0xFFFF00;
    *entity.direction() = Vec3::from_xyz(1.0, 0.0, 0.0);
}

fn update_tonemap(entity: &Entity) {
    *entity.enable_bloom_scale() = true;
    *entity.enable_min_exposure() = true;
    *entity.enable_max_exposure() = true;
    *entity.min_exposure() = 0.5;
    *entity.max_exposure() = 0.5;
    *entity.bloom_scale() = 3.5;
}

/// `FrameStageNotify` hook.
pub unsafe extern "C" fn frame_stage_notify(this: *const u8, frame: i32) {
    // used interfaces
    let engine = &*state::engine().cast::<Engine>();
    let entity_list = &*state::entity_list().cast::<EntityList>();
    let globals = &*state::globals().cast::<Globals>();
    let input = &mut *state::input().as_mut().cast::<Input>();
    let input_system = &*state::input_system().cast::<InputSystem>();
    let vars = &*state::vars().cast::<Vars>();

    // is menu open !
    let is_menu_open = state::is_menu_open();

    *state::view_angle() = engine.view_angle();

    // TODO: Frame::from_i32(frame);
    let frame: Frame = mem::transmute(frame);
    let index = engine.local_player_index();
    let local = entity_list.entity(index as i32);

    // force vars
    update_vars(&vars);

    if engine.is_in_game() {
        input_system.enable_input(is_menu_open);
        input_system.cursor_visible(is_menu_open);

        if is_menu_open {
            input_system.reset_input_state();
        }
    } else {
        // apparently needs to be enabled as you're enterting a map
        input_system.enable_input(true);
        input_system.cursor_visible(true);
    }

    // TODO: refactor state, probably do like state::local::reset();
    if local.is_null() {
        state::local::set_aim_punch_angle(Vec3::zero());
        state::local::set_player_none();
        state::local::set_view_punch_angle(Vec3::zero());
    } else {
        state::local::set_player(NonNull::new_unchecked(local.as_mut()));

        let local = &*local.cast::<Entity>();

        if local.observer_mode().breaks_thirdperson() {
            input.thirdperson = false;
        } else {
            input.thirdperson = state::local::thirdperson();
        }

        match frame {
            Frame::RenderStart => {
                if input.thirdperson {
                    // fix the local player's view_angle when in thirdperson
                    *local.view_angle() = state::local::view_angle();
                    // other players can't see roll, so why should we?
                    local.view_angle().z = 0.0;
                } else {
                    // in cooperation with override_view, this will change the view model's position.
                    if state::local::use_shot_view_angle() != 0.0 {
                        if state::local::use_shot_view_angle() > globals.current_time {
                            *local.view_angle() = state::local::shot_view_angle();
                        } else {
                            *local.view_angle() = *state::view_angle();
                            state::local::set_use_shot_view_angle(0.0);
                        }
                    }

                    // rotate view model
                    local.view_angle().z = -35.0;
                }

                for index in entity_list.non_player_range() {
                    let entity = entity_list.entity(index);

                    if entity.is_null() {
                        continue;
                    }

                    let entity = &*entity.cast::<Entity>();

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
            _ => {
                if input.thirdperson {
                    // restore to the expected value
                    *local.view_angle() = *state::view_angle();
                }
            }
        }
    }

    state::hooks::frame_stage_notify(this, frame as i32);
}
