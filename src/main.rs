use bevy::prelude::*;
use bevy_log::LogPlugin;
use elysium_sdk::{OnceLoaded, Renderer, SourcePlugin, SourceSettings};

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
    .run();
}
