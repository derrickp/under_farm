use std::fs;
use walkdir::WalkDir;

use bevy::prelude::{Commands, Res, ResMut, State};

use crate::{
    states::{AppState, GameLoadState},
    world_generation::templates::RoomTemplates,
};

pub fn load_templates(mut commands: Commands, mut load_state: ResMut<GameLoadState>) {
    if load_state.room_templates_loaded {
        return;
    }

    let mut templates: Vec<String> = Vec::new();
    for dir_entry in WalkDir::new("assets/room_templates") {
        if let Ok(entry) = dir_entry {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                templates.push(content);
            }
        }
    }

    println!("{}", templates.clone().len());

    let room_templates = RoomTemplates {
        all_templates: templates,
    };

    commands.insert_resource(room_templates);
    load_state.room_templates_loaded = true;
}

pub fn check_load_state(mut state: ResMut<State<AppState>>, load_state: Res<GameLoadState>) {
    if load_state.texture_load_complete && load_state.room_templates_loaded {
        state.set(AppState::FinishedLoading).unwrap();
    }
}

pub fn start_game(mut state: ResMut<State<AppState>>, load_state: Res<GameLoadState>) {
    if load_state.textures_set && load_state.game_world_generated {
        state.set(AppState::InGame).unwrap();
    }
}
