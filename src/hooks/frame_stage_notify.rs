use crate::entity::{Entity, EntityRef, Fog, FogRef, Player, PlayerRef, Tonemap, TonemapRef};
use crate::state::Local;
use crate::State;
use core::mem;
use elysium_sdk::entity::EntityId;
use elysium_sdk::material::{Group, MaterialFlag, MaterialKind};
use elysium_sdk::{Engine, EntityList, Frame, Globals, Input, Interfaces, Vars};

fn update_vars(vars: &mut Vars, engine: &Engine) {
    let state = State::get();

    state.ffa = vars.ffa.read();

    // misc
    vars.allow_developer.write(true);
    //vars.fast_render.write(!engine.is_in_game());
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

    let show_hud = !engine.is_in_game();

    // TODO: do more hud stuff before disabling (e.g. hook console write)
    //vars.hud.write(show_hud);
    //vars.vgui.write(show_hud);

    //vars.other_models.write(2);

    // shadows
    //vars.csm.write(false);
    /*vars.csm_shadows.write(false);
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
    //
    vars.water_fog.write(false);

    // overlay
    vars.underwater_overlay.write(false);

    // effects
    vars.alien_blood.write(false);
    vars.human_blood.write(false);
    vars.decals.write(false);
    vars.jiggle_bones.write(false);
    //vars.rain.write(false);*/

    // phsyics
    vars.physics_timescale.write(0.5);

    // meme
    //vars.interpolate.write(false);
    //vars.lag_comp.write(0.0);
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
            EntityId::CPrecipitation => (), //println!("got rain"),
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
        material_system,
        ..
    } = state.interfaces.as_ref().unwrap();

    let globals = state.globals.as_ref().unwrap();

    if engine.is_in_game() && !state.new_game {
        state.new_game = false;
        state.update_materials = true;
    } else {
        state.new_game = true;
    }

    let _glow = state.materials.get(MaterialKind::Glow, material_system);

    if mem::take(&mut state.update_materials) {
        state.smoke.clear();
        state.particles.clear();

        for material in material_system.iter() {
            let name = material.name();
            let group = material.group();

            match group {
                Group::World => {
                    material.set_rgba([0.7, 0.4, 0.4, 1.0]);

                    continue;
                }
                Group::Skybox => {
                    material.set_rgba([0.7, 0.0, 0.0, 0.7]);

                    continue;
                }
                Group::Other => {
                    if name == "particle/vistasmokev1/vistasmokev1" {
                        state.smoke.push(material);

                        continue;
                    }
                }
                _ => {}
            }

            if name.starts_with("__") || name.starts_with("ui") {
                continue;
            }

            if name.starts_with("models/player") || name.starts_with("models/weapons") {
                state.players_m.push(material);

                continue;
            }

            if name.starts_with("particle") {
                println!("{name:?} {group:?}");
                state.particles.push(material);
            }
        }
    }

    for material in state.smoke.iter() {
        material.set_flag(MaterialFlag::WIREFRAME, true);
    }

    for material in state.players_m.iter() {
        if engine.is_in_game() {
            material.set_rgba([1.0, 1.0, 1.0, 1.0]);
        } else {
            use palette::{Hsl, Hue, IntoColor, Pixel, Srgb};

            let rgb: Hsl = Srgb::new(1.0, 0.0, 0.0).into_color();
            let rgb = rgb.shift_hue(state.init_time.unwrap().elapsed().as_secs_f32() * 100.0);
            let rgb: Srgb = rgb.into_color();
            let [r, g, b]: [f32; 3] = rgb.into_raw();

            material.set_rgba([r, g, b, 1.0]);
        }
    }

    for material in state.particles.iter() {
        material.set_rgba([0.0, 1.0, 1.0, 1.0]);
    }

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
