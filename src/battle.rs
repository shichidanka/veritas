use std::sync::{LazyLock, Mutex, MutexGuard};

use anyhow::{Context, Result};

use crate::{
    models::{
        events::{
            OnBattleEndEvent, Event, OnDamageEvent, OnKillEvent, OnUseSkillEvent,
            OnSetLineupEvent, OnTurnBeginEvent,
        },
        misc::{Avatar, TurnInfo},
        packets::{EventPacket, Packet},
    },
    server,
};

pub enum BattleState {
    Preparing,
    Started,
    Ended,
}

impl Default for BattleState {
    fn default() -> Self {
        Self::Preparing
    }
}

#[derive(Default)]
pub struct BattleContext {
    pub state: BattleState,
    pub lineup: Vec<Avatar>,
    pub turn_history: Vec<TurnInfo>,
    pub current_turn_info: TurnInfo,
    pub turn_count: usize,
    pub total_damage: f64,
    // Index w/ lineup index
    pub real_time_damages: Vec<f64>,
}

static BATTLE_CONTEXT: LazyLock<Mutex<BattleContext>> =
    LazyLock::new(|| Mutex::new(BattleContext::default()));

impl BattleContext {
    pub fn get_instance() -> MutexGuard<'static, Self> {
        BATTLE_CONTEXT.lock().unwrap()
    }

    fn find_lineup_index_by_avatar_id(
        battle_context: &MutexGuard<'static, Self>,
        avatar_id: u32,
    ) -> Option<usize> {
        let res = battle_context
            .lineup
            .iter()
            .enumerate()
            .find(|(_index, avatar)| avatar.id == avatar_id);
        res.map_or(None, |(index, _)| Some(index))
    }

    fn initialize_battle_context(
        battle_context: &mut MutexGuard<'static, Self>,
        lineup: Vec<Avatar>,
    ) {
        battle_context.current_turn_info = TurnInfo::default();
        battle_context.current_turn_info.avatars_damage = vec![0f64; lineup.len()];
        battle_context.current_turn_info.avatars_damage_chunks = vec![Vec::new(); lineup.len()];
        battle_context.turn_history = Vec::new();
        battle_context.turn_count = 0;
        battle_context.lineup = lineup.clone();
        battle_context.total_damage = 0.;
        battle_context.real_time_damages = vec![0f64; lineup.len()];
    }

    fn handle_on_battle_begin_event(
        mut _battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Battle has started");

        let packet_body = EventPacket::BattleBegin {};
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_set_lineup_event(
        e: OnSetLineupEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.state = BattleState::Started;
        Self::initialize_battle_context(&mut battle_context, e.avatars);

        for avatar in &battle_context.lineup {
            log::info!("{} was loaded in lineup", avatar);
        }

        let packet_body = EventPacket::SetBattleLineup {
            avatars: battle_context.lineup.clone(),
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_damage_event(
        e: OnDamageEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let lineup_index = Self::find_lineup_index_by_avatar_id(&battle_context, e.attacker.id)
            .with_context(|| format!("Could not find avatar {} in lineup", e.attacker))?;
        let turn = &mut battle_context.current_turn_info;
        // Record character damage chunk
        turn.avatars_damage_chunks[lineup_index].push(e.damage);

        battle_context.total_damage += e.damage as f64;
        battle_context.real_time_damages[lineup_index] += e.damage as f64;

        let packet_body = EventPacket::OnDamage {
            attacker: e.attacker,
            damage: e.damage,
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_turn_begin_event(
        e: OnTurnBeginEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let action_value = e.action_value;
        battle_context.current_turn_info.action_value = action_value;
        log::info!("AV: {:.2}", action_value);
        let packet_body = EventPacket::TurnBegin { action_value };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_turn_end_event(
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let mut turn_info = battle_context.current_turn_info.clone();

        // Calculate net damages
        let avatars_damage = turn_info
            .avatars_damage_chunks
            .iter()
            .map(|avatar_dmg_string| {
                if avatar_dmg_string.is_empty() {
                    0.0
                } else {
                    avatar_dmg_string.iter().sum()
                }
            })
            .collect::<Vec<f64>>();
        turn_info.total_damage = if avatars_damage.is_empty() {
            0.0
        } else {
            avatars_damage.iter().sum()
        };
        turn_info.avatars_damage = avatars_damage;
        battle_context.turn_history.push(turn_info.clone());

        for (i, avatar) in battle_context.lineup.iter().enumerate() {
            log::info!(
                "Turn Summary: {} has dealt {:.2} damage",
                avatar,
                turn_info.avatars_damage[i]
            )
        }

        log::info!(
            "Turn Summary: Total damage of {:.2}",
            turn_info.total_damage
        );

        let packet_body = EventPacket::TurnEnd {
            avatars: battle_context.lineup.clone(),
            avatars_damage: turn_info.avatars_damage,
            total_damage: turn_info.total_damage,
            action_value: turn_info.action_value,
        };

        // Restart turn info
        battle_context.current_turn_info = TurnInfo::default();
        battle_context.current_turn_info.avatars_damage = vec![0f64; battle_context.lineup.len()];
        battle_context.current_turn_info.avatars_damage_chunks =
            vec![Vec::new(); battle_context.lineup.len()];
        battle_context.turn_count += 1;

        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_kill_event(
        e: OnKillEvent,
        mut _battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("{} has killed", e.attacker);

        let packet_body = EventPacket::OnKill {
            attacker: e.attacker,
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_battle_end_event(
        e: OnBattleEndEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.state = BattleState::Ended;
        let packet_body = EventPacket::BattleEnd {
            avatars: battle_context.lineup.clone(),
            turn_history: battle_context.turn_history.clone(),
            turn_count: battle_context.turn_count,
            total_damage: battle_context.total_damage as f64,
            action_value: e.action_value,
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_use_skill_event(
        e: OnUseSkillEvent,
        mut _battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("{} has used {}", e.avatar, e.skill);

        let packet_body = EventPacket::OnUseSkill {
            avatar: e.avatar,
            skill: e.skill,
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    // Should wrap in option
    pub fn handle_event(event: Result<Event>) {
        let battle_context = Self::get_instance();
        let packet = match event {
            Result::Ok(event) => {
                match event {
                    Event::OnBattleBegin => Self::handle_on_battle_begin_event(battle_context),
                    Event::OnSetLineup(e) => {
                        Self::handle_on_set_lineup_event(e, battle_context)
                    }
                    Event::OnDamage(e) => Self::handle_on_damage_event(e, battle_context),
                    Event::OnTurnBegin(e) => Self::handle_on_turn_begin_event(e, battle_context),
                    Event::OnTurnEnd => Self::handle_on_turn_end_event(battle_context),
                    // Not used atm
                    Event::OnKill(e) => Self::handle_on_kill_event(e, battle_context),
                    Event::OnBattleEnd(e) => Self::handle_on_battle_end_event(e, battle_context),
                    Event::OnUseSkill(e) => Self::handle_on_use_skill_event(e, battle_context),
                }
            }
            Err(e) => {
                log::error!("Event Error: {}", e);
                Packet::from_event_packet(EventPacket::Error { msg: e.to_string() })
            }
        };

        match packet {
            Result::Ok(packet) => {
                server::broadcast(packet);
            }
            Err(e) => log::error!("Packet Error: {}", e),
        };
    }
}
