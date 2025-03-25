use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar {
    pub id: u32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TurnInfo {
    pub avatars_damage_chunks: Vec<Vec<u32>>,
    pub avatars_damage: Vec<u32>,
    pub total_damage: u32, // pub turn_owner: Avatar
}
