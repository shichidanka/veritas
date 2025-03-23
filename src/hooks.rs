#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::sr::{
    functions::rpg::client::{AvatarData_get_AvatarName, AvatarModule_GetAvatar},
    globals::GlobalVars,
    types::RPG::Client::ModuleManager,
};
use crate::{
    globals::{
        Avatar, AvatarTurnDamage, BattleLineupTurnDamage, BATTLE_LINEUP_TURN_DAMAGE,
        GAMEASSEMBLY_HANDLE, SOCKETS,
    },
    packets::{
        AvatarKillPacket, BattleEndPacket, BattleLineupPacket, Damage, DamageChunkPacket,
        TurnDamagePacket,
    },
    sr::{
        functions::rpg::{
            client::UIGameEntityUtils_GetAvatarID,
            gamecore::{
                AbilityStatic_GetActualOwner, EntityManager__GetEntitySummoner,
                GamePlayStatic_GetEntityManager,
            },
        },
        types::{
            HBIAGLPHICO, NOPBAAAGGLA,
            RPG::GameCore::{BattleLineupData, EntityType, GameEntity, TeamType},
        },
    },
};
use base64::{prelude::BASE64_STANDARD, Engine};
use retour::static_detour;
use std::{error, ffi::c_void, mem};

static_detour! {
    static DMFMLMJKKHB_OMPLOLLELLK_Detour: fn(
        *const c_void,
        *const c_void,
        *const NOPBAAAGGLA,
        *const c_void,
        *const c_void,
        *const GameEntity,
        *const GameEntity,
        *const GameEntity,
        bool,
        *const c_void
    ) -> bool;

    static RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour: fn(*const c_void);
    static RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour: fn(*const c_void, *const BattleLineupData);
    static RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour: fn(*const c_void, *const HBIAGLPHICO) -> bool;
    static RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour: fn(*const c_void, i32) -> *const c_void;
}

pub fn game_mode_end(instance: *const c_void) {
    let mut battle_lineup = BATTLE_LINEUP_TURN_DAMAGE.get().unwrap().write().unwrap();
    *battle_lineup = BattleLineupTurnDamage::default();

    let battle_end_packet = BattleEndPacket { id: 0xFFFFFFFF };

    let mut buf: String = String::new();
    BASE64_STANDARD.encode_string(serde_json::to_string(&battle_end_packet).unwrap(), &mut buf);
    let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
    sockets.broadcast(unsafe { buf.as_mut_vec().as_slice() });

    RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour.call(instance);
}

pub fn set_battle_lineup_data(
    instance: *const c_void,
    battle_lineup_data: *const BattleLineupData,
) {
    unsafe {
        let s_module_manager =
            GlobalVars::get_global_var(GlobalVars::s_ModuleManager) as *const ModuleManager;
        let avatar_module = (*s_module_manager).AvatarModule;

        let light_team = (*battle_lineup_data).LightTeam;
        let mut avatars = Vec::<Avatar>::new();
        for (i, character_ptr) in (*light_team).to_slice().iter().enumerate() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            let avatar_data = AvatarModule_GetAvatar(avatar_module, avatar_id);
            let avatar_name = (*AvatarData_get_AvatarName(avatar_data))
                .to_string()
                .unwrap();

            let mut battle_lineup = BATTLE_LINEUP_TURN_DAMAGE.get().unwrap().write().unwrap();
            // implement constructor
            battle_lineup.lineup[i as usize] = AvatarTurnDamage::default();
            battle_lineup.lineup[i as usize].avatar.avatar_id = avatar_id;
            battle_lineup.lineup[i as usize].avatar.avatar_name = avatar_name.clone();
            avatars.push(Avatar {
                avatar_id,
                avatar_name: avatar_name.clone(),
            });

            println!(
                "[VERITAS] ({}: {}) was loaded in lineup",
                avatar_id, avatar_name
            );
        }

        let battle_lineup_packet = BattleLineupPacket { id: 0, avatars };
        let mut buf: String = String::new();
        BASE64_STANDARD.encode_string(serde_json::to_string(&battle_lineup_packet).unwrap(), &mut buf);
        let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
        sockets.broadcast(buf.as_mut_vec().as_slice());
    }
    RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour.call(instance, battle_lineup_data);
}

pub fn damage_chunk(
    task_context: *const c_void,
    damage_by_attack_property: *const c_void,
    nopbaaaggla: *const NOPBAAAGGLA,
    turn_based_ability_component_1: *const c_void,
    turn_based_ability_component_2: *const c_void,
    attacker: *const GameEntity,
    defender: *const GameEntity,
    attacker_task_single_target: *const GameEntity,
    flag: bool,
    obkbghmgbne: *const c_void,
) -> bool {
    unsafe {
        match (*attacker)._Team {
            // Light
            TeamType::TeamLight => {
                // Unsure of the relevance of the last 32 bits; not null
                // Might contain the decimal part of DMG?
                let damage = (((*nopbaaaggla).JFKEEOMKMLI.m_rawValue as u64 & 0xFFFFFFFF00000000)
                    >> 32) as u32;
                let attack_owner = AbilityStatic_GetActualOwner(attacker);

                match (*attack_owner)._EntityType {
                    EntityType::Avatar => {
                        let avatar_id = UIGameEntityUtils_GetAvatarID(attack_owner);
                        let s_module_manager =
                            GlobalVars::get_global_var(GlobalVars::s_ModuleManager)
                                as *const ModuleManager;
                        let avatar_module = (*s_module_manager).AvatarModule;

                        let avatar_data = AvatarModule_GetAvatar(avatar_module, avatar_id);
                        let avatar_name = (*AvatarData_get_AvatarName(avatar_data))
                            .to_string()
                            .unwrap();

                        let mut battle_lineup =
                            BATTLE_LINEUP_TURN_DAMAGE.get().unwrap().write().unwrap();
                        let index = battle_lineup.find_avatar_index_by_id(avatar_id);
                        battle_lineup.lineup[index].damage_chunks.push(damage);

                        let packet = DamageChunkPacket {
                            id: 1,
                            attacker: Avatar {
                                avatar_id,
                                avatar_name: avatar_name.clone(),
                            },
                            damage_chunk: damage,
                        };


                        let mut buf: String = String::new();
                        BASE64_STANDARD.encode_string(serde_json::to_string(&packet).unwrap(), &mut buf);
                        let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
                        sockets.broadcast(buf.as_mut_vec().as_slice());
                
                        println!(
                            "[VERITAS] ({}: {}) dealt {} damage",
                            avatar_id, avatar_name, damage
                        );
                    }
                    EntityType::Servant => {
                        let entity_manager = GamePlayStatic_GetEntityManager();
                        let avatar_entity =
                            EntityManager__GetEntitySummoner(entity_manager, attack_owner);

                        let avatar_id = UIGameEntityUtils_GetAvatarID(avatar_entity);
                        let s_module_manager =
                            GlobalVars::get_global_var(GlobalVars::s_ModuleManager)
                                as *const ModuleManager;
                        let avatar_module = (*s_module_manager).AvatarModule;

                        let avatar_data = AvatarModule_GetAvatar(avatar_module, avatar_id);
                        let avatar_name = (*AvatarData_get_AvatarName(avatar_data))
                            .to_string()
                            .unwrap();

                        let mut battle_lineup =
                            BATTLE_LINEUP_TURN_DAMAGE.get().unwrap().write().unwrap();
                        let index = battle_lineup.find_avatar_index_by_id(avatar_id);
                        battle_lineup.lineup[index].damage_chunks.push(damage);

                        let packet = DamageChunkPacket {
                            id: 1,
                            attacker: Avatar {
                                avatar_id,
                                avatar_name: avatar_name.clone(),
                            },
                            damage_chunk: damage,
                        };

                        let mut buf: String = String::new();
                        BASE64_STANDARD
                            .encode_string(serde_json::to_string(&packet).unwrap(), &mut buf);

                        let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
                        sockets.broadcast(buf.as_mut_vec().as_slice());

                        println!(
                            "[VERITAS] ({}: {})'s servant dealt {} damage",
                            avatar_id, avatar_name, damage
                        );
                    }
                    _ => (),
                }
            }
            _ => {}
        }

        return DMFMLMJKKHB_OMPLOLLELLK_Detour.call(
            task_context,
            damage_by_attack_property,
            nopbaaaggla,
            turn_based_ability_component_1,
            turn_based_ability_component_2,
            attacker,
            defender,
            attacker_task_single_target,
            flag,
            obkbghmgbne,
        );
    }
}

pub fn ProcessOnLevelTurnActionEndEvent(instance: *const c_void, a1: i32) -> *const c_void {
    unsafe {
        // Can match player v enemy turn w/
        // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam

        let mut battle_lineup = BATTLE_LINEUP_TURN_DAMAGE.get().unwrap().write().unwrap();

        let mut damages = Vec::new();
        for item in &mut (*battle_lineup).lineup {
            damages.push(Damage {
                attacker: item.avatar.clone(),
                damage_chunks: item.damage_chunks.clone(),
                damage: item.damage_chunks.iter().sum(),
            });
            item.damage_chunks = Vec::new();
        }

        let turn_damage_packet = TurnDamagePacket { id: 2, damages };

        let mut buf: String = String::new();
        BASE64_STANDARD.encode_string(serde_json::to_string(&turn_damage_packet).unwrap(), &mut buf);
        let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
        sockets.broadcast(buf.as_mut_vec().as_slice());
    }
    return RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour
        .call(instance, a1);
}

pub fn _MakeLimboEntityDie(instance: *const c_void, a1: *const HBIAGLPHICO) -> bool {
    // Only passes the attacker ref once even if multiple kills in one attack
    unsafe {
        let attacker = (*a1).JKCOIOLCMEP;
        match (*attacker)._Team {
            TeamType::TeamLight => match (*attacker)._EntityType {
                EntityType::Avatar => {
                    let avatar_id = UIGameEntityUtils_GetAvatarID((*a1).JKCOIOLCMEP);
                    let s_module_manager = GlobalVars::get_global_var(GlobalVars::s_ModuleManager)
                        as *const ModuleManager;
                    let avatar_module = (*s_module_manager).AvatarModule;

                    let avatar_data = AvatarModule_GetAvatar(avatar_module, avatar_id);
                    let avatar_name = (*AvatarData_get_AvatarName(avatar_data))
                        .to_string()
                        .unwrap();

                    let packet = AvatarKillPacket {
                        id: 3,
                        attacker: Avatar {
                            avatar_id,
                            avatar_name: avatar_name.clone(),
                        },
                    };

                    let mut buf: String = String::new();
                    BASE64_STANDARD
                        .encode_string(serde_json::to_string(&packet).unwrap(), &mut buf);

                    let mut sockets = SOCKETS.get().unwrap().lock().unwrap();
                    sockets.broadcast(buf.as_mut_vec().as_slice());
                }
                _ => {}
            },
            _ => {}
        }
    }
    return RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour.call(instance, a1);
}

macro_rules! hook_function {
    (
        $detour:ident,
        $target:expr,
        $reroute:ident
    ) => {
        $detour.initialize($target, $reroute)?;
        $detour.enable()?;
    };
}

pub fn install_hooks() -> Result<(), Box<dyn error::Error>> {
    unsafe {
        hook_function!(
            DMFMLMJKKHB_OMPLOLLELLK_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x75d1360),
            damage_chunk
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode__GameModeEnd_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943ebd0),
            game_mode_end
        );
        hook_function!(
            RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x762dba0),
            set_battle_lineup_data
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943d6c0),
            _MakeLimboEntityDie
        );
        hook_function!(
            RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x9400f10),
            ProcessOnLevelTurnActionEndEvent
        );

        Ok(())
    }
}
