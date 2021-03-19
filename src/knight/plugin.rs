use bevy::prelude::*;

use crate::knight::components::knight::Knight;
use crate::knight::events::CollisionEvent;
use crate::knight::systems::collision::detect_collision;
use crate::knight::systems::input::handle_movement;
use crate::load::plugin::LoadLabel;
use crate::load::resources::sprites::Sprites;

pub struct JustMoved;

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CollisionEvent>()
            .add_startup_system(spawn_knight.system().after(LoadLabel))
            .add_system(handle_movement.system())
            .add_system(detect_collision.system());
    }
}

fn spawn_knight(
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
        // .with(knight)
        // .with(Timer::from_seconds(0.1, true));
    }
}
