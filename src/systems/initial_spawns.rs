use bevy::prelude::{Commands, Res, ResMut};

use tdlg::{cell::CellLayerType, grid::Grid};

use crate::{
    components::{ground::GroundTileBundle, player::PlayerBundle, structure::StructureBundle},
    configuration::map::world_coordinate_from_grid,
    sprites::Sprites,
    states::GameState,
};

pub fn spawn_opening_bundles(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut game_state: ResMut<GameState>,
    mut grid: ResMut<Grid<i32>>,
) {
    if game_state.initial_spawn_complete {
        return;
    }

    for cell in grid.cells.values() {
        for layer in cell.layers.iter() {
            let coordinate = world_coordinate_from_grid(&cell.coordinate);
            match *layer {
                CellLayerType::Floor => {
                    commands.spawn_bundle(GroundTileBundle::build_floor(&coordinate, &sprites));
                }
                CellLayerType::RoomWall => {
                    commands.spawn_bundle(GroundTileBundle::build_floor(&coordinate, &sprites));
                    commands.spawn_bundle(StructureBundle::build_room_wall(&coordinate, &sprites));
                }
                CellLayerType::RoomFloor => {
                    commands
                        .spawn_bundle(GroundTileBundle::build_room_floor(&coordinate, &sprites));
                }
                CellLayerType::Door => {
                    commands
                        .spawn_bundle(GroundTileBundle::build_room_floor(&coordinate, &sprites));
                }
                CellLayerType::OuterWall => {
                    commands.spawn_bundle(StructureBundle::build_outer_wall(&coordinate, &sprites));
                }
                CellLayerType::Rubble => {
                    commands.spawn_bundle(StructureBundle::build_rubble(&coordinate, &sprites));
                }
                CellLayerType::Table => {
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

    game_state.initial_spawn_complete = true;
}
