use rand::Rng;

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

#[derive(Clone, Copy, Debug)]
pub struct Damage {
    pub min_damage: i32,
    pub max_damage: i32,
}

impl Damage {
    pub fn damage_dealt(&self) -> i32 {
        if self.min_damage == self.max_damage {
            return self.max_damage;
        }

        let mut rng = rand::thread_rng();
        rng.gen_range(self.min_damage..self.max_damage)
    }
}
