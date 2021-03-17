use bevy::prelude::*;

use bevy_knight::camera::plugin::CameraPlugin;
use bevy_knight::level::plugin::LevelPlugin;
use bevy_knight::load::plugin::LoadSpritesPlugin;
use bevy_knight::player::plugin::PlayerPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(LoadSpritesPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
