use rand::Rng;

use super::{cell::Cell, templates::all_templates};

#[derive(Clone)]
pub struct Room {
    pub cells: Vec<Cell>,
    pub max_side_length: usize,
}

impl Room {
    pub fn random_template() -> Self {
        let templates = all_templates();

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..templates.len());

        return templates.get(index).unwrap().clone();
    }

    // 2x2 floor, so 4x4 with walls
    pub fn move_room_coordinates(bottom_left_x: i32, bottom_left_y: i32, template: Room) -> Self {
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

        return Self {
            cells,
            max_side_length: template.max_side_length,
        };
    }
}
