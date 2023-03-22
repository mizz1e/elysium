use crate::{global, math, trace, Config, IClientEntity, IEngineTrace, IVEngineClient, Ptr};
use bevy::prelude::*;
use std::ffi;

bitflags::bitflags! {
    /// Movement state flags.
    ///
    /// [`game/shared/in_buttons.h`](https://github.com/alliedmodders/hl2sdk/blob/csgo/game/shared/in_buttons.h).
    #[repr(transparent)]
    pub struct Button: i32 {
        /// Attack with the current weapon.
        ///
        /// You cannot attack if you do not have any weapons.
        const ATTACK = 1 << 0;

        /// Jump.
        ///
        /// You can only jump if you are on the ground.
        const JUMP = 1 << 1;

        /// Duck (Go into a crouching position).
        const DUCK = 1 << 2;

        /// Move forward.
        ///
        /// Used for animations.
        const MOVE_FORWARD = 1 << 3;

        /// Move backward.
        ///
        /// Used for animations.
        const MOVE_BACKWARD = 1 << 4;

        /// Interact with something.
        ///
        /// Plant a bomb, defuse a bomb, open a door, so on.
        const USE = 1 << 5;

        /// TODO.
        const CANCEL = 1 << 6;

        /// TODO.
        const LEFT = 1 << 7;

        /// TODO.
        const RIGHT = 1 << 8;

        /// Move to the left.
        ///
        /// Used for animations.
        const MOVE_LEFT = 1 << 9;

        /// Move to the right.
        ///
        /// Used for animations.
        const MOVE_RIGHT = 1 << 10;

        /// Secondary attack with the current weapon.
        ///
        /// Switch firing mode, quick fire a revolver, etc.
        ///
        /// You cannot attack if you do not have any weapons.
        const ATTACK_SECONDARY = 1 << 11;

        /// TODO.
        const RUN = 1 << 12;

        /// Reload the current weapon.
        ///
        /// You cannot reload if you do not have any weapons.
        const RELOAD = 1 << 13;

        /// TODO.
        const ALT = 1 << 14;

        /// TODO.
        const ALT_SECONDARY = 1 << 15;

        /// TODO.
        const SCOREBOARD = 1 << 16;

        /// TODO.
        const SPEED = 1 << 17;

        /// TODO.
        const WALK = 1 << 18;

        /// TODO.
        const ZOOM = 1 << 19;

        /// TODO.
        const WEAPON = 1 << 20;

        /// TODO.
        const WEAPON_SECONDARY = 1 << 21;

        /// Enables fast duck.
        ///
        /// Must be enabled for the duration of ducking normally.
        ///
        /// See [`CCSGameMovement::CheckParameters` in `game/shared/cstrike15/cs_gamemovement.cpp`](https://github.com/elysian6969/cstrike/blob/master/game/shared/cstrike15/cs_gamemovement.cpp#L169) for why this works.
        const FAST_DUCK = 1 << 22;

        /// TODO.
        const GRENADE = 1 << 23;

        /// TODO.
        const GRENADE_SECONDARY = 1 << 24;

        /// TODO.
        const LOOK_SPIN = 1 << 25;
    }
}

#[derive(Clone, Copy, Resource)]
#[repr(C)]
pub struct CUserCmd {
    vtable: *const (),
    pub number: ffi::c_int,
    pub tick_count: ffi::c_int,
    pub view_angle: Vec3,
    pub aim_direction: Vec3,
    pub movement: Vec3,
    pub buttons: Button,
    pub impulse: u8,
    pub weapon_select: ffi::c_int,
    pub weapon_subtype: ffi::c_int,
    pub random_seed: ffi::c_int,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Vec3,
    pub head_offset: Vec3,
}

unsafe impl Send for CUserCmd {}
unsafe impl Sync for CUserCmd {}

#[derive(Resource)]
pub struct CamThink(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct CamToFirstPerson(pub(crate) unsafe extern "C" fn(this: *mut u8));

#[derive(Resource)]
pub struct CamToThirdPerson(pub(crate) unsafe extern "C" fn(this: *mut u8));

/// See `game/client/iinput.h`.
#[derive(Resource)]
pub struct CInput {
    pub(crate) ptr: Ptr,
}

impl CInput {
    pub(crate) unsafe fn setup(&self) {
        tracing::trace!("setup input");

        global::with_app_mut(|app| {
            debug!("IInput->CAM_Think");
            app.insert_resource(CamThink(self.ptr.vtable_replace(31, cam_think)));

            tracing::trace!("cam to third person");
            app.insert_resource(CamToThirdPerson(
                self.ptr.vtable_replace(35, cam_to_third_person),
            ));

            tracing::trace!("cam to first person");
            app.insert_resource(CamToFirstPerson(
                self.ptr.vtable_replace(36, cam_to_first_person),
            ));
        });
    }

    unsafe fn internal(&self) -> &mut internal::CInput {
        &mut *self.ptr.as_ptr().cast::<internal::CInput>()
    }

    pub fn in_thirdperson(&self) -> bool {
        unsafe { self.internal().in_thirdperson }
    }

    pub fn set_in_thirdperson(&self, enabled: bool) {
        unsafe {
            self.internal().in_thirdperson = enabled;
        }
    }

    pub fn camera_offset(&self) -> Vec3 {
        unsafe { self.internal().camera_offset }
    }

    pub fn set_camera_offset(&self, offset: Vec3) {
        unsafe {
            self.internal().camera_offset = offset;
        }
    }

    fn to_firstperson(&self) {
        let method = global::with_resource::<CamToFirstPerson, _>(|method| method.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    fn to_thirdperson(&self) {
        let method = global::with_resource::<CamToThirdPerson, _>(|method| method.0);

        unsafe { (method)(self.ptr.as_ptr()) }
    }
}

// xref: "CAM_Think" @ client_client.so
unsafe extern "C" fn cam_think(this: *mut u8) {
    debug_assert!(!this.is_null());

    let method = global::with_resource::<CamThink, _>(|method| method.0);

    global::with_app(|app| {
        let config = app.world.resource::<Config>();
        let engine = app.world.resource::<IVEngineClient>();
        let input = app.world.resource::<CInput>();
        let trace = app.world.resource::<IEngineTrace>();

        let mut in_thirdperson = config.thirdperson_enabled & config.in_thirdperson;

        in_thirdperson &= IClientEntity::local_player()
            .map(|local_player| {
                !(local_player.observer_mode().breaks_thirdperson() | local_player.is_scoped())
            })
            .unwrap_or_default();

        // Only update if a difference occured.
        if in_thirdperson != input.in_thirdperson() {
            // Full control here as the CAM_To* methods are hooked, and do nothing.
            if in_thirdperson {
                // Camera view angle isn't updated if thirdperson is already enabled.
                //
                // See `CInput::CAM_ToThirdPerson` in `game/client/in_camera.cpp`.
                input.set_in_thirdperson(false);
                input.to_thirdperson();
            } else {
                // Doesn't properly update if already in firstperson.
                //
                // See `CInput::CAM_ToThirdPerson` in `game/client/in_camera.cpp`.
                input.set_in_thirdperson(true);
                input.to_firstperson();
            }
        }

        if in_thirdperson {
            let engine_view_angle = engine.view_angle();
            let mut camera_offset = engine_view_angle.truncate().extend(120.0);

            if let Some(local_player) = IClientEntity::local_player() {
                // See `CInput::CAM_Think`.
                let eye_origin = local_player.origin() + Vec3::new(0.0, 0.0, 64.0);
                let (forward, _right, _up) = math::to_vectors(camera_offset);
                let result = trace.filtered_trace(
                    eye_origin,
                    eye_origin - (forward * camera_offset.z),
                    trace::MASK_SOLID,
                    Some(&local_player),
                );

                camera_offset.z *= result.fraction.clamp(0.0, 1.0);
            }

            input.set_camera_offset(camera_offset);
        }
    });

    (method)(this)
}

unsafe extern "C" fn cam_to_first_person(this: *mut u8) {
    debug_assert!(!this.is_null());

    return;
}

unsafe extern "C" fn cam_to_third_person(this: *mut u8) {
    debug_assert!(!this.is_null());

    return;
}

mod internal {
    use bevy::math::Vec3;
    use std::mem::MaybeUninit;

    #[repr(C)]
    pub struct CInput {
        _pad0: MaybeUninit<[u8; 18]>,
        pub mouse_active: bool,
        _pad1: MaybeUninit<[u8; 162]>,
        pub in_thirdperson: bool,
        pub is_camera_moving_with_mouse: bool,
        pub camera_offset: Vec3,
    }
}
