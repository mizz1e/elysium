use crate::{global, Ptr};
use bevy::prelude::{Deref, DerefMut, Resource};
use std::time::Duration;

/// `public/globalvars_base.h`.
#[derive(Resource)]
pub struct CGlobalVarsBase {
    pub(crate) ptr: Ptr,
}

#[derive(Clone, Copy, Debug, Deref, DerefMut, PartialEq, PartialOrd)]
pub struct Time(pub Duration);

#[derive(Clone, Copy, Debug, Deref, DerefMut, PartialEq, PartialOrd)]
pub struct Tick(pub u32);

impl CGlobalVarsBase {
    unsafe fn internal(&self) -> &internal::CGlobalVarsBase {
        &*self.ptr.as_ptr().cast::<internal::CGlobalVarsBase>()
    }

    fn current_time(&self) -> f32 {
        unsafe { self.internal().current_time }
    }

    pub fn interval_per_tick(&self) -> f32 {
        unsafe { self.internal().interval_per_tick }
    }
}

impl Time {
    pub fn now() -> Self {
        let current_time = global::with_resource::<CGlobalVarsBase, _>(|vars| vars.current_time());

        Time::from_secs_f32(current_time)
    }

    pub fn from_secs_f32(secs: f32) -> Self {
        Time(Duration::from_secs_f32(secs))
    }

    pub fn as_secs_f32(self) -> f32 {
        self.0.as_secs_f32()
    }

    /// `TIME_TO_TICKS` in `game/shared/shareddefs.h`.
    pub fn to_tick(self) -> Tick {
        let interval_per_tick =
            global::with_resource::<CGlobalVarsBase, _>(|vars| vars.interval_per_tick());

        let tick = ((self.as_secs_f32() + 0.5) / interval_per_tick) as u32;

        Tick(tick)
    }
}

impl Tick {
    /// `TICKS_TO_TIME` in `game/shared/shareddefs.h`.
    pub fn to_time(self) -> Time {
        let interval_per_tick =
            global::with_resource::<CGlobalVarsBase, _>(|vars| vars.interval_per_tick());

        Time::from_secs_f32((self.0 as f32) * interval_per_tick)
    }
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
