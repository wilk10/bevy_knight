use bevy::prelude::*;

use crate::load::sprites::Sprites;

pub struct LoadSpritesPlugin;

impl Plugin for LoadSpritesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Sprites>()
            .add_startup_system(load_sprites.system());
    }
}

fn load_sprites(
    mut sprites: ResMut<Sprites>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let knight_handle: Handle<Texture> = asset_server.load("textures/characters/knight.png");
    let knight_atlas = TextureAtlas::from_grid(knight_handle, Vec2::new(32.0, 48.0), 1, 1);
    let knight_atlas_handle = texture_atlases.add(knight_atlas);
    sprites.add("knight", knight_atlas_handle);
}
