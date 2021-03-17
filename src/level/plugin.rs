use bevy::prelude::*;
use tiled::parse_file;

use std::path::Path;

use crate::constants::{MAP_N_TILES_HEIGHT, SCALE, TILE_SIZE};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, spawn_tiled_map.system());
    }
}

#[allow(dead_code)]
pub struct Tile {
    size: Vec2,
}

fn spawn_tiled_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_sheet_handle = asset_server.load("map/tilesheet.png");
    let tiles_atlas = TextureAtlas::from_grid(tile_sheet_handle, Vec2::new(32.0, 32.0), 3, 7);
    let tiles_atlas_handle = texture_atlases.add(tiles_atlas);

    let map = parse_file(&Path::new("assets/map/map.tmx")).unwrap();

    for layer in map.layers.iter() {
        for i in 0..map.width {
            for j in 0..map.height {
                let tile = match &layer.tiles {
                    tiled::LayerData::Finite(tiles) => &tiles[j as usize][i as usize],
                    _ => panic!("Infinte maps not supported"),
                };

                if tile.gid == 0 {
                    continue;
                }

                let window_height = MAP_N_TILES_HEIGHT * TILE_SIZE;
                let tile_x = (TILE_SIZE * i as f32) * SCALE;
                let tile_y = (window_height / 2.0 - TILE_SIZE * j as f32) * SCALE;

                commands
                    .spawn(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(tile_x, tile_y, 10.0),
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        },

                        sprite: TextureAtlasSprite::new(tile.gid - 1),
                        texture_atlas: tiles_atlas_handle.clone(),
                        ..Default::default()
                    })
                    .with(Tile {
                        size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                    });
            }
        }
    }
}
