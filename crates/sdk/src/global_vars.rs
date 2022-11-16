use crate::{global, Ptr};
use bevy::prelude::*;
use std::time::Duration;

/// `public/globalvars_base.h`.
#[derive(Resource)]
pub struct CGlobalVarsBase {
    pub(crate) ptr: Ptr,
}

#[derive(Clone, Copy)]
pub struct Time {
    time: Duration,
}

#[derive(Clone, Copy)]
pub struct Tick {
    tick: u32,
}

impl Time {
    /// `TIME_TO_TICKS` in `game/shared/shareddefs.h`.
    #[inline]
    pub fn to_tick(self) -> Tick {
        let tick = ((self.time.as_secs_f32() + 0.5) / interval_per_tick()) as u32;

        Tick { tick }
    }
}

impl Tick {
    /// `TICKS_TO_TIME` in `game/shared/shareddefs.h`.
    #[inline]
    pub fn to_time(self) -> Time {
        let time = Duration::from_secs_f32((self.tick as f32) * interval_per_tick());

        Time { time }
    }
}

#[inline]
fn interval_per_tick() -> f32 {
    global::with_app(|app| {
        let global_vars = app.world.resource::<CGlobalVarsBase>();
        let global_vars = unsafe { &*global_vars.ptr.as_ptr().cast::<internal::CGlobalVarsBase>() };

        global_vars.interval_per_tick
    })
}

mod internal {
    use std::ffi;

    #[repr(C)]
    pub struct CSaveRestoreData;

    #[repr(C)]
    pub struct CGlobalVarsBase {
        pub real_time: f32,
        pub frame_count: ffi::c_int,
        pub absolute_frame_time: f32,
        pub absolute_frame_start_standard_deviation: f32,
        pub current_time: f32,
        pub frame_time: f32,
        pub max_clients: ffi::c_int,
        pub tick_count: ffi::c_int,
        pub interval_per_tick: f32,
        pub interpolation_amount: f32,
        pub simulation_ticks_this_frame: ffi::c_int,
        pub network_protocol: ffi::c_int,
        pub save_data: *const CSaveRestoreData,
        pub is_client: bool,
        pub is_remote_client: bool,
        pub timestamp_networking_base: ffi::c_int,
        pub timestamp_randomize_window: ffi::c_int,
    }
}
