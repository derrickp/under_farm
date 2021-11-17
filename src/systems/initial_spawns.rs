use bevy::{
    prelude::{AssetServer, Commands, Query, Res, ResMut},
    window::Windows,
};

use tdlg::{cells::layer::LayerType, coordinate::Coordinate, grid::Grid};

use crate::{
    components::{
        cameras::GameCameraState,
        ground::GroundTileBundle,
        player::{Player, PlayerBundle},
        spawns::Spawns,
        structure::StructureBundle,
        text::PlayerStatsTextBundle,
        world::WorldTickTimer,
    },
    configuration::{game::GameConfiguration, map::world_coordinate_from_grid},
    sprites::Sprites,
};

pub fn spawn_player_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let coordinate = Coordinate::splat(0);
    let player_text_bundle = PlayerStatsTextBundle::build(&coordinate, &asset_server, &windows);
    commands.spawn_bundle(player_text_bundle);
}

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut grid: ResMut<Grid>,
    query: Query<&Player>,
    game_config: Res<GameConfiguration>,
) {
    if query.single().is_ok() {
        return;
    }

    for cell in grid.cells.values() {
        for layer in cell.layers.iter() {
            let coordinate = world_coordinate_from_grid(&cell.coordinate);
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
                    ));
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                    ));
                }
                LayerType::RoomFloor => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(&coordinate, &sprites, config));
                }
                LayerType::Door => {
                    let config = game_config
                        .floors_config
                        .config_by_key("room_floor")
                        .unwrap();
                    commands.spawn_bundle(GroundTileBundle::build(&coordinate, &sprites, config));
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
                    ));
                }
                _ => {}
            }
        }
    }

    let player_spawn = grid.random_spawnable_coordinate().unwrap();
    let coordinate = world_coordinate_from_grid(&player_spawn);
    let player_bundle =
        PlayerBundle::build_main_player(coordinate, &sprites, &game_config.player_config);
    commands.spawn_bundle(player_bundle);

    commands.spawn().insert(GameCameraState::default());
    commands.spawn().insert(Spawns::default());
    commands.spawn().insert(WorldTickTimer::default());
}
