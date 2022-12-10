use crate::{Color, WalkingAnimation};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::{fmt, fs};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Pitch {
    #[default]
    Default,
    Down,
    Up,
}

impl Pitch {
    pub fn apply(self, pitch: &mut f32) {
        match self {
            Pitch::Down => *pitch = 89.0,
            Pitch::Up => *pitch = -89.0,
            _ => {}
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Pitch::Default => "default",
            Pitch::Down => "down",
            Pitch::Up => "up",
        }
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct AntiAim {
    pub enabled: bool,
    pub fake_pitch: Pitch,
    pub fake_roll: f32,
    pub fake_yaw_offset: f32,
    pub pitch: Pitch,
    pub roll: f32,
    pub yaw_offset: f32,
}

#[derive(Debug, Default, Deserialize, Resource, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing)]
    pub active_tab: usize,
    pub anti_aim: AntiAim,
    pub auto_shoot: bool,
    pub fake_lag: i32,
    pub in_thirdperson: bool,
    #[serde(skip_serializing)]
    pub menu_open: bool,
    pub thirdperson_enabled: bool,
    pub walking_animation: WalkingAnimation,
    pub cham_color: Color,
    #[serde(skip_serializing)]
    pub command: String,
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
    let config_file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(config_path)
        .ok()?;

    serde_json::to_writer(config_file, &config).ok()?;

    Some(())
}
