use bevy::prelude::*;
use bevy_tiled_prototype::{TiledMapCenter, TiledMapComponents, TiledMapPlugin};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TiledMapPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, spawn_tiled_map.system());
    }
}

fn spawn_tiled_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TiledMapComponents {
        map_asset: asset_server.load("map/map.tmx"),
        center: TiledMapCenter(true),
        ..Default::default()
    });
}
