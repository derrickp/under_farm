use rand::Rng;

use super::cell::Cell;

pub enum RoomSize {
    TwoByTwo,
}

pub struct Room {
    pub cells: Vec<Cell>,
}

impl Room {
    // 2x2 floor, so 4x4 with walls
    pub fn two_by_two_square(bottom_left_x: i32, bottom_left_y: i32) -> Self {
        let templates = Self::two_by_two_templates();

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..templates.len());

        let template = templates.get(index).unwrap();
        let cells = template
            .cells
            .iter()
            .map(|cell| {
                Cell::new(
                    cell.coordinate.x + bottom_left_x,
                    cell.coordinate.y + bottom_left_y,
                    cell.cell_type,
                )
            })
            .collect();

        return Self { cells };
    }

    fn two_by_two_templates() -> Vec<Self> {
        let mut templates: Vec<Self> = Vec::new();

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        templates.push(Self {
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
        });

        return templates;
    }
}

pub fn room_sizes(room_size: RoomSize) -> (i32, i32) {
    match room_size {
        RoomSize::TwoByTwo => (4, 4),
    }
}
