use std::collections::HashMap;

use super::rooms::Room;

const DEFAULT_GRID_SIZE: usize = 100;

#[derive(PartialEq)]
pub enum CellType {
    Floor,
    RoomFloor,
    OuterWall,
    RoomWall,
    Door,
    None,
}

pub struct Cell {
    pub cell_type: CellType,
    pub x: i32,
    pub y: i32,
}

pub struct Grid {
    pub cells: HashMap<(i32, i32), Cell>,
    size: usize,
}

impl Grid {
    fn add_cell(&mut self, cell: Cell) {
        self.cells.insert((cell.x, cell.y), cell);
    }

    fn set_cell_type(&mut self, x: i32, y: i32, cell_type: CellType) {
        if let Some(mut cell) = self.cells.get_mut(&(x, y)) {
            cell.cell_type = cell_type;
        };
    }

    pub fn add_room(&mut self, room: Room) {
        for wall_coordinate in room.wall_coordinates {
            self.set_cell_type(wall_coordinate.0, wall_coordinate.1, CellType::RoomWall);
        }

        for floor_coordinate in room.floor_coordinates {
            self.set_cell_type(floor_coordinate.0, floor_coordinate.1, CellType::RoomFloor);
        }

        self.set_cell_type(room.door_coordinate.0, room.door_coordinate.1, CellType::Door);
    }

    pub fn fill_empty_cells(&mut self) {
        for mut cell in self.cells.values_mut() {
            if cell.cell_type == CellType::None {
                cell.cell_type = CellType::Floor;
            }
        }
    }

    pub fn create_outer_wall(&mut self) {
        // X rows
        for x in -1..=self.size as i32 {
            let bottom_y: i32 = -1;
            let cell = Cell {
                x,
                cell_type: CellType::OuterWall,
                y: bottom_y,
            };
            self.add_cell(cell);

            let top_y: i32 = self.size as i32;
            let cell = Cell {
                x,
                cell_type: CellType::OuterWall,
                y: top_y,
            };
            self.add_cell(cell);
        }

        // Y rows
        for y in 0..=self.size as i32 {
            let left_x = -1;
            let cell = Cell {
                y,
                x: left_x,
                cell_type: CellType::OuterWall,
            };
            self.add_cell(cell);

            let right_x = self.size as i32;
            let cell = Cell {
                y,
                x: right_x,
                cell_type: CellType::OuterWall,
            };
            self.add_cell(cell);
        }
    }

    pub fn is_cell_empty(&self, x: i32, y: i32) -> bool {
        let cell = self.cells.get(&(x, y));
        match cell {
            Some(c) => c.cell_type == CellType::None,
            None => false,
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Self {
            cells: HashMap::default(),
            size: DEFAULT_GRID_SIZE,
        };

        for x in 0..DEFAULT_GRID_SIZE as i32 {
            for y in 0..DEFAULT_GRID_SIZE as i32 {
                let cell = Cell {
                    cell_type: CellType::None,
                    x,
                    y,
                };
                grid.add_cell(cell);
            }
        }

        return grid;
    }
}
