use bevy::prelude::*;

use bevy_knight::level::plugin::LevelPlugin;
use bevy_knight::load::plugin::LoadPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadPlugin)
        .add_plugin(LevelPlugin)
        .add_startup_system(camera.system())
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(OrthographicCameraBundle::new_2d());
}
