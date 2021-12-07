use bevy::prelude::{Commands, Entity, Query};

use crate::components::cameras::GameCamera;

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&GameCamera, Entity) = query.single();
    commands.entity(entity).despawn();
}
