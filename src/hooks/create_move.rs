use crate::entity::{Entity, Player, PlayerRef, Weapon};
use crate::State;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, Renderable};
use elysium_sdk::ClientMode;
use elysium_sdk::{Command, EntityList, HitGroup, Interfaces};
use std::arch::asm;

#[inline]
unsafe fn do_create_move(command: &mut Command, local: PlayerRef<'_>, send_packet: &mut bool) {
    let state = State::get();
    let vars = state.vars.as_ref().unwrap();
    let mut local_vars = &mut state.local;
    let Interfaces { entity_list, .. } = state.interfaces.as_ref().unwrap();
    let globals = state.globals.as_ref().unwrap();
    let players = &mut state.players;

    let do_attack = command.in_attack();
    let do_duck = command.in_duck();
    let do_jump = command.in_jump();
    let on_ground = local.flags().on_ground();
    let was_attacking = local_vars.was_attacking;
    let was_jumping = local_vars.was_jumping;
    let side = if command.command % 2 != 0 { 1.0 } else { -1.0 };

    local_vars.was_attacking = do_attack;
    local_vars.was_jumping = do_jump;

    if do_attack {
        if was_attacking {
            command.attack(false);
            local_vars.was_attacking = false;
        } else {
            local_vars.shift = 8;
        }
    }

    if do_jump {
        if on_ground {
            command.duck(false);
        } else {
            command.jump(false);
        }
    }

    if !on_ground {
        if state.fake_lag != 0 {
            *send_packet = command.command % 14 as i32 == 0;
        }

        // don't do anything fancy whilest on a ladder or noclipping
        if !matches!(local.move_kind(), MoveKind::NoClip | MoveKind::Ladder) {
            let velocity = local.velocity();
            let magnitude = velocity.xy().magnitude();
            let ideal_strafe = (15.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0);
            let mut wish_angle = command.view_angle;
            let strafe_dir = command.movement.xy();
            let strafe_dir_yaw_offset = strafe_dir.y.atan2(strafe_dir.x).to_degrees();

            wish_angle.y -= strafe_dir_yaw_offset;

            let mut wish_angle = wish_angle.sanitize_angle();
            let yaw_delta = libm::remainderf(wish_angle.y - local_vars.old_yaw, 360.0);
            let abs_yaw_delta = yaw_delta.abs();

            local_vars.old_yaw = wish_angle.y;

            let horizontal_speed = vars.horizontal_speed.read();

            if abs_yaw_delta <= ideal_strafe || abs_yaw_delta >= 30.0 {
                let velocity_dir = velocity.to_angle();
                let velocity_yaw_delta = libm::remainderf(wish_angle.y - velocity_dir.y, 360.0);
                let retrack = (30.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0) * 2.0;

                if velocity_yaw_delta <= retrack || magnitude <= 15.0 {
                    if -retrack <= velocity_yaw_delta || magnitude <= 15.0 {
                        wish_angle.y += side * ideal_strafe;
                        command.movement.y = horizontal_speed * side;
                    } else {
                        wish_angle.y = velocity_dir.y - retrack;
                        command.movement.y = horizontal_speed;
                    }
                } else {
                    wish_angle.y = velocity_dir.y + retrack;
                    command.movement.y = -horizontal_speed;
                }
            } else if yaw_delta > 0.0 {
                command.movement.y = -horizontal_speed;
            } else if yaw_delta < 0.0 {
                command.movement.y = horizontal_speed
            }

            command.movement.x = 0.0;
            command.movement = command.movement.movement(command.view_angle, wish_angle);
        }
    }

    let fake_lag = state.fake_lag;

    if fake_lag != 0 {
        let fake_lag = fake_lag + 2;

        *send_packet = command.command % fake_lag as i32 == 0;
    }

    // don't do anything fancy whilest on a ladder or noclipping
    if !matches!(local.move_kind(), MoveKind::NoClip | MoveKind::Ladder) {
        command.view_angle = state
            .anti_aim
            .apply(command.command % 2 == 0, command.view_angle);
    }

    let player_iter = entity_list
        .player_range()
        .flat_map(|index| Some((index, PlayerRef::from_raw(entity_list.entity(index))?)));

    for (index, player) in player_iter {}

    if do_attack {
        command.view_angle = state.view_angle;
    }

    command.fast_duck(true);

    command.movement = command
        .movement
        .movement(command.view_angle, state.view_angle);

    if state.anti_untrusted {
        command.view_angle = command.view_angle.sanitize_angle();
    }
}

/// `CreateMove` hook.
pub unsafe extern "C" fn create_move(
    this: &mut ClientMode,
    sample: f32,
    command: &mut Command,
) -> bool {
    //let return_address = cake::return_address!();
    //let send_packet = &mut *return_address.offset(24);
    let mut send_packet = true;
    let send_packet = &mut send_packet;

    create_move_inner(this, sample, command, send_packet);

    false
}

unsafe fn create_move_inner(
    this: &mut ClientMode,
    sample: f32,
    command: &mut Command,
    send_packet: &mut bool,
) -> Option<()> {
    let state = State::get();
    let create_move_original = state.hooks.create_move?;
    let globals = state.globals.as_ref()?;

    (create_move_original)(this, sample, command);

    if command.tick_count == 0 {
        return None;
    }

    let mut local_player = PlayerRef::from_raw(state.local.player)?;

    // don't mess with input if you are spectating
    if local_player.observer_mode() != ObserverMode::None {
        return None;
    }

    do_create_move(command, local_player, send_packet);

    let mut local_player = PlayerRef::from_raw(state.local.player).unwrap();
    let time = globals.current_time;

    if command.command < state.last_command {
        command.view_angle = Vec3::splat(0.0);
    }

    state.last_command = command.command;

    let fake_lag = state.fake_lag;

    if *send_packet && fake_lag != 0 {
        let mut bones = &mut state.local.fake_bones;

        load_bones(&mut local_player, command, bones, time);
    } else {
        let mut bones = &mut state.local.bones;

        load_bones(&mut local_player, command, bones, time);

        state.local.view_angle = command.view_angle;
        state.local.time = globals.current_time;
    }

    println!("{:?}", crate::state::is_record_valid(globals.current_time));

    None
}

fn load_bones(
    local_player: &mut PlayerRef<'_>,
    command: &Command,
    bones: &mut [Matrix3x4; 256],
    time: f32,
) {
    let view_angle = local_player.view_angle();

    unsafe {
        local_player.set_view_angle(command.view_angle);
    }

    local_player.setup_bones(&mut bones[..128], 0x00000100, time);
    local_player.setup_bones(&mut bones[..128], 0x000FFF00, time);

    unsafe {
        local_player.set_view_angle(view_angle);
    }
}
