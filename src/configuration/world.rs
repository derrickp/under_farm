use tdlg::{generator::Generator, loading::RoomPaths};

use super::map::MAP_SIZE;

const NUMBER_OF_ROOMS: usize = 100;

fn room_configurations() -> Vec<RoomPaths> {
    vec![
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
    ]
}

pub fn generator() -> Generator {
    Generator {
        grid_size: MAP_SIZE,
        target_number_rooms: NUMBER_OF_ROOMS,
        all_room_paths: room_configurations(),
        seed: "godzilla mothra",
    }
}
