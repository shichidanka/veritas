use std::sync::{LazyLock, Mutex, MutexGuard};

use anyhow::{Context, Result};

use crate::{
    models::{
        events::{
            Event, OnBattleBeginEvent, OnBattleEndEvent, OnDamageEvent, OnKillEvent, OnSetLineupEvent, OnTurnBeginEvent, OnUpdateCycleEvent, OnUpdateWaveEvent, OnUseSkillEvent
        },
        misc::{Avatar, TurnInfo},
        packets::{EventPacket, Packet},
    },
    server,
};

#[derive(Default)]
pub enum BattleState {
    #[default]
    Preparing,
    Started,
    Ended,
}

#[derive(Default)]
pub struct BattleContext {
    pub state: BattleState,
    pub lineup: Vec<Avatar>,
    pub turn_history: Vec<TurnInfo>,
    pub av_history: Vec<TurnInfo>,
    pub last_wave_action_value: f64,
    pub total_elapsed_action_value: f64,
    // pub wave_history: WaveInfo,
    // pub cycle_history: CycleInfo,
    pub current_turn_info: TurnInfo,
    pub turn_count: usize,
    pub total_damage: f64,
    // Index w/ lineup index
    // Used to update UI damage when dmg occurs
    pub real_time_damages: Vec<f64>,
    pub max_waves: u32,
    pub wave: u32,
    pub cycle: u32,

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


    // A word of caution:
    // The lineup is setup first 
    fn handle_on_battle_begin_event(
        e: OnBattleBeginEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        *battle_context = BattleContext {
            state: BattleState::Started,
            max_waves: e.max_waves,
            current_turn_info: battle_context.current_turn_info.clone(),
            lineup: battle_context.lineup.clone(),
            real_time_damages: battle_context.real_time_damages.clone(),
            ..Default::default()
        };
        log::info!("Battle has started");
        log::info!("Max Waves: {}", e.max_waves);
        battle_context.max_waves = e.max_waves;

        let packet_body = EventPacket::OnBattleBegin { max_waves: e.max_waves };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_set_lineup_event(
        e: OnSetLineupEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        battle_context.current_turn_info.avatars_turn_damage = vec![0f64; e.avatars.len()];
        battle_context.lineup = e.avatars.clone();
        battle_context.real_time_damages = vec![0f64; e.avatars.len()];

        for avatar in &battle_context.lineup {
            log::info!("{} was loaded in lineup", avatar);
        }

        let packet_body = EventPacket::OnSetBattleLineup {
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
        turn.avatars_turn_damage[lineup_index] += e.damage;

        battle_context.total_damage += e.damage as f64;
        battle_context.real_time_damages[lineup_index] += e.damage as f64;

        let packet_body = EventPacket::OnDamage {
            attacker: e.attacker,
            damage: e.damage,
            damage_type: e.damage_type
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_turn_begin_event(
        e: OnTurnBeginEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let cur_action_value = e.total_elapsed_action_value - battle_context.last_wave_action_value;
        battle_context.total_elapsed_action_value = e.total_elapsed_action_value;
        battle_context.current_turn_info.action_value = cur_action_value;
        log::info!("AV: {:.2}", cur_action_value);
        let packet_body = EventPacket::OnTurnBegin {
            action_value: cur_action_value,
            turn_owner: e.turn_owner
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_turn_end_event(
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        let mut turn_info = battle_context.current_turn_info.clone();

        // Calculate net damages
        turn_info.total_damage = if turn_info.avatars_turn_damage.is_empty() {
            0.0
        } else {
            turn_info.avatars_turn_damage.iter().sum()
        };
        battle_context.turn_history.push(turn_info.clone());

        if let Some(last_turn) = battle_context.av_history.last_mut() {
            // If same AV, update damage
            if last_turn.action_value == turn_info.action_value {
                for (i, incoming_dmg) in turn_info.avatars_turn_damage.iter().enumerate() {
                   last_turn.avatars_turn_damage[i] += incoming_dmg;
                }
            }
            else {
                battle_context.av_history.push(turn_info.clone());
            }
        }
        else {
            battle_context.av_history.push(turn_info.clone());
        }

        for (i, avatar) in battle_context.lineup.iter().enumerate() {
            if turn_info.avatars_turn_damage[i] > 0.0 {
                log::info!(
                    "Turn Summary: {} has dealt {:.2} damage",
                    avatar,
                    turn_info.avatars_turn_damage[i]
                );
            }
        }

        if turn_info.total_damage > 0.0 {
            log::info!(
                "Turn Summary: Total damage of {:.2}",
                turn_info.total_damage
            );
        }

        let packet_body = EventPacket::OnTurnEnd {
            avatars: battle_context.lineup.clone(),
            avatars_damage: turn_info.avatars_turn_damage.clone(),
            total_damage: turn_info.total_damage,
            action_value: turn_info.action_value,
        };

        // Restart turn info
        battle_context.current_turn_info.total_damage = 0.0;
        battle_context.current_turn_info.avatars_turn_damage = vec![0f64; battle_context.lineup.len()];
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

        let total_elapsed_action_value = e.total_elapsed_action_value;
        let cur_action_value = battle_context.total_elapsed_action_value - battle_context.last_wave_action_value;
        battle_context.total_elapsed_action_value = total_elapsed_action_value;
        battle_context.current_turn_info.action_value = cur_action_value;

        let packet_body = EventPacket::OnBattleEnd {
            avatars: battle_context.lineup.clone(),
            turn_history: battle_context.turn_history.clone(),
            av_history: battle_context.av_history.clone(),
            turn_count: battle_context.turn_count,
            total_damage: battle_context.total_damage as f64,
            total_elapsed_action_value: e.total_elapsed_action_value,
            action_value: cur_action_value
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

    fn handle_on_update_wave_event(
        e: OnUpdateWaveEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Wave: {}", e.wave);

        battle_context.wave = e.wave;
        battle_context.last_wave_action_value = battle_context.current_turn_info.action_value;
        battle_context.current_turn_info.action_value = 0.;

        let packet_body = EventPacket::OnUpdateWave { 
            wave: e.wave
        };
        Packet::from_event_packet(packet_body.clone())
            .with_context(|| format!("Failed to create {}", packet_body.name()))
    }

    fn handle_on_update_cycle_event(
        e: OnUpdateCycleEvent,
        mut battle_context: MutexGuard<'static, BattleContext>,
    ) -> Result<Packet> {
        log::info!("Cycle: {}", e.cycle);

        battle_context.cycle = e.cycle;
        let packet_body = EventPacket::OnUpdateCycle {
            cycle: e.cycle
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
                    Event::OnBattleBegin(e) => Self::handle_on_battle_begin_event(e, battle_context),
                    Event::OnSetLineup(e) => Self::handle_on_set_lineup_event(e, battle_context),
                    Event::OnDamage(e) => Self::handle_on_damage_event(e, battle_context),
                    Event::OnTurnBegin(e) => Self::handle_on_turn_begin_event(e, battle_context),
                    Event::OnTurnEnd => Self::handle_on_turn_end_event(battle_context),
                    Event::OnKill(e) => Self::handle_on_kill_event(e, battle_context),
                    Event::OnBattleEnd(e) => Self::handle_on_battle_end_event(e, battle_context),
                    Event::OnUseSkill(e) => Self::handle_on_use_skill_event(e, battle_context),
                    Event::OnUpdateWave(e) => Self::handle_on_update_wave_event(e, battle_context),
                    Event::OnUpdateCycle(e) => {
                        if e.cycle == battle_context.cycle {
                            return;
                        }
                        Self::handle_on_update_cycle_event(e, battle_context)
                    },
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
