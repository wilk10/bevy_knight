use bevy::prelude::*;

use bevy_knight::level::plugin::LevelPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LevelPlugin)
        .run();
}
