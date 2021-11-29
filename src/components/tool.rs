use super::{damage::Damage, name::Name};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    PickAxe,
    Hammer,
    Hoe,
    Shovel,
}

impl From<&str> for ToolType {
    fn from(tool_type: &str) -> Self {
        match tool_type {
            "pickaxe" => ToolType::PickAxe,
            "hammer" => ToolType::Hammer,
            "hoe" => ToolType::Hoe,
            "shovel" => ToolType::Shovel,
            _ => ToolType::PickAxe,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tool {
    pub name: Name,
    pub key: String,
    pub tool_type: ToolType,
    pub damage: Option<Damage>,
}
