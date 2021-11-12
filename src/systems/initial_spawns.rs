use bevy::{
    prelude::{AssetServer, Commands, Query, Res, ResMut},
    window::Windows,
};

use tdlg::{cells::layer::LayerType, grid::Grid};

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
    configuration::{map::world_coordinate_from_grid, structures::StructuresConfig},
    sprites::Sprites,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut grid: ResMut<Grid>,
    query: Query<&Player>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    structures_config: Res<StructuresConfig>,
) {
    if query.single().is_ok() {
        return;
    }

    for cell in grid.cells.values() {
        for layer in cell.layers.iter() {
            let coordinate = world_coordinate_from_grid(&cell.coordinate);
            match *layer {
                LayerType::Floor => {
                    commands.spawn_bundle(GroundTileBundle::build_floor(&coordinate, &sprites));
                }
                LayerType::RoomWall => {
                    let structure_config = structures_config.config_by_key("room_wall").unwrap();
                    commands.spawn_bundle(GroundTileBundle::build_floor(&coordinate, &sprites));
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                    ));
                }
                LayerType::RoomFloor => {
                    commands
                        .spawn_bundle(GroundTileBundle::build_room_floor(&coordinate, &sprites));
                }
                LayerType::Door => {
                    commands
                        .spawn_bundle(GroundTileBundle::build_room_floor(&coordinate, &sprites));
                }
                LayerType::OuterWall => {
                    let structure_config = structures_config.config_by_key("outer_wall").unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                    ));
                }
                LayerType::Rubble => {
                    let structure_config = structures_config.config_by_key("rubble").unwrap();
                    commands.spawn_bundle(StructureBundle::build(
                        &coordinate,
                        &sprites.atlas_handle,
                        structure_config,
                    ));
                }
                LayerType::Table => {
                    let structure_config = structures_config.config_by_key("table").unwrap();
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
    let player_bundle = PlayerBundle::build_main_player(coordinate, &sprites);
    let player_text_bundle =
        PlayerStatsTextBundle::from_player_bundle(&player_bundle, &asset_server, &windows);
    commands.spawn_bundle(player_bundle);
    commands.spawn_bundle(player_text_bundle);

    for configuration in structures_config.configurations.iter() {
        println!("{}", configuration.key);
    }

    commands.spawn().insert(GameCameraState::default());
    commands.spawn().insert(Spawns::default());
    commands.spawn().insert(WorldTickTimer::default());
}
