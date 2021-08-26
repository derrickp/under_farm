use super::coordinate::Coordinate;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CellType {
    Floor,
    RoomFloor,
    OuterWall,
    RoomWall,
    Door,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,
    pub coordinate: Coordinate<i32>,
    pub spawnable: bool,
}

impl Cell {
    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.cell_type = cell_type;
        match self.cell_type {
            CellType::Floor => self.spawnable = true,
            CellType::RoomFloor => self.spawnable = true,
            _ => {}
        }
    }

    pub fn translate(&self, x: i32, y: i32) -> Self {
        return Self {
            coordinate: Coordinate::new(x, y),
            cell_type: self.cell_type,
            spawnable: self.spawnable,
        };
    }

    pub fn new(x: i32, y: i32, cell_type: CellType, spawnable: bool) -> Self {
        return Self {
            cell_type,
            spawnable,
            coordinate: Coordinate::new(x, y),
        };
    }

    pub fn splat(value: i32, cell_type: CellType, spawnable: bool) -> Self {
        return Self {
            cell_type,
            spawnable,
            coordinate: Coordinate::splat(value),
        };
    }

    pub fn splatted_room_wall(value: i32) -> Self {
        return Self::splat(value, CellType::RoomWall, false);
    }

    pub fn room_wall(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::RoomWall, false);
    }

    pub fn splatted_room_floor(value: i32) -> Self {
        return Self::splat(value, CellType::RoomFloor, true);
    }

    pub fn room_floor(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::RoomFloor, true);
    }

    pub fn room_door(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::Door, false);
    }

    pub fn outer_wall(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::OuterWall, false);
    }

    pub fn empty_cell(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::None, false);
    }
}
