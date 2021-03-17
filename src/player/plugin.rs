use bevy::prelude::*;

use crate::constants::{PLAYER_SPAWN_Y, SCALE, TILE_SIZE};
use crate::load::sprites::Sprites;
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
    if let Some(knight_atlas_handle) = sprites.get("knight") {
        let knight_atlas = texture_atlases.get(knight_atlas_handle).unwrap();
        let sprite_index = 1 % knight_atlas.textures.len();
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(TILE_SIZE / 2.0, PLAYER_SPAWN_Y, 10.0),
                    scale: Vec3::splat(SCALE * 2.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprite_index as u32),
                texture_atlas: knight_atlas_handle.clone(),
                ..Default::default()
            })
            .with(Player);
    }
}
