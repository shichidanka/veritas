
use super::misc::{Avatar, Skill};

pub enum Event {
    BattleBegin,
    SetBattleLineup(SetBattleLineupEvent),
    OnDamage(OnDamageEvent),
    TurnBegin(TurnBeginEvent),
    TurnEnd,
    OnKill(OnKillEvent),
    OnUseSkill(OnUseSkillEvent),
    BattleEnd(BattleEndEvent),
}

pub struct TurnBeginEvent {
    pub action_value: f64
}

pub struct BattleEndEvent {
    pub action_value: f64
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