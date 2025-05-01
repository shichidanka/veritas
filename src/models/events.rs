
use super::misc::{Avatar, Skill};

pub enum Event {
    OnBattleBegin(OnBattleBeginEvent),
    OnSetLineup(OnSetLineupEvent),
    OnDamage(OnDamageEvent),
    OnTurnBegin(OnTurnBeginEvent),
    OnTurnEnd,
    OnKill(OnKillEvent),
    OnUseSkill(OnUseSkillEvent),
    OnBattleEnd(OnBattleEndEvent),
    OnUpdateWave(OnUpdateWaveEvent),
    OnUpdateCycle(OnUpdateCycleEvent),
}

pub struct OnBattleBeginEvent {
    pub max_waves: u32,
    pub stage_id: u32
}

pub struct OnUpdateWaveEvent {
    pub wave: u32,
    pub action_value: f64
}

pub struct OnUpdateCycleEvent {
    pub cycle: u32,
}

pub struct OnTurnBeginEvent {
    pub action_value: f64,
    pub turn_owner: Option<Avatar>
}

pub struct OnBattleEndEvent {
    pub action_value: f64
}

pub struct OnUseSkillEvent {
    pub avatar: Avatar,
    pub skill: Skill
}

pub struct OnSetLineupEvent {
    pub avatars: Vec<Avatar>,
}

pub struct OnDamageEvent {
    pub attacker: Avatar,
    pub damage: f64,
    pub damage_type: &'static str
}

pub struct OnKillEvent {
    pub attacker: Avatar,
}