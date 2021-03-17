use bevy::prelude::*;

use crate::constants::{MAP_N_TILES_WIDTH, PLAYER_SPAWN_Y, TILES_SIZE};
use crate::load::sprites::Sprites;
// use crate::level::plugin::MapMarker;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_player.system());
    }
}

pub struct Player;

fn spawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let map_width = MAP_N_TILES_WIDTH * TILES_SIZE;
    let player_x = -map_width / 2.0 + TILES_SIZE / 2.0;

    if let Some(knight_atlas_handle) = sprites.get("knight") {
        let knight_atlas = texture_atlases.get(knight_atlas_handle).unwrap();
        let sprite_index = 1 % knight_atlas.textures.len();
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(player_x, PLAYER_SPAWN_Y, 10.0),
                    scale: Vec3::splat(2.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprite_index as u32),
                texture_atlas: knight_atlas_handle.clone(),
                ..Default::default()
            })
            .with(Player);
    }
}
