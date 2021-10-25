use super::{damage::Damage, name::Name};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    PickAxe,
    Hammer,
    Hoe,
    Shovel,
}

#[derive(Clone, Debug)]
pub struct Tool {
    pub name: Name,
    pub tool_type: ToolType,
    pub damage: Option<Damage>,
}
