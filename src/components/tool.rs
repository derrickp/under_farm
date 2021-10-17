use super::health::Damage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    PickAxe,
    Hammer,
    Hoe,
    Shovel,
}

#[derive(Clone, Debug)]
pub struct ToolName(pub String);

#[derive(Clone, Debug)]
pub struct Tool {
    pub name: ToolName,
    pub tool_type: ToolType,
    pub damage: Option<Damage>,
}
