use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};

pub struct LoadPlugin;

impl Default for LoadPlugin {
    fn default() -> Self {
        LoadPlugin
    }
}

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpriteHandles>()
            .insert_resource(State::new(LoadAssetsState::InProgress))
            .add_stage_after(
                CoreStage::First,
                LoadAssetsStage,
                StateStage::<LoadAssetsState>::default(),
            )
            .on_state_enter(
                LoadAssetsStage,
                LoadAssetsState::InProgress,
                load_textures.system(),
            )
            .on_state_update(
                LoadAssetsStage,
                LoadAssetsState::InProgress,
                check_textures.system(),
            )
            .on_state_enter(
                LoadAssetsStage,
                LoadAssetsState::Finished,
                spawn_atlas.system(),
            );
    }
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

pub struct TextureAtlasEntity {
    pub handle: Handle<TextureAtlas>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct LoadAssetsStage;

#[derive(Clone)]
enum LoadAssetsState {
    InProgress,
    Finished,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("textures/").unwrap(); // should panic if not found
}

fn check_textures(
    mut state: ResMut<State<LoadAssetsState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set_next(LoadAssetsState::Finished).unwrap(); // should panic if AppState::Finished is already in the queue or is already the current state
    }
}

fn spawn_atlas(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = sprite_handles
        .handles
        .iter()
        .fold(
            TextureAtlasBuilder::default(),
            |mut texture_atlas_builder, handle| {
                if let Some(texture) = textures.get(handle) {
                    let texture_handle = handle.clone_weak().typed::<Texture>();
                    texture_atlas_builder.add_texture(texture_handle, texture)
                }
                texture_atlas_builder
            },
        )
        .finish(&mut textures)
        .unwrap(); // panics if there's not enough space for all textures on the atlas. set different defaults

    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((TextureAtlasEntity {
        handle: atlas_handle,
    },));
}
