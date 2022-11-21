use crate::{
    global, math, Button, CUserCmd, CViewSetup, Config, EntityFlag, IClientEntity,
    IClientEntityList, IEngineTrace, IVEngineClient, Mat4x3, Ptr, Time,
};
use bevy::ecs::system::SystemState;
use bevy::prelude::{Res, ResMut, Resource, Vec3};
use std::arch::asm;
use std::ptr;

#[derive(Resource)]
pub struct OverrideView(pub(crate) unsafe extern "C" fn(this: *mut u8, setup: *mut CViewSetup));

// rustfmt breaks with "trailing space left behind" after `pub(crate)`
type CreateMoveFn =
    unsafe extern "C" fn(this: *mut u8, input_sample_time: f32, command: *mut CUserCmd) -> bool;

#[derive(Resource)]
pub struct CreateMove(pub(crate) CreateMoveFn);

/// `game/client/iclientmode.h`.
#[derive(Resource)]
pub struct IClientMode {
    pub(crate) ptr: Ptr,
}

impl IClientMode {
    pub(crate) unsafe fn setup(&self) {
        tracing::trace!("setup IClientMode");

        global::with_app_mut(|app| {
            app.insert_resource(OverrideView(self.ptr.vtable_replace(19, override_view)));

            // rustc apparently is a little too overzealous with it's optimization, and
            // deletes this hook if the result is unused?
            app.insert_resource(CreateMove(self.ptr.vtable_replace(25, create_move)));
        });
    }
}

unsafe extern "C" fn override_view(this: *mut u8, setup: *mut CViewSetup) {
    debug_assert!(!this.is_null());
    debug_assert!(!setup.is_null());

    let setup = &mut *setup;

    let method = global::with_app(|app| {
        let engine = app.world.resource::<IVEngineClient>();

        setup.view_angle = engine.view_angle();

        app.world.resource::<OverrideView>().0
    });

    (method)(this, setup)
}

#[derive(Resource)]
pub struct LastYaw(pub f32);

unsafe extern "C" fn create_move(
    this: *mut u8,
    input_sample_time: f32,
    command: *mut CUserCmd,
) -> bool {
    let rbp: *mut *mut bool;

    asm!("mov {}, rbp", out(reg) rbp, options(nostack));

    let send_packet = &mut *(*rbp).sub(24);

    debug_assert!(!this.is_null());
    debug_assert!(!command.is_null());

    let command = &mut *command;
    let method = global::with_resource::<CreateMove, _>(|method| method.0);

    (method)(this, input_sample_time, command);

    // ignore input sampling
    if command.number == 0 {
        return false;
    }

    global::with_app_mut(|app| {
        if !app.world.contains_resource::<LastYaw>() {
            app.insert_resource::<LastYaw>(LastYaw(0.0));
        }

        let mut system_state: SystemState<(
            Res<Config>,
            Res<IVEngineClient>,
            Res<IClientEntityList>,
            Res<IEngineTrace>,
            ResMut<LastYaw>,
        )> = SystemState::new(&mut app.world);

        let (config, engine, entity_list, trace, mut last_yaw) =
            system_state.get_mut(&mut app.world);
        let engine_view_angle = engine.view_angle();
        let local_player = IClientEntity::local_player().unwrap();
        let local_flags = local_player.flags();
        let last_yaw = &mut last_yaw.0;
        let flip = command.tick_count % 2 == 0;
        let side = if flip { 1.0 } else { -1.0 };

        if local_flags.contains(EntityFlag::IN_AIR) {
            let velocity = local_player.velocity();
            let hypot = velocity.truncate().length();
            let ideal_strafe = (15.0 / hypot).atan().to_degrees().clamp(0.0, 90.0);
            let mut wish_angle = command.view_angle;
            let strafe_dir = command.movement.truncate();
            let strafe_dir_yaw_offset = strafe_dir.y.atan2(strafe_dir.x).to_degrees();

            wish_angle.y -= strafe_dir_yaw_offset;

            let mut wish_angle = math::sanitize_angle(wish_angle);
            let yaw_delta = math::normalize_component(wish_angle.y - *last_yaw);
            let abs_yaw_delta = yaw_delta.abs();

            *last_yaw = wish_angle.y;

            let horizontal_speed = 450.0; //vars.horizontal_speed.read();

            if abs_yaw_delta <= ideal_strafe || abs_yaw_delta >= 30.0 {
                let velocity_dir = math::to_angle(velocity);
                let velocity_yaw_delta = math::normalize_component(wish_angle.y - velocity_dir.y);
                let retrack = (30.0 / hypot).atan().to_degrees().clamp(0.0, 90.0) * 2.0;

                if velocity_yaw_delta <= retrack || hypot <= 15.0 {
                    if -retrack <= velocity_yaw_delta || hypot <= 15.0 {
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
            command.movement = math::fix_movement(command.movement, command.view_angle, wish_angle);
        }

        if command.buttons.contains(Button::JUMP) {
            let remove = if local_flags.contains(EntityFlag::IN_AIR) {
                Button::JUMP
            } else {
                Button::DUCK
            };

            command.buttons.remove(remove);
        }

        config.pitch.apply(&mut command.view_angle.x);
        command.view_angle.y += config.yaw_offset;
        command.view_angle.z = config.roll;

        if config.desync_enabled {
            let max_desync_angle = local_player.max_desync_angle();
            let is_lby_updating = local_player.is_lby_updating();

            *send_packet = flip;

            if is_lby_updating {
                *send_packet = false;
            } else if !*send_packet {
                command.view_angle.y += max_desync_angle * 2.0;
            }

            if command.movement.y.abs() < 5.0 {
                let amount = if command.buttons.contains(Button::DUCK) {
                    3.25
                } else {
                    1.1
                };

                command.movement.y = amount * side;
            }
        }

        let now = Time::now();
        let eye_pos = local_player.eye_pos();

        const BONE_USED_BY_HITBOX: i32 = 0x100;
        const CONTENTS_HITBOX: u32 = 0x40000000;
        const CONTENTS_SOLID: u32 = 0x1;

        for i in 1..=64 {
            let Some(player) = entity_list.get(i) else {
                    continue;
                };

            let flags = player.flags();

            if !flags.contains(EntityFlag::ENEMY) {
                continue;
            }

            let mut bones = [Mat4x3::ZERO; 256];

            player.setup_bones(&mut bones, BONE_USED_BY_HITBOX, now);

            let head_bone = bones[8];
            let head_origin: Vec3 = head_bone.to_affine().translation.into();

            command.view_angle = math::calculate_angle(eye_pos, head_origin);

            let direction = command.view_angle.normalize_or_zero() * 4096.0;

            let result = trace.trace(
                eye_pos,
                eye_pos + direction,
                CONTENTS_SOLID | CONTENTS_HITBOX,
            );

            if let Some(entity) = result.entity_hit {
                command.buttons.insert(Button::ATTACK);

                break;
            }
        }

        if let Some(weapon) = local_player.active_weapon() {
            let next_primary_attack = weapon.next_primary_attack().0;
            let server_time = local_player.tick_base().to_time().0;

            if server_time < next_primary_attack {
                command.buttons.remove(Button::ATTACK);
            }
        }

        // https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/in_main.cpp#L1166
        if command.weapon_select != 0 {
            command.buttons.remove(Button::ATTACK);
        }

        command.view_angle -= local_player.aim_punch();
        command.view_angle = math::sanitize_angle(command.view_angle);
        command.movement =
            math::fix_movement(command.movement, command.view_angle, engine_view_angle);

        config.walking_animation.apply(command);

        if *send_packet {
            app.insert_resource(ptr::read(command));
        }

        false
    })
}
