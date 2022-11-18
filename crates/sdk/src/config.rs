use crate::WalkingAnimation;
use bevy::prelude::Resource;

#[derive(Debug, Default)]
pub enum Pitch {
    #[default]
    Default,
    Down,
    Up,
}

#[derive(Debug, Default, Resource)]
pub struct Config {
    pub desync_enabled: bool,
    pub pitch: Pitch,
    pub yaw_offset: f32,
    pub walking_animation: WalkingAnimation,
}
