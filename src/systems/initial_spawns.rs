use bevy::{
    math::Vec3,
    prelude::{
        AssetServer, Assets, Commands, Handle, Query, Res, ResMut, SpriteSheetBundle, State,
        Texture, Transform,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_tilemap::{Tile, Tilemap};

use crate::{
    app_state::AppState,
    components::{
        player::{Player, PlayerBundle, PlayerName},
        speed::Speed,
    },
    game_state::GameState,
    sprite_handles::Sprites,
};

const CHUNK_WIDTH: u32 = 24;
const CHUNK_HEIGHT: u32 = 24;
const TILEMAP_WIDTH: i32 = CHUNK_WIDTH as i32 * 48;
const TILEMAP_HEIGHT: i32 = CHUNK_HEIGHT as i32 * 48;

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprite_handles: Res<Sprites>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut state: ResMut<State<AppState>>,
) {
    let texture_atlas = texture_atlases.get(&sprite_handles.atlas_handle).unwrap();
    let sprite_index = texture_atlas
        .get_texture_index(&sprite_handles.player_sprite_handle)
        .unwrap();

    commands.spawn_bundle(PlayerBundle {
        name: PlayerName("Goblin?".to_string()),
        speed: Speed { x: 0, y: 0 },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(sprite_index as u32),
            texture_atlas: sprite_handles.atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        player: Player,
    });

    state.set(AppState::Playing).unwrap(); // For now
}

pub fn build_random_dungeon(
    mut game_state: ResMut<GameState>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Tilemap>,
) {
    if game_state.map_loaded {
        return;
    }

    for mut map in query.iter_mut() {
        // Then we need to find out what the handles were to our textures we are going to use.
        let floor_sprite: Handle<Texture> = asset_server.get_handle("sprites/background.png");
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let floor_idx = texture_atlas.get_texture_index(&floor_sprite).unwrap();

        // Now we fill the entire space with floors.
        let mut tiles = Vec::new();
        for y in 0..TILEMAP_HEIGHT {
            for x in 0..TILEMAP_WIDTH {
                let y = y - TILEMAP_HEIGHT / 2;
                let x = x - TILEMAP_WIDTH / 2;
                // By default tile sets the Z order at 0. Lower means that tile
                // will render lower than others. 0 is the absolute bottom
                // level which is perfect for backgrounds.
                let tile = Tile {
                    point: (x, y),
                    sprite_index: floor_idx,
                    ..Default::default()
                };
                tiles.push(tile);
            }
        }

        // Now we pass all the tiles to our map.
        map.insert_tiles(tiles).unwrap();

        game_state.map_loaded = true;
    }
}
