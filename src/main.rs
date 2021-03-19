use bevy::prelude::*;

use bevy_knight::camera::plugin::CameraPlugin;
use bevy_knight::knight::plugin::KnightPlugin;
use bevy_knight::level::plugin::LevelPlugin;
use bevy_knight::load::plugin::LoadSpritesPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadSpritesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(KnightPlugin)
        .run();
}
