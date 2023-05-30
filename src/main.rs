use {bevy::prelude::*, bevy_source::prelude::*};

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SourcePlugin)
        .run();
}
