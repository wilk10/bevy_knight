use bevy::prelude::*;
use bevy_tiled_prototype::{TiledMapCenter, TiledMapComponents, TiledMapPlugin};

// use crate::load::plugin::TextureHandles;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TiledMapPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, spawn_tiled_map.system());
    }
}

fn spawn_tiled_map(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let top_left_vertex = if let Some(window) = windows.get_primary() {
        Vec3::new(-window.width() / 2.0, window.height() / 2.0, 0.0)
    } else {
        return;
    };

    commands.spawn(TiledMapComponents {
        map_asset: asset_server.load("map/map.tmx"),
        center: TiledMapCenter(false),
        origin: Transform::from_translation(top_left_vertex),
        ..Default::default()
    });
}

// fn build_level(
//     mut commands: Commands,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     texture_handles: ResMut<TextureHandles>,
// ) {
//     texture_handles.atlas.as_ref().map(|atlas_handle| {
//         if let Some(atlas) = texture_atlases.get(atlas_handle.clone()) {
//             let sprite_index = atlas.len() - 1;

//             commands.spawn(SpriteSheetBundle {
//                 transform: Transform {
//                     translation: Vec3::new(0.0, 0.0, 0.0),
//                     scale: Vec3::splat(6.0),
//                     ..Default::default()
//                 },
//                 sprite: TextureAtlasSprite::new(sprite_index as u32),
//                 texture_atlas: atlas_handle.clone(),
//                 ..Default::default()
//             });
//         }
//     });
// }
