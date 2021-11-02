use bevy::prelude::{Commands, Res, ResMut};

use tdlg::{cells::layer::LayerType, grid::Grid};

use crate::{
    components::{
        cameras::GameCameraState, ground::GroundTileBundle, player::PlayerBundle, spawns::Spawns,
        structure::StructureBundle,
    },
    configuration::map::world_coordinate_from_grid,
    sprites::Sprites,
    states::GameState,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut game_state: ResMut<GameState>,
    mut grid: ResMut<Grid>,
) {
    if game_state.initial_spawn_complete {
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
                    commands.spawn_bundle(GroundTileBundle::build_floor(&coordinate, &sprites));
                    commands.spawn_bundle(StructureBundle::build_room_wall(&coordinate, &sprites));
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
                    commands.spawn_bundle(StructureBundle::build_outer_wall(&coordinate, &sprites));
                }
                LayerType::Rubble => {
                    commands.spawn_bundle(StructureBundle::build_rubble(&coordinate, &sprites));
                }
                LayerType::Table => {
                    commands.spawn_bundle(StructureBundle::build_table(&coordinate, &sprites));
                }
                _ => {}
            }
        }
    }

    let player_spawn = grid.random_spawnable_coordinate().unwrap();
    println!("{} {}", player_spawn.x, player_spawn.y);
    let coordinate = world_coordinate_from_grid(&player_spawn);
    commands.spawn_bundle(PlayerBundle::build_main_player(coordinate, &sprites));

    commands.spawn().insert(GameCameraState::default());
    commands.spawn().insert(Spawns::default());

    game_state.initial_spawn_complete = true;
}
