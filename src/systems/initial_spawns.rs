use bevy::{
    prelude::{AssetServer, Commands, Query, Res},
    window::Windows,
};

use tdlg::{cells::layer::LayerType, coordinate::Coordinate, map::TopDownMap};

use crate::{
    components::{
        cameras::GameCameraState,
        ground::GroundTileBundle,
        player::{Player, PlayerBundle},
        spawns::Spawns,
        structure::StructureBundle,
        text::{PlayerStatsText, PlayerStatsTextBundle},
        world::WorldTickTimer,
    },
    configuration::{game::GameConfiguration, map::world_coordinate_from_grid},
    sprites::Sprites,
};

pub fn spawn_player_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    query: Query<&PlayerStatsText>,
) {
    if query.single().is_ok() {
        return;
    }

    let coordinate = Coordinate::splat(0);
    let player_text_bundle = PlayerStatsTextBundle::build(&coordinate, &asset_server, &windows);
    commands.spawn_bundle(player_text_bundle);
}

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    map: Res<TopDownMap>,
    query: Query<&Player>,
    game_config: Res<GameConfiguration>,
) {
    if query.single().is_ok() {
        return;
    }

    for cell in map.grid.cells.values() {
        for layer in cell.layers.iter() {
            let coordinate = world_coordinate_from_grid(
                &cell.coordinate,
                game_config.map_size(),
                game_config.tile_size(),
            );
            let floor_config = game_config
                .floors_config
                .config_by_key("cave_floor")
                .unwrap();
            match *layer {
                LayerType::Floor => {
                    commands.spawn_bundle(GroundTileBundle::build(
                        &coordinate,
                        &sprites,
                        floor_config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::RoomWall => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("room_wall")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        &coordinate,
                        &sprites,
                        floor_config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::RoomFloor => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        &coordinate,
                        &sprites,
                        config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Door => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(
                        &coordinate,
                        &sprites,
                        config,
                        game_config.sprite_config.scale,
                        game_config.tile_size(),
                    ));
                }
                LayerType::OuterWall => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("outer_wall")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Rubble => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("rubble")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                LayerType::Table => {
                    let structure_config = game_config
                        .structures_config
                        .config_by_key("table")
                        .unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                        &game_config.sprite_config,
                        game_config.tile_size(),
                    ));
                }
                _ => {}
            }
        }
    }

    let player_spawn = map.entry_coordinate;
    let coordinate = world_coordinate_from_grid(
        &player_spawn,
        game_config.world_config.world_stats.map_size,
        game_config.tile_size(),
    );
    let player_bundle = PlayerBundle::build_main_player(coordinate, &sprites, &game_config);
    commands.spawn_bundle(player_bundle);

    commands.spawn().insert(GameCameraState::default());
    commands.spawn().insert(Spawns::default());
    commands.spawn().insert(WorldTickTimer::default());
}
