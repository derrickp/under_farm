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

#[derive(PartialEq, Eq)]
pub struct Cell {
    pub cell_type: CellType,
    pub coordinate: Coordinate<i32>,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_type: CellType) -> Self {
        return Self {
            cell_type,
            coordinate: Coordinate::new(x, y),
        };
    }

    pub fn splat(value: i32, cell_type: CellType) -> Self {
        return Self {
            cell_type,
            coordinate: Coordinate::splat(value),
        };
    }

    pub fn splatted_room_wall(value: i32) -> Self {
        return Self::splat(value, CellType::RoomWall);
    }

    pub fn room_wall(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::RoomWall);
    }

    pub fn splatted_room_floor(value: i32) -> Self {
        return Self::splat(value, CellType::RoomFloor);
    }

    pub fn room_floor(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::RoomFloor);
    }

    pub fn room_door(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::Door);
    }

    pub fn outer_wall(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::OuterWall);
    }

    pub fn empty_cell(x: i32, y: i32) -> Self {
        return Self::new(x, y, CellType::None);
    }
}
