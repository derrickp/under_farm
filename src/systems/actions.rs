use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Mut, Query, Transform},
    sprite::TextureAtlasSprite,
};

use crate::components::{
    action::{CurrentAction, InteractAction},
    body::Body,
    bounding_box::BoundingBox,
    crop::{Crop, CropSpawn},
    item::{Item, ItemType},
    player::{Player, PlayerInventory},
    spawns::Spawns,
    structure::{Structure, StructureSpawn},
};

pub fn hit_actions(
    player_query: Query<(&Player, &CurrentAction)>,
    mut structure_query: Query<(&mut Structure, &mut TextureAtlasSprite)>,
) {
    if player_query.is_empty() {
        return;
    }

    let (_, current_action): (&Player, &CurrentAction) = player_query.single();

    let hit = match current_action.hit {
        Some(it) => it,
        _ => return,
    };

    let (mut structure, mut sprite): (Mut<Structure>, Mut<TextureAtlasSprite>) =
        match structure_query.get_mut(hit.target) {
            Ok(it) => it,
            _ => return,
        };

    structure.damage(hit.damage);

    if let Some(sprite_index) = structure.current_texture_index() {
        sprite.index = sprite_index as u32;
    }
}

pub fn pickup_actions(
    mut commands: Commands,
    mut player_query: Query<(&Player, &CurrentAction, &mut PlayerInventory)>,
    item_query: Query<&Item>,
) {
    if player_query.is_empty() {
        return;
    }

    let (_, current_action, mut player_inventory): (&Player, &CurrentAction, Mut<PlayerInventory>) =
        player_query.single_mut();

    let pickup = match current_action.pickup {
        Some(it) => it,
        _ => return,
    };

    let item: &Item = match item_query.get(pickup.target) {
        Ok(it) => it,
        _ => return,
    };

    match &item.item_type {
        ItemType::Tool(tool) => {
            if !player_inventory
                .held_tools
                .iter()
                .any(|config| config.key() == tool.key())
            {
                commands.entity(pickup.target).despawn();
                player_inventory.held_tools.push(tool.clone())
            }
        }
    }
}

pub fn clear_structure_action(mut commands: Commands, query: Query<(&Player, &CurrentAction)>) {
    if query.is_empty() {
        return;
    }

    let (_, action): (&Player, &CurrentAction) = query.single();

    if let Some(InteractAction::ClearAction(clear)) = &action.interact {
        commands.entity(clear.entity).despawn()
    }
}

pub fn reset_pickup_actions(mut query: Query<(&Player, &mut CurrentAction)>) {
    if query.is_empty() {
        return;
    }

    let (_, mut current_action): (&Player, Mut<CurrentAction>) = query.single_mut();
    current_action.pickup = None;
}

pub fn reset_hit_actions(mut query: Query<(&Player, &mut CurrentAction)>) {
    if query.is_empty() {
        return;
    }

    let (_, mut current_action): (&Player, Mut<CurrentAction>) = query.single_mut();
    current_action.hit = None;
}

pub fn dig_action(query: Query<(&Player, &CurrentAction)>, mut spawns_query: Query<&mut Spawns>) {
    if query.is_empty() {
        return;
    }

    let (_, action): (&Player, &CurrentAction) = query.single();

    let dig_action = match &action.interact {
        Some(InteractAction::DigAction(it)) => it,
        _ => return,
    };

    if spawns_query.is_empty() {
        return;
    }

    let mut spawns: Mut<Spawns> = spawns_query.single_mut();

    spawns.structures.push(StructureSpawn {
        position: Vec3::new(dig_action.position.x, dig_action.position.y, 1.),
        structure_key: "dug_spot",
    });
}

pub fn crop_actions(
    query: Query<(&Player, &CurrentAction, &PlayerInventory)>,
    crop_query: Query<(&Crop, &Transform)>,
    structure_query: Query<(&Structure, &Body)>,
    mut spawns_query: Query<&mut Spawns>,
) {
    if query.is_empty() {
        return;
    }

    let (_, action, inventory): (&Player, &CurrentAction, &PlayerInventory) = query.single();

    let plant_action = match &action.interact {
        Some(InteractAction::PlantCrop(it)) => it,
        _ => return,
    };

    let planting_bounds =
        BoundingBox::square(plant_action.position.x, plant_action.position.y, 60.0);

    for crop_data in crop_query.iter() {
        let (_, crop_transform): (&Crop, &Transform) = crop_data;
        let crop_bounds = BoundingBox::square(
            crop_transform.translation.x.floor(),
            crop_transform.translation.y.floor(),
            60.0,
        );

        if crop_bounds.intersects(&planting_bounds) {
            return;
        }
    }

    for structure_data in structure_query.iter() {
        let (_, body): (&Structure, &Body) = structure_data;

        if body.intersects_box(&planting_bounds) {
            return;
        }
    }

    let config = match &inventory.current_crop_config {
        Some(it) => it,
        _ => return,
    };

    if spawns_query.is_empty() {
        return;
    }

    let mut spawns = spawns_query.single_mut();

    spawns.crops.push(CropSpawn {
        config: config.clone(),
        location: Vec2::new(plant_action.position.x, plant_action.position.y),
    });
}
