// use std::ops::Deref;

use crate::load::plugin::TextureAtlasEntity;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Default for LevelPlugin {
    fn default() -> Self {
        LevelPlugin
    }
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, build_level.system());
    }
}

fn build_level(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<&TextureAtlasEntity>,
) {
    for texture_atlas_entity in query.iter() {
        let texture_atlas = texture_atlases
            .get(texture_atlas_entity.handle.clone())
            .unwrap();
        let sprite_index = texture_atlas.len() - 1;
        commands.spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(6.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprite_index as u32),
            texture_atlas: texture_atlas_entity.handle.clone(),
            ..Default::default()
        });
    }
}
