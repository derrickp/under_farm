use bevy::prelude::{Component, Entity};

#[derive(Default, Component)]
pub struct CurrentAction {
    pub interact_pressed: bool,
    pub hit: Option<HitAction>,
    pub pickup: Option<PickupAction>,
}

#[derive(Component, Default)]
pub struct WorldActions {
    pub grow_crops: bool,
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

#[derive(Clone, Copy)]
pub struct PickupAction {
    pub target: Entity,
}
