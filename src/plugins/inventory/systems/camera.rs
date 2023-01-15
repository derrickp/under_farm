use bevy::prelude::{Camera2dBundle, Commands, Entity, Mut, Query, Visibility};

use crate::components::{
    body::Body,
    cameras::{GameCamera, UiCamera},
};

pub fn remove_gameplay_camera(mut commands: Commands, query: Query<(&GameCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&GameCamera, Entity) = query.single();
    commands.entity(entity).despawn();

    commands.spawn(Camera2dBundle::default()).insert(UiCamera);
}

pub fn remove_ui_camera(mut commands: Commands, query: Query<(&UiCamera, Entity)>) {
    if query.is_empty() {
        return;
    }

    let (_, entity): (&UiCamera, Entity) = query.single();
    commands.entity(entity).despawn();
}

pub fn hide_game_sprites(mut query: Query<(&mut Body, &mut Visibility)>) {
    for entry in query.iter_mut() {
        let (mut body, mut visibility): (Mut<Body>, Mut<Visibility>) = entry;

        body.visibility_before_inventory = visibility.is_visible;
        visibility.is_visible = false;
    }
}

pub fn show_game_sprites(mut query: Query<(&mut Body, &mut Visibility)>) {
    for entry in query.iter_mut() {
        let (mut body, mut visibility): (Mut<Body>, Mut<Visibility>) = entry;

        visibility.is_visible = body.visibility_before_inventory;
        body.visibility_before_inventory = false;
    }
}
