use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Assets, Res, ResMut, State, Texture},
    sprite::{TextureAtlas, TextureAtlasBuilder},
};

use crate::{
    app_state::AppState,
    sprite_handles::{LoadedTextures, Sprites},
};

pub fn load_textures(mut loaded_textures: ResMut<LoadedTextures>, asset_server: Res<AssetServer>) {
    loaded_textures.handles = asset_server.load_folder("sprites").unwrap();
}

pub fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: Res<LoadedTextures>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::TexturesLoaded).unwrap();
    }
}

pub fn load_sprites(
    mut sprite_handles: ResMut<Sprites>,
    loaded_textures: Res<LoadedTextures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in loaded_textures.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);
    sprite_handles.atlas_handle = atlas_handle;

    let texture_handle = asset_server.load("sprites/goblin_big_hat.png");
    sprite_handles.player_sprite_handle = texture_handle;

    let background_handle = asset_server.get_handle("sprites/background.png");
    sprite_handles.background_handle = background_handle;

    state.set(AppState::SpritesLoaded).unwrap();
}
