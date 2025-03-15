use pumpkin_util::permission::PermissionLvl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Op {
    pub uuid: Uuid,
    pub name: String,
    pub level: PermissionLvl,
    pub bypasses_player_limit: bool,
}

impl Op {
    pub fn new(
        uuid: Uuid,
        name: String,
        level: PermissionLvl,
        bypasses_player_limit: bool,
    ) -> Self {
        Self {
            uuid,
            name,
            level,
            bypasses_player_limit,
        }
    }
}
