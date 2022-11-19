use crate::WalkingAnimation;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Pitch {
    #[default]
    Default,
    Down,
    Up,
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
pub struct Config {
    pub desync_enabled: bool,
    pub in_thirdperson: bool,
    pub menu_open: bool,
    pub pitch: Pitch,
    pub thirdperson_enabled: bool,
    pub walking_animation: WalkingAnimation,
    pub yaw_offset: f32,
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| "/".into())
        .join("elysium")
}

pub fn load() -> Config {
    try_load().unwrap_or_default()
}

pub fn try_load() -> Option<Config> {
    let config_path = config_dir().join("config.json");
    let config_file = File::open(config_path).ok()?;
    let config = serde_json::from_reader(config_file).ok()?;

    Some(config)
}

pub fn save(config: &Config) {
    try_save(config);
}

pub fn try_save(config: &Config) -> Option<()> {
    let config_dir = config_dir();
    let _ = fs::create_dir_all(&config_dir);
    let config_path = config_dir.join("config.json");
    let mut config_file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(config_path)
        .ok()?;

    serde_json::to_writer(config_file, &config).ok()?;

    Some(())
}
