use bevy::prelude::Entity;

#[derive(Default)]
pub struct CurrentAction {
    pub interact_pressed: bool,
    pub hit: Option<HitAction>,
}

impl CurrentAction {
    pub fn hit_entity(&mut self, damage: i32, target: Entity) {
        self.hit = Some(HitAction { damage, target })
    }
}

#[derive(Clone, Copy)]
pub struct HitAction {
    pub damage: i32,
    pub target: Entity,
}
