use bevy::{asset::LoadState, prelude::*, sprite::TextureAtlasBuilder};
use std::collections::HashMap;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TextureHandles>()
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
                generate_atlas.system(),
            );
    }
}

const TEXTURES_FOLDER: &'static str = "textures";
const TEXTURE_TYPES: [&'static str; 2] = ["tiles", "characters"];

pub struct TextureHandles {
    pub atlas: Option<Handle<TextureAtlas>>,
    pub texture_map: HashMap<String, Vec<HandleUntyped>>,
}

impl Default for TextureHandles {
    fn default() -> Self {
        TEXTURE_TYPES.iter().fold(
            TextureHandles {
                atlas: None,
                texture_map: HashMap::new(),
            },
            |mut handles, texture_type| {
                handles
                    .texture_map
                    .insert(texture_type.to_string(), Vec::new());
                handles
            },
        )
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct LoadAssetsStage;

#[derive(Clone)]
enum LoadAssetsState {
    InProgress,
    Finished,
}

fn load_textures(mut texture_handles: ResMut<TextureHandles>, asset_server: Res<AssetServer>) {
    for (texture_type, handles) in texture_handles.texture_map.iter_mut() {
        *handles = asset_server
            .load_folder([TEXTURES_FOLDER, texture_type].join("/"))
            .unwrap()
    }
}

fn check_textures(
    mut state: ResMut<State<LoadAssetsState>>,
    texture_handles: ResMut<TextureHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(
        texture_handles
            .texture_map
            .iter()
            .flat_map(|(_, handles)| handles.iter().map(|handle| handle.id)),
    ) {
        state.set_next(LoadAssetsState::Finished).unwrap();
    }
}

fn generate_atlas(
    mut texture_handles: ResMut<TextureHandles>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas = texture_handles
        .texture_map
        .iter()
        .fold(
            TextureAtlasBuilder::default(),
            |mut texture_atlas_builder, (_, handles)| {
                for handle in handles.iter() {
                    if let Some(texture) = textures.get(handle) {
                        let texture_handle = handle.clone_weak().typed::<Texture>();
                        texture_atlas_builder.add_texture(texture_handle, texture)
                    }
                }
                texture_atlas_builder
            },
        )
        .finish(&mut textures)
        .unwrap();

    texture_handles.atlas = Some(texture_atlases.add(texture_atlas));
}
