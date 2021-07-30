use bevy::{
    asset::LoadState,
    prelude::{AssetServer, Assets, Res, ResMut, State, Texture},
    sprite::{TextureAtlas, TextureAtlasBuilder},
};

use crate::{
    sprites::{LoadedTextures, Sprites},
    states::AppState,
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
        state.set(AppState::FinishedLoading).unwrap();
    }
}

pub fn load_sprites(
    mut sprites: ResMut<Sprites>,
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
    let texture_handle = asset_server.load("sprites/goblin_big_hat.png");
    let background_handle = asset_server.get_handle("sprites/second_background.png");
    let outline_handle = asset_server.get_handle("sprites/cell_outline_32.png");
    let mushroom_handle = asset_server.get_handle("sprites/giant_mushroom.png");
    sprites.player_sprite_index = texture_atlas.get_texture_index(&texture_handle).unwrap();
    sprites.background_index = texture_atlas.get_texture_index(&background_handle).unwrap();
    sprites.outline_index = texture_atlas.get_texture_index(&outline_handle).unwrap();
    sprites.mushroom_index = texture_atlas.get_texture_index(&mushroom_handle).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);
    sprites.atlas_handle = atlas_handle;

    state.set(AppState::InGame).unwrap();
}
