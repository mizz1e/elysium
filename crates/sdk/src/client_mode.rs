use crate::{
    global, math, Button, CUserCmd, CViewSetup, Config, EntityFlag, IClientEntity,
    IClientEntityList, IVEngineClient, Mat4x3, Ptr, Time,
};
use bevy::ecs::system::SystemState;
use bevy::prelude::{Res, Resource, Vec3};
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
        )> = SystemState::new(&mut app.world);

        let (config, engine, entity_list) = system_state.get(&app.world);
        let engine_view_angle = engine.view_angle();
        let local_player = IClientEntity::local_player().unwrap();

        config.pitch.apply(&mut command.view_angle.x);
        command.view_angle.y += config.yaw_offset;
        command.view_angle.z = config.roll;

        if config.desync_enabled {
            let max_desync_angle = local_player.max_desync_angle();
            let is_lby_updating = local_player.is_lby_updating();
            let flip = command.tick_count % 2 == 0;

            *send_packet = flip;

            if is_lby_updating {
                *send_packet = false;
            } else if !*send_packet {
                command.view_angle.y += max_desync_angle * 2.0;
                //(rand::random::<f32>() + 1.0);
                //command.view_angle.z = (rand::random::<f32>() * 100.0) - 50.0;
            }

            if command.movement.y.abs() < 5.0 {
                let amount = if command.buttons.contains(Button::DUCK) {
                    3.25
                } else {
                    1.1
                };

                command.movement.y = amount * if flip { 1.0 } else { -1.0 };
            }
        }

        if command.buttons.contains(Button::ATTACK)
            || command.buttons.contains(Button::ATTACK_SECONDARY)
        {
            let now = Time::now();
            let eye_pos = local_player.eye_pos();

            for i in 1..=64 {
                let Some(player) = entity_list.get(i) else {
                    continue;
                };

                let flags = player.flags();

                tracing::trace!("{i} {flags:?}");

                if !flags.contains(EntityFlag::ENEMY) {
                    continue;
                }

                let mut bones = [Mat4x3::ZERO; 256];

                player.setup_bones(&mut bones, 0x100 /* BONE_USED_BY_HITBOX */, now);

                let head_bone = bones[8];
                let head_origin: Vec3 = head_bone.to_affine().translation.into();

                command.view_angle = math::calculate_angle(eye_pos, head_origin);
            }
        }

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
