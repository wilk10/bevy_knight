use bevy::prelude::*;

use crate::load::plugin::TextureHandles;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PreUpdate, build_level.system());
    }
}

fn build_level(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_handles: ResMut<TextureHandles>,
) {
    texture_handles.atlas.as_ref().map(|atlas_handle| {
        let atlas = texture_atlases.get(atlas_handle.clone()).unwrap();
        let sprite_index = atlas.len() - 1;

        commands.spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(6.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprite_index as u32),
            texture_atlas: atlas_handle.clone(),
            ..Default::default()
        });
    });
}
