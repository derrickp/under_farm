use rand::Rng;
use tdlg::cell::{Cell, CellType};
use tdlg::coordinate::Coordinate;
use tdlg::room::Room;
use std::collections::HashMap;

const DEFAULT_GRID_SIZE: usize = 100;

pub struct Grid {
    pub cells: HashMap<Coordinate<i32>, Cell<i32>>,
    size: usize,
}

impl Grid {
    fn add_cell(&mut self, cell: Cell<i32>) {
        self.cells.insert(cell.coordinate, cell);
    }

    fn set_cell_type(&mut self, x: i32, y: i32, cell_type: CellType) {
        let coordinate = Coordinate { x, y };
        if let Some(cell) = self.cells.get_mut(&coordinate) {
            cell.set_cell_type(cell_type);
        };
    }

    pub fn add_room(&mut self, room: Room<i32>) {
        for cell in room.cells.iter() {
            self.set_cell_type(cell.coordinate.x, cell.coordinate.y, cell.cell_type);
        }
    }

    pub fn fill_empty_cells(&mut self) {
        for cell in self.cells.values_mut() {
            if cell.cell_type == CellType::Empty {
                cell.set_cell_type(CellType::Floor);
            }
        }
    }

    pub fn create_outer_wall(&mut self) {
        // X rows
        for x in -1..=self.size as i32 {
            self.add_cell(Cell::outer_wall(x, -1));
            self.add_cell(Cell::outer_wall(x, self.size as i32));
        }

        // Y rows
        for y in 0..=self.size as i32 {
            self.add_cell(Cell::outer_wall(-1, y));
            self.add_cell(Cell::outer_wall(self.size as i32, y));
        }
    }

    pub fn is_cell_empty(&self, coordinate: &Coordinate<i32>) -> bool {
        let cell = self.cells.get(coordinate);
        match cell {
            Some(c) => c.cell_type == CellType::Empty,
            None => false,
        }
    }

    pub fn random_spawnable_coordinate(&self) -> Option<Coordinate<i32>> {
        let mut rng = rand::thread_rng();
        let spawnable_cells: Vec<Coordinate<i32>> = self
            .cells
            .iter()
            .filter_map(|(coordinate, cell)| {
                if cell.spawnable {
                    Some(coordinate.clone())
                } else {
                    None
                }
            })
            .collect();
        let index: usize = rng.gen_range(0..spawnable_cells.len());
        if let Some(coordinate) = spawnable_cells.get(index) {
            return Some(coordinate.clone());
        } else {
            return None;
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
                grid.add_cell(Cell::empty_cell(x, y));
            }
        }

        return grid;
    }
}
