use bevy::math::Vec2;

pub struct Speed {
    pub current: Vec2,
}

impl Default for Speed {
    fn default() -> Self {
        return Speed {
            current: Vec2::new(0.0, 0.0),
        };
    }
}
