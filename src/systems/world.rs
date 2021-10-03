use bevy::{
    core::Time,
    prelude::{Commands, Res, ResMut},
};
use tdlg::{generator::Generator, loading::RoomPaths};

use crate::{states::GameLoadState, world::WorldTickTimer};

const DEFAULT_GRID_SIZE: usize = 150;
const NUMBER_OF_ROOMS: usize = 100;

pub fn tick_game_world(time: Res<Time>, mut timer: ResMut<WorldTickTimer>) {
    timer.0.tick(time.delta());
}

pub fn generate_world_grid(mut commands: Commands, mut load_state: ResMut<GameLoadState>) {
    let generator = Generator {
        grid_size: DEFAULT_GRID_SIZE,
        target_number_rooms: NUMBER_OF_ROOMS,
        all_room_paths: vec![
            RoomPaths {
                name: "two_by_two",
                base_template_path: "assets/room_templates/two_by_two",
                fill_template_paths: vec![
                    "assets/room_templates/two_by_two_fill/two_by_two_tables_1",
                    "assets/room_templates/two_by_two_fill/two_by_two_tables_2",
                    "assets/room_templates/two_by_two_fill/two_by_two_tables_3",
                    "assets/room_templates/two_by_two_fill/two_by_two_tables_4",
                ],
            },
            RoomPaths {
                name: "two_by_two_empty",
                base_template_path: "assets/room_templates/two_by_two",
                fill_template_paths: Vec::new(),
            },
            RoomPaths {
                name: "three_by_three",
                base_template_path: "assets/room_templates/three_by_three",
                fill_template_paths: Vec::new(),
            },
            RoomPaths {
                name: "four_by_four",
                base_template_path: "assets/room_templates/four_by_four",
                fill_template_paths: Vec::new(),
            },
            RoomPaths {
                name: "four_by_four_broken",
                base_template_path: "assets/room_templates/four_by_four_broken",
                fill_template_paths: vec!["assets/room_templates/four_by_four_rubble_1"],
            },
            RoomPaths {
                name: "other_rooms",
                base_template_path: "assets/room_templates/other",
                fill_template_paths: Vec::new(),
            },
        ],
    };
    let world = generator.generate_top_down_map().unwrap();
    println!("{}", world.room_count);
    commands.insert_resource(world.grid);

    load_state.game_world_generated = true;
}
