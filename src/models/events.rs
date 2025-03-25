use serde::{Deserialize, Serialize};

use super::misc::Avatar;

#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    SetBattleLineup(SetBattleLineupEvent),
    OnDamage(OnDamageEvent),
    TurnEnd,
    OnKill(OnKillEvent),
    BattleEnd,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetBattleLineupEvent {
    pub avatars: Vec<Avatar>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnDamageEvent {
    pub attacker: Avatar,
    pub damage: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnKillEvent {
    pub attacker: Avatar,
}