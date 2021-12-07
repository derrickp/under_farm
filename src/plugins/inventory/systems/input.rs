use bevy::{
    input::Input,
    prelude::{KeyCode, Res, ResMut, State},
};

use crate::states::AppState;

pub fn open_close_inventory_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    if state.current().ne(&AppState::InGame) && state.current().ne(&AppState::InventoryScreen) {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::I) {
        if state.current().eq(&AppState::InGame) {
            state.set(AppState::InventoryScreen).unwrap();
        } else {
            state.set(AppState::InGame).unwrap();
        }
    }
}
