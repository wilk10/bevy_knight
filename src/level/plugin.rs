use bevy::prelude::*;
use tiled::parse_file;

use std::path::Path;

use crate::{
    constants::Const,
    level::components::tile::Tile,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_tiled_map.system());
    }
}

fn spawn_tiled_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_sheet_handle = asset_server.load("map/tilesheet.png");
    let tiles_atlas = TextureAtlas::from_grid(tile_sheet_handle, Vec2::new(32.0, 32.0), 3, 7);
    let tiles_atlas_handle = texture_atlases.add(tiles_atlas);
    let map_path = "assets/map/map.tmx";
    let map = parse_file(&Path::new(map_path))
        .unwrap_or_else(|error| panic!("{:?} || could not parse the map at {:?}", error, map_path));

    for layer in map.layers.iter() {
        for i in 0..map.width {
            for j in 0..map.height {
                let tile = match &layer.tiles {
                    tiled::LayerData::Finite(tiles) => &tiles[j as usize][i as usize],
                    _ => panic!("infinte maps not supported"),
                };

                if tile.gid == 0 {
                    continue;
                }

                let window_height = Const::map_n_tiles().y * Const::tile_size().y;
                let tile_x = (Const::tile_size().x * i as f32) * Const::global_scale();
                let tile_y =
                    (window_height / 2.0 - Const::tile_size().y * j as f32) * Const::global_scale();

                commands
                    .spawn(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(tile_x, tile_y, 10.0),
                            scale: Vec3::splat(Const::global_scale()),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(tile.gid - 1),
                        texture_atlas: tiles_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .with(Tile::default());
            }
        }
    }
}
