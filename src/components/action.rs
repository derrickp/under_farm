pub struct Action {
    pub interact_pressed: bool,
}

impl Default for Action {
    fn default() -> Self {
        return Action {
            interact_pressed: false,
        };
    }
}
