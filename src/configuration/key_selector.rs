use bevy::prelude::KeyCode;

#[derive(Clone)]
pub struct KeySelector {
    pub key_code: KeyCode,
    pub display_code: String,
}
