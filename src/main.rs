//#![deny(warnings)]

use bevy::prelude::*;
//use bevy_source::prelude::*;
use elysium_sdk::{OnceLoaded, SourcePlugin, SourceSettings, WindowMode};

pub use error::Error;
pub use options::Options;

pub mod error;
pub mod options;
pub mod util;

fn main() {
    let options = Options::parse();
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    if let Err(error) = util::pre_launch() {
        error!("{error}");

        return;
    }

    let once_loaded = options
        .map
        .map(OnceLoaded::LoadMap)
        .or_else(|| options.address.map(OnceLoaded::ConnectTo))
        .unwrap_or_default();

    let window_mode = if options.fullscreen {
        WindowMode::Fullscreen
    } else if options.windowed {
        WindowMode::Windowed
    } else {
        WindowMode::Last
    };

    app.insert_resource(SourceSettings {
        max_fps: options.max_fps,
        no_vac: options.no_vac,
        once_loaded,
        window_mode,
        ..default()
    })
    .add_plugin(SourcePlugin)
    .run();
}
