use bevy::prelude::{Camera2dBundle, Commands, Entity, Query};

use crate::components::cameras::{GameCamera, UiCamera};

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&GameCamera, Entity) = query.single();
    commands.entity(entity).despawn();

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(UiCamera);
}

pub fn remove_ui_camera(mut commands: Commands, query: Query<(&UiCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&UiCamera, Entity) = query.single();
    commands.entity(entity).despawn();
}
