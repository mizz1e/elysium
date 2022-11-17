use bevy::prelude::*;
use bevy_log::LogPlugin;
use elysium_sdk::{CUserCmd, OnceLoaded, Renderer, SourcePlugin, SourceSettings};

pub use error::Error;

pub mod error;
pub mod util;

fn main() {
    let mut app = App::new();

    app.add_plugin(LogPlugin::default());

    if let Err(error) = util::pre_launch() {
        tracing::error!("{error}");

        return;
    }

    app.insert_resource(SourceSettings {
        max_fps: Some(144),
        once_loaded: OnceLoaded::LoadMap("de_mirage".into()),
        renderer: Renderer::OpenGl,
    })
    .add_plugin(SourcePlugin)
    .add_system(player_controller)
    .run();
}

fn player_controller(command: Option<ResMut<CUserCmd>>) {
    if let Some(mut command) = command {
        command.view_angle.x = 89.0;
    }
}
