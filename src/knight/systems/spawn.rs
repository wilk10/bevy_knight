use bevy::prelude::*;

use crate::knight::components::knight::Knight;
use crate::load::resources::sprites::Sprites;

pub fn spawn_knight(
    mut commands: Commands,
    sprites: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let knight = Knight::default();

    if let Some(knight_atlas_handle) = sprites.get("knight") {
        let knight_atlas = texture_atlases.get(knight_atlas_handle).unwrap_or_else(|| {
            panic!(
                "did not find knight_atlas_handle: {:?}",
                knight_atlas_handle
            )
        });

        let sprite_index = 1 % knight_atlas.textures.len();
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: knight.initial_position,
                    scale: knight.scale,
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprite_index as u32),
                texture_atlas: knight_atlas_handle.clone(),
                ..Default::default()
            })
            .with(knight);
    }
}
