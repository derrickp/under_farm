use bevy::{
    asset::LoadState,
    core::Timer,
    math::Vec3,
    prelude::{
        AssetServer, Assets, Commands, OrthographicCameraBundle, Res, ResMut, State, Texture,
        Transform, Visible,
    },
    sprite::{TextureAtlas, TextureAtlasBuilder},
};
use bevy_tiled_prototype::TiledMapCenter;
use bevy_tilemap::{
    prelude::{LayerKind, TilemapBundle},
    Tilemap, TilemapLayer,
};

const CHUNK_WIDTH: u32 = 24;
const CHUNK_HEIGHT: u32 = 24;
const TILEMAP_WIDTH: i32 = CHUNK_WIDTH as i32 * 48;
const TILEMAP_HEIGHT: i32 = CHUNK_HEIGHT as i32 * 48;

use crate::{
    app_state::AppState,
    game_state::GameState,
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
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    game_state.map_loaded = false;
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in loaded_textures.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);
    sprite_handles.atlas_handle = atlas_handle.clone();

    let texture_handle = asset_server.load("sprites/goblin_big_hat.png");
    sprite_handles.player_sprite_handle = texture_handle;

    let background_handle = asset_server.get_handle("sprites/background.png");
    sprite_handles.background_handle = background_handle;

    let tilemap = Tilemap::builder()
        .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
        .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
        .texture_dimensions(24, 24)
        .auto_chunk()
        .auto_spawn(2, 2)
        .add_layer(
            TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
        )
        .texture_atlas(atlas_handle.clone())
        .finish()
        .unwrap();

    let tilemap_components = TilemapBundle {
        tilemap,
        visible: Visible {
            is_visible: true,
            is_transparent: true,
        },
        transform: Default::default(),
        global_transform: Default::default(),
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(tilemap_components)
        .insert(Timer::from_seconds(0.075, true));
    commands.spawn_bundle(bevy_tiled_prototype::TiledMapBundle {
        map_asset: asset_server.load("maps/cave_1.tmx"),
        origin: Transform::from_scale(Vec3::new(4.0, 4.0, 1.0)),
        center: TiledMapCenter(true),
        ..Default::default()
    });

    state.set(AppState::SpritesLoaded).unwrap();
}
