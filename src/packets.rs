use serde::{Deserialize, Serialize};

use crate::globals::Avatar;

#[derive(Serialize, Deserialize, Debug)]
pub struct Damage {
    pub attacker: Avatar,
    pub damage_chunks: Vec<u32>,
    pub damage: u32,
}

// TODO: impl Serializer
#[derive(Serialize, Deserialize, Debug)]
pub struct BattleLineupPacket {
    // ID: 0
    pub id: u32,
    pub avatars: Vec<Avatar>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DamageChunkPacket {
    // ID: 1
    pub id: u32,
    pub attacker: Avatar,
    pub damage_chunk: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurnDamagePacket {
    // ID: 2
    pub id: u32,
    pub damages: Vec<Damage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvatarKillPacket {
    // ID: 3
    pub id: u32,
    pub attacker: Avatar,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BattleEndPacket {
    // ID: 0xFFFFFFFF
    pub id: u32,
}
