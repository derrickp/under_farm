use std::ops::Add;

use bevy::math::Vec2;

pub struct Speed {
    pub current: Vec2,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    None,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Add for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs == Direction::None {
            return self;
        } else if self == Direction::None {
            return rhs;
        } else if self == rhs {
            return self;
        }

        return match (self, rhs) {
            (Direction::North, Direction::East) => Direction::NorthEast,
            (Direction::East, Direction::North) => Direction::NorthEast,
            (Direction::North, Direction::West) => Direction::NorthWest,
            (Direction::West, Direction::North) => Direction::NorthWest,
            (Direction::South, Direction::East) => Direction::SouthEast,
            (Direction::East, Direction::South) => Direction::SouthEast,
            (Direction::South, Direction::West) => Direction::SouthWest,
            (Direction::West, Direction::South) => Direction::SouthWest,
            (Direction::NorthWest, Direction::North) => Direction::NorthWest,
            (Direction::NorthEast, Direction::North) => Direction::NorthEast,
            (Direction::NorthWest, Direction::West) => Direction::NorthWest,
            (Direction::NorthEast, Direction::East) => Direction::NorthEast,
            (Direction::SouthWest, Direction::South) => Direction::SouthWest,
            (Direction::SouthEast, Direction::South) => Direction::SouthEast,
            (Direction::SouthWest, Direction::West) => Direction::SouthWest,
            (Direction::SouthEast, Direction::East) => Direction::SouthEast,
            _ => Direction::None,
        };
    }
}

impl Default for Speed {
    fn default() -> Self {
        return Speed {
            current: Vec2::new(0.0, 0.0),
        };
    }
}
