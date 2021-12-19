use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

#[derive(Default, Component)]
pub struct CurrentAction {
    pub interact: Option<InteractAction>,
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

#[derive(Debug, PartialEq)]
pub enum InteractAction {
    PlantCrop(PlantCropAction),
    DropFloors,
    DigAction(DigAction),
}

#[derive(Debug, PartialEq)]
pub struct PlantCropAction {
    pub position: Vec2,
}

#[derive(Debug, PartialEq)]
pub struct DigAction {
    pub position: Vec2,
}
