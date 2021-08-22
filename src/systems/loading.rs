use bevy::prelude::{Res, ResMut, State};

use crate::states::{AppState, GameLoadState};

pub fn check_load_state(mut state: ResMut<State<AppState>>, load_state: Res<GameLoadState>) {
    if load_state.texture_load_complete {
        state.set(AppState::FinishedLoading).unwrap();
    }
}

pub fn start_game(mut state: ResMut<State<AppState>>, load_state: Res<GameLoadState>) {
    if load_state.textures_set {
        state.set(AppState::InGame).unwrap();
    }
}
