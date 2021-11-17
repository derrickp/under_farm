use super::{
    crops::CropsConfig, floors::FloorsConfig, load::Load, player::PlayerConfig,
    structures::StructuresConfig,
};

pub struct GameConfiguration {
    pub crops_config: CropsConfig,
    pub floors_config: FloorsConfig,
    pub structures_config: StructuresConfig,
    pub player_config: PlayerConfig,
}

impl Load for GameConfiguration {
    fn load(path: &str) -> Self {
        let crops_config_path = format!("{}/crops.kdl", path);
        let floors_config_path = format!("{}/floors.kdl", path);
        let structures_config_path = format!("{}/structures.kdl", path);
        let player_config_path = format!("{}/player.kdl", path);

        let crops_config = CropsConfig::load(&crops_config_path);
        let floors_config = FloorsConfig::load(&floors_config_path);
        let structures_config = StructuresConfig::load(&structures_config_path);
        let player_config = PlayerConfig::load(&player_config_path);

        Self {
            crops_config,
            floors_config,
            structures_config,
            player_config,
        }
    }
}
