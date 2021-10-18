use rand::Rng;

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
