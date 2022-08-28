#[derive(Debug)]
#[repr(C)]
pub struct Globals {
    pub real_time: f32,
    pub frame_count: i32,
    pub absolute_frame_time: f32,
    pub absolute_frame_start_time_standard: f32,
    pub current_time: f32,
    pub frame_time: f32,
    pub max_clients: i32,
    pub tick_count: i32,
    pub tick_interval: f32,
    pub interpolation_amount: f32,
    pub simulation_ticks_this_frame: i32,
    pub network_protocol: i32,
    pub save_data: *const (),
    pub is_client: bool,
    pub is_remote_client: bool,
}

impl Globals {
    #[inline]
    pub fn time_to_ticks(&self, time: i32) -> i32 {
        ((time as f32 + 0.5) / self.tick_interval) as i32
    }

    #[inline]
    pub fn ticks_to_time(&self, ticks: i32) -> f32 {
        self.tick_interval * ticks as f32
    }

    #[inline]
    pub fn round_to_ticks(&self, time: i32) -> f32 {
        self.time_to_ticks(time) as f32 * self.tick_interval
    }
}
