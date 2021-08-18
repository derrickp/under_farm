#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl Coordinate<i32> {
    pub fn new(x: i32, y: i32) -> Self {
        return Coordinate { x, y };
    }

    pub fn splat(value: i32) -> Self {
        return Coordinate { x: value, y: value };
    }
}
