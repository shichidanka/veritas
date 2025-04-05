use crate::sr::types::rpg::gamecore::TurnBasedGameMode;

use super::misc::{Avatar, Skill};

pub enum Event {
    BattleBegin(BattleBeginEvent),
    SetBattleLineup(SetBattleLineupEvent),
    OnDamage(OnDamageEvent),
    TurnBegin,
    TurnEnd,
    OnKill(OnKillEvent),
    OnUseSkill(OnUseSkillEvent),
    BattleEnd,
}

pub struct BattleBeginEvent {
    pub turn_based_game_mode: *const TurnBasedGameMode,
}

pub struct OnUseSkillEvent {
    pub avatar: Avatar,
    pub skill: Skill
}

pub struct SetBattleLineupEvent {
    pub avatars: Vec<Avatar>,
}

pub struct OnDamageEvent {
    pub attacker: Avatar,
    pub damage: f64,
}

pub struct OnKillEvent {
    pub attacker: Avatar,
}