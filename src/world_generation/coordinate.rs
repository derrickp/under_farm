#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        return Coordinate { x, y };
    }

    pub fn splat(value: i32) -> Self {
        return Coordinate { x: value, y: value };
    }
}
