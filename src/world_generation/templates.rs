use super::cell::Cell;
use super::rooms::Room;

pub fn two_by_two_templates() -> Vec<Room> {
    let max_side_length: usize = 4;
    return vec![
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(3, 1),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(3, 2),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(1, 0),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(2, 0),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(1, 0),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(0, 1),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(0, 2),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 2),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(3, 1),
            ],
        },
        Room {
            max_side_length,
            cells: vec![
                // Walls
                Cell::splatted_room_wall(0),
                Cell::room_wall(0, 1),
                Cell::room_wall(0, 2),
                Cell::room_wall(0, 3),
                Cell::room_wall(1, 3),
                Cell::room_wall(2, 3),
                Cell::splatted_room_wall(3),
                Cell::room_wall(3, 1),
                Cell::room_wall(3, 0),
                Cell::room_wall(2, 0),
                Cell::room_wall(1, 0),
                // Floor
                Cell::splatted_room_floor(1),
                Cell::room_floor(1, 2),
                Cell::splatted_room_floor(2),
                Cell::room_floor(2, 1),
                // Door
                Cell::room_door(3, 2),
            ],
        },
    ];
}
