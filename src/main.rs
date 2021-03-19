use bevy::prelude::*;

use bevy_knight::{
    camera::plugin::CameraPlugin,
    knight::plugin::KnightPlugin,
    level::plugin::LevelPlugin,
    load::plugin::LoadSpritesPlugin,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadSpritesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(KnightPlugin)
        .run();
}
