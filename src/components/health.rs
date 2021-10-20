#[derive(Clone, Copy, Debug, Default)]
pub struct Health {
    pub current_health: i32,
    pub max_health: i32,
}

impl Health {
    pub fn same_health(health: i32) -> Health {
        Health {
            current_health: health,
            max_health: health,
        }
    }

    pub fn has_no_health(&self) -> bool {
        self.current_health == 0
    }
}

pub struct HealthTextureMap {
    pub min_health: i32,
    pub max_health: i32,
    pub texture_index: usize,
}
