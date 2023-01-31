use crate::{
    global, math, trace, Button, CUserCmd, CViewSetup, ClientState, Config, EntityFlag,
    IClientEntity, IClientEntityList, IEngineTrace, IPhysicsSurfaceProps, IVEngineClient, Mat4x3,
    Ptr, SurfaceKind, Time, TraceResult, WeaponInfo,
};
use bevy::ecs::system::SystemState;
use bevy::prelude::{Res, ResMut, Resource, Vec3};
use rand::Rng;
use std::arch::asm;
use std::cmp::Ordering;
use std::ptr;

#[derive(Resource)]
pub struct OverrideView(pub(crate) unsafe extern "C" fn(this: *mut u8, setup: *mut CViewSetup));

// rustfmt breaks with "trailing space left behind" after `pub(crate)`
type CreateMoveFn =
    unsafe extern "C" fn(this: *mut u8, input_sample_time: f32, command: *mut CUserCmd) -> bool;

#[derive(Resource)]
pub struct CreateMove(pub(crate) CreateMoveFn);

#[derive(Resource)]
pub struct LastYaw(pub f32);

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

            app.insert_resource::<LastYaw>(LastYaw(0.0));
        });
    }
}

unsafe extern "C" fn override_view(this: *mut u8, setup: *mut CViewSetup) {
    debug_assert!(!this.is_null());
    debug_assert!(!setup.is_null());

    let setup = &mut *setup;

    let method = global::with_app(|app| {
        let engine = app.world.resource::<IVEngineClient>();

        let is_scoped = IClientEntity::local_player()
            .map(|local_player| local_player.is_scoped())
            .unwrap_or_default();

        setup.fov = if is_scoped { 70.0 } else { 110.0 };
        setup.view_angle = engine.view_angle();
        setup.view_angle.z = 0.0;

        app.world.resource::<OverrideView>().0
    });

    (method)(this, setup)
}

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
        let movement = command.movement;
        let now = Time::now();
        let eye_origin = local_player.eye_origin();

        if local_flags.contains(EntityFlag::IN_AIR) {
            command.buttons.remove(Button::JUMP);

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

        let entities = entity_list.players();
        let mut entities = entities
            .iter()
            .filter(|entity| entity.flags().contains(EntityFlag::ENEMY))
            .map(|entity| {
                const BONE_USED_BY_HITBOX: i32 = 0x100;

                let mut bones = [Mat4x3::ZERO; 256];

                entity.setup_bones(&mut bones, BONE_USED_BY_HITBOX, now);

                let head_bone = bones[8];
                let head_origin = head_bone.to_affine().translation.into();
                let view_angle = math::calculate_angle(eye_origin, head_origin);
                let distance = engine_view_angle.distance(view_angle);

                (entity, view_angle, distance)
            })
            .collect::<Vec<_>>();

        // sort by distance
        entities.sort_unstable_by(|a, b| a.2.total_cmp(&b.2));

        let target_entity = entities.first();

        if let Some((_enemy, view_angle, _distance)) = target_entity {
            command.view_angle = *view_angle;
        } else {
            command.view_angle.y += 37.0
                * match command.tick_count % 4 {
                    0 => 1.0,
                    1 => 1.0,
                    2 => -1.0,
                    3 => -1.0,
                    _ => unreachable!(),
                };
        }

        config.anti_aim.pitch.apply(&mut command.view_angle.x);
        command.view_angle.y += config.anti_aim.yaw_offset;
        command.view_angle.z = config.anti_aim.roll;

        if config.anti_aim.enabled {
            if let Some(client_state) = crate::ClientState::get() {
                let fake_lag = if local_flags.contains(EntityFlag::IN_AIR) {
                    6
                } else {
                    config.fake_lag
                };

                *send_packet = client_state.choked_commands() >= fake_lag;
            }

            if !*send_packet {
                command.view_angle.y += 89.0;
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

        if let Some(weapon) = local_player.active_weapon() {
            let next_primary_attack = weapon.next_primary_attack().0;
            let server_time = local_player.tick_base().to_time().0;

            let weapon_cant_fire = server_time < next_primary_attack;
            let switching_weapons = command.weapon_select != 0;
            let no_ammo = weapon.remaining_ammo() == 0;

            if weapon_cant_fire || switching_weapons || no_ammo {
                command.buttons.remove(Button::ATTACK);
            } else if command.buttons.contains(Button::ATTACK) {
                if let Some((_enemy, view_angle, _distance)) = target_entity {
                    command.view_angle = *view_angle;
                }
            }
        }

        command.view_angle -= local_player.aim_punch();
        command.view_angle = math::sanitize_angle(command.view_angle);
        command.movement =
            math::fix_movement(command.movement, command.view_angle, engine_view_angle);

        config.walking_animation.apply(command);

        if *send_packet || command.buttons.contains(Button::ATTACK) {
            app.insert_resource(ptr::read(command));
        }

        false
    })
}

#[derive(Debug)]
pub struct ShotData {
    pub current_damage: f32,
    pub direction: Vec3,
    //pub damage_modifier: f32,
    pub end: Vec3,
    pub penetration_modifier: f32,
    pub penetrations: u8,
    pub range: f32,
    pub range_modifier: f32,
    pub result: Option<TraceResult>,
    pub start: Vec3,
    pub trace_length: f32,
    pub trace_length_remaining: f32,
}

pub fn simulate_shot(eye_origin: Vec3, direction: Vec3, info: WeaponInfo) -> ShotData {
    let mut data = ShotData {
        current_damage: info.damage,
        //damage_modifier: info.damage_modifier,
        direction,
        end: Vec3::ZERO,
        penetration_modifier: info.penetration_modifier,
        penetrations: 4,
        range: info.range,
        range_modifier: info.range_modifier,
        result: None,
        start: eye_origin,
        trace_length: 0.0,
        trace_length_remaining: 0.0,
    };

    data.simulate();
    data
}

impl ShotData {
    fn simulate(&mut self) {
        global::with_resource::<IEngineTrace, _>(|trace| {
            while self.penetrations > 0 && self.current_damage >= 1.0 {
                self.trace_length_remaining = self.range - self.trace_length;
                self.end = self.start + self.direction * self.trace_length_remaining;

                let new_end = self.end + self.direction * 40.0;
                let result = trace.filtered_trace(
                    self.start,
                    self.start + self.direction,
                    trace::MASK_SHOT,
                    &IClientEntity::local_player(),
                );

                self.result = Some(result);

                if let Some(entity) = result.entity_hit {
                    self.trace_length = result.fraction * self.trace_length_remaining;
                    self.current_damage = self.range_modifier.powf(self.trace_length * 0.002);

                    break;
                }

                if !self.handle_bullet_penetration() {
                    break;
                }
            }
        })
    }

    fn handle_bullet_penetration(&mut self) -> bool {
        let enter_result = self.result.unwrap();
        //tracing::trace!("{enter_result:?}");
        return false;
        let enter_surface = global::with_resource::<IPhysicsSurfaceProps, _>(|surface_props| {
            surface_props
                .data(enter_result.surface.surface_props as i32)
                .unwrap()
        });

        self.trace_length = enter_result.fraction * self.trace_length_remaining;
        self.current_damage = self.range_modifier.powf(self.trace_length * 0.002);

        if self.trace_length > self.range || enter_surface.penetration_modifier < 0.1 {
            self.penetrations = 0;

            return false;
        }

        if !self.trace_to_exit() {
            return false;
        }

        let exit_result = self.result.unwrap();
        let exit_surface = global::with_resource::<IPhysicsSurfaceProps, _>(|surface_props| {
            surface_props
                .data(exit_result.surface.surface_props as i32)
                .unwrap()
        });

        let is_solid = (enter_result.contents & trace::CONTENTS_SOLID) != 0;
        let is_light = (enter_result.surface.flags & trace::SURF_LIGHT) != 0;

        let (damage_lost, mut penetration_modifier) =
            if matches!(enter_surface.kind, SurfaceKind::Grate | SurfaceKind::Glass) {
                (0.05, 3.0)
            } else if is_solid || is_light {
                (0.16, 1.0)
            } else {
                let penetration_modifier =
                    (enter_surface.penetration_modifier * exit_surface.penetration_modifier) / 2.0;

                (0.16, penetration_modifier)
            };

        if enter_surface.kind == exit_surface.kind {
            penetration_modifier = match exit_surface.kind {
                SurfaceKind::Cardboard | SurfaceKind::Wood => 3.0,
                SurfaceKind::Plastic => 2.0,
                _ => penetration_modifier,
            };
        }

        let modifier = 1.0 / penetration_modifier;
        let distance_squared = exit_result.end.distance_squared(enter_result.end);
        let lost_damage = modifier * 3.0 * (3.0 / self.penetration_modifier * 1.25)
            + self.current_damage
            + distance_squared * modifier / 24.0;

        self.result = Some(enter_result);
        self.current_damage -= lost_damage;

        if self.current_damage < 1.0 {
            return false;
        }

        self.penetrations -= 1;

        true
    }

    fn trace_to_exit(&mut self) -> bool {
        global::with_resource::<IEngineTrace, _>(|trace| {
            let mut distance = 0.0;
            let start = self.result.unwrap().end;

            while distance <= 90.0 {
                distance += trace::TO_EXIT_STEP;

                self.end = start + self.direction * distance;

                let contents = trace.contents(self.end, trace::MASK_TO_EXIT);

                if (contents & trace::MASK_TO_EXIT) != 0 {
                    continue;
                }

                let new_end = self.end - self.direction * trace::TO_EXIT_STEP;
                let result = trace.trace(self.end, new_end, trace::MASK_TO_EXIT);

                self.result = Some(result);

                if result.start_solid && (result.surface.flags & trace::SURF_HITBOX) != 0 {
                    let entity_hit = result.entity_hit;
                    let result =
                        trace.filtered_trace(self.end, new_end, trace::MASK_TO_EXIT, &entity_hit);

                    self.result = Some(result);

                    if (result.fraction <= 1.0 || result.plane.is_some()) && !result.start_solid {
                        self.end = result.end;

                        return true;
                    }

                    continue;
                }

                if result.entity_hit.is_some() && !result.start_solid {
                    return true;
                }

                if (result.surface.flags & trace::SURF_NODRAW) != 0 {
                    continue;
                }

                if result.plane.unwrap().normal.dot(self.direction) <= 1.0 {
                    self.end = self.end - self.direction * result.fraction * trace::TO_EXIT_STEP;

                    return true;
                }
            }

            false
        })
    }
}
