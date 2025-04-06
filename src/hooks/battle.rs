use crate::{
    battle::BattleContext,
    models::{
        events::{
            BattleEndEvent, Event, OnDamageEvent, OnUseSkillEvent, SetBattleLineupEvent,
            TurnBeginEvent,
        },
        misc::Avatar,
    },
    sr::{
        functions::rpg::gamecore::{
            AbilityStatic_GetActualOwner, CharacterConfig_GetSkillIndexByTriggerKey,
            SkillCharacterComponent_GetSkillData, TurnBasedAbilityComponent_GetAbilityMappedSkill,
        },
        helpers::{
            self, fixpoint_to_raw, get_avatar_from_entity, get_avatar_from_servant_entity,
            get_avatar_skill_from_skilldata, get_battle_event_skill_from_skilldata,
            get_servant_skill_from_skilldata,
        },
        il2cpp_types::Il2CppString,
        types::{
            rpg::gamecore::{
                BattleEventDataComponent, BattleLineupData, EntityType, GameEntity,
                SkillCharacterComponent, SkillData, TeamType, TurnBasedGameMode,
            },
            HBIAGLPHICO, MMNDIEBMDNL, NOPBAAAGGLA,
        },
    },
    GAMEASSEMBLY_HANDLE,
};
use anyhow::Result;
use anyhow::{anyhow, Error};
use function_name::named;
use retour::static_detour;
use std::{
    ffi::c_void,
    mem::{self},
};

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
    static MMNDIEBMDNL_FECMPGBOBOI_Detour: fn(*const MMNDIEBMDNL);
    static RPG_GameCore_SkillCharacterComponent_UseSkill_Detour: fn(*const SkillCharacterComponent, i32, *const c_void, bool, i32);
    static RPG_GameCore_AbilityStatic_ComboProcessAfterSkillUse_Detour: fn(*const GameEntity, *const SkillData);
    static RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour: fn(*const c_void, *const BattleLineupData);
    static RPG_GameCore_TurnBasedGameMode_GameModeBegin_Detour: fn(*const TurnBasedGameMode);
    static RPG_GameCore_TurnBasedGameMode_GameModeEnd_Detour: fn(*const TurnBasedGameMode);
    static RPG_GameCore_TurnBasedGameMode_DoTurnPrepareStartWork_Detour: fn(*const TurnBasedGameMode);
    static RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour: fn(*const c_void, i32) -> *const c_void;
    static RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour: fn(*const c_void, *const HBIAGLPHICO) -> bool;
}

static mut TURN_BASED_GAME_MODE_REF: Option<*const TurnBasedGameMode> = None;

fn get_elapsed_av() -> f64 {
    unsafe {
        match TURN_BASED_GAME_MODE_REF {
            Some(x) => fixpoint_to_raw(&(*x).ElapsedActionDelay__BackingField) * 10f64,
            None => panic!("There was no reference to RPG.GameCore.TurnBasedGameMode"),
        }
    }
}

// Called on any instance of damage
#[named]
fn on_damage(
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
        log::debug!(function_name!());
        let mut event: Option<Result<Event>> = None;
        match (*attacker)._Team {
            TeamType::TeamLight => {
                let damage = fixpoint_to_raw(&(*nopbaaaggla).JFKEEOMKMLI);
                let attack_owner = AbilityStatic_GetActualOwner(attacker);

                match (*attack_owner)._EntityType {
                    EntityType::Avatar => {
                        let e = match helpers::get_avatar_from_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: avatar,
                                damage,
                            })),
                            Err(e) => {
                                log::error!("Avatar Event Error: {}", e);
                                Err(anyhow!("{} Avatar Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    EntityType::Servant => {

                        let e = match helpers::get_avatar_from_servant_entity(attack_owner) {
                            Ok(avatar) => Ok(Event::OnDamage(OnDamageEvent {
                                attacker: avatar,
                                damage,
                            })),
                            Err(e) => {
                                log::error!("Servant Event Error: {}", e);
                                Err(anyhow!("{} Servant Event Error: {}", function_name!(), e))
                            }
                        };
                        event = Some(e);
                    }
                    _ => log::warn!(
                        "Light entity type {} was not matched",
                        (*attacker)._EntityType as usize
                    ),
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
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

// Called when a manual skill is used. Does not account for insert skills (out of turn automatic skills)
#[named]
fn use_skill(
    instance: *const SkillCharacterComponent,
    skill_index: i32,
    a3: *const c_void,
    a4: bool,
    skill_extra_use_param: i32,
) {
    log::debug!(function_name!());
    unsafe {
        let entity = ((*instance)._parent_class)._OwnerRef;
        let skill_owner = AbilityStatic_GetActualOwner(entity);
        let mut event: Option<Result<Event>> = None;
        match (*skill_owner)._Team {
            TeamType::TeamLight => {
                let skill_data = SkillCharacterComponent_GetSkillData(
                    instance,
                    skill_index,
                    skill_extra_use_param,
                );

                if !skill_data.is_null() {
                    match (*skill_owner)._EntityType {
                        EntityType::Avatar => {
                            let e = match get_avatar_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
                                    Err(e) => {
                                        log::error!("Avatar Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Avatar Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Avatar Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Avatar Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e)
                        }
                        EntityType::Servant => {
                            let e = match get_servant_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_servant_entity(skill_owner) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
                                    Err(e) => {
                                        log::error!("Servant Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Servant Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Servant Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Servant Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        EntityType::BattleEvent => {
                            let battle_event_data_comp =
                                (*instance)._CharacterDataRef as *const BattleEventDataComponent;
                            let avatar_entity: *const GameEntity =
                                (*battle_event_data_comp).SourceCaster__BackingField;

                            let e = match get_battle_event_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
                                    Err(e) => {
                                        log::error!("Summon Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Summon Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Summon Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Summon Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        _ => log::warn!(
                            "Light entity type {} was not matched",
                            (*skill_owner)._EntityType as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
    }

    RPG_GameCore_SkillCharacterComponent_UseSkill_Detour.call(
        instance,
        skill_index,
        a3,
        a4,
        skill_extra_use_param,
    );
}

// Insert skills are out of turn automatic skills
#[named]
fn try_insert_ability(instance: *const MMNDIEBMDNL) {
    log::debug!(function_name!());

    MMNDIEBMDNL_FECMPGBOBOI_Detour.call(instance);
    unsafe {
        // Needs to be manually updated
        let turn_based_ability_component = (*instance).FIMNOPAAFEP;
        let skill_character_component = (*instance).HECCDOHIAFD;
        //
        let entity = (*skill_character_component)._parent_class._OwnerRef;
        let skill_owner = AbilityStatic_GetActualOwner(entity);

        let mut event: Option<Result<Event>> = None;
        match (*skill_owner)._Team {
            TeamType::TeamLight => {
                let ability_name = *((instance as usize + 0x30) as *const *const Il2CppString);

                let skill_name = TurnBasedAbilityComponent_GetAbilityMappedSkill(
                    turn_based_ability_component,
                    ability_name,
                );

                let character_data_ref = (*turn_based_ability_component)._CharacterDataRef;
                let character_config = (*character_data_ref).JsonConfig__BackingField;
                let skill_index =
                    CharacterConfig_GetSkillIndexByTriggerKey(character_config, skill_name);
                let skill_data = SkillCharacterComponent_GetSkillData(
                    skill_character_component,
                    skill_index,
                    -1,
                );

                if !skill_data.is_null() {
                    match (*skill_owner)._EntityType {
                        EntityType::Avatar => {
                            let e: std::result::Result<Event, anyhow::Error> =
                                match get_avatar_skill_from_skilldata(skill_data) {
                                    Ok(skill) => match get_avatar_from_entity(skill_owner) {
                                        Ok(avatar) => {
                                            Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                        }
                                        Err(e) => {
                                            log::error!("Avatar Event Error: {}", e);
                                            Err(anyhow!(
                                                "{} Avatar Event Error: {}",
                                                function_name!(),
                                                e
                                            ))
                                        }
                                    },
                                    Err(e) => {
                                        log::error!("Avatar Combo Skill Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Avatar Combo Skill Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                };
                            event = Some(e);
                        }
                        EntityType::Servant => {
                            let e = match get_servant_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_servant_entity(skill_owner) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
                                    Err(e) => {
                                        log::error!("Servant Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Servant Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Servant Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Servant Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        EntityType::BattleEvent => {
                            let battle_event_data_comp = (*skill_character_component)
                                ._CharacterDataRef
                                as *const BattleEventDataComponent;
                            let avatar_entity: *const GameEntity =
                                (*battle_event_data_comp).SourceCaster__BackingField;

                            let e = match get_battle_event_skill_from_skilldata(skill_data) {
                                Ok(skill) => match get_avatar_from_entity(avatar_entity) {
                                    Ok(avatar) => {
                                        Ok(Event::OnUseSkill(OnUseSkillEvent { avatar, skill }))
                                    }
                                    Err(e) => {
                                        log::error!("Summon Event Error: {}", e);
                                        Err(anyhow!(
                                            "{} Summon Event Error: {}",
                                            function_name!(),
                                            e
                                        ))
                                    }
                                },
                                Err(e) => {
                                    log::error!("Summon Skill Event Error: {}", e);
                                    Err(anyhow!(
                                        "{} Summon Skill Event Error: {}",
                                        function_name!(),
                                        e
                                    ))
                                }
                            };
                            event = Some(e);
                        }
                        _ => log::warn!(
                            "Light entity type {} was not matched",
                            (*skill_owner)._EntityType as usize
                        ),
                    }
                }
            }
            _ => {}
        }
        if let Some(event) = event {
            BattleContext::handle_event(event);
        }
    }
}

#[named]
fn set_battle_lineup_data(instance: *const c_void, battle_lineup_data: *const BattleLineupData) {
    log::debug!(function_name!());
    unsafe {
        let light_team = (*battle_lineup_data).LightTeam;
        let mut avatars = Vec::<Avatar>::new();
        let mut errors = Vec::<Error>::new();
        for character_ptr in (*light_team).to_slice() {
            let character = *character_ptr;
            let avatar_id = (*character).CharacterID;
            match helpers::get_avatar_from_id(avatar_id) {
                Ok(avatar) => avatars.push(avatar),
                Err(e) => {
                    log::error!("BattleLineup Error: {}", e);
                    errors.push(e);
                }
            }
        }
        let event = if !errors.is_empty() {
            let errors = errors
                .iter()
                .map(|e| format!("{}\n", e.to_string()))
                .collect::<String>();
            Err(anyhow!(errors))
        } else {
            Ok(Event::SetBattleLineup(SetBattleLineupEvent { avatars }))
        };
        BattleContext::handle_event(event);
    }
    RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour.call(instance, battle_lineup_data);
}

#[named]
fn game_mode_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    unsafe {
        RPG_GameCore_TurnBasedGameMode_GameModeBegin_Detour.call(instance);
        TURN_BASED_GAME_MODE_REF = Some(instance);
        BattleContext::handle_event(Ok(Event::BattleBegin));
    }
}

#[named]
fn game_mode_end(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    unsafe {
        RPG_GameCore_TurnBasedGameMode_GameModeEnd_Detour.call(instance);
        BattleContext::handle_event(Ok(Event::BattleEnd(BattleEndEvent {
            action_value: get_elapsed_av(),
        })));
        TURN_BASED_GAME_MODE_REF = None;
    }
}

#[named]
fn turn_begin(instance: *const TurnBasedGameMode) {
    log::debug!(function_name!());
    // Update AV first
    RPG_GameCore_TurnBasedGameMode_DoTurnPrepareStartWork_Detour.call(instance);
    BattleContext::handle_event(Ok(Event::TurnBegin(TurnBeginEvent {
        action_value: get_elapsed_av(),
    })));
}

#[named]
fn turn_end(instance: *const c_void, a1: i32) -> *const c_void {
    log::debug!(function_name!());
    // Can match player v enemy turn w/
    // RPG.GameCore.TurnBasedGameMode.GetCurrentTurnTeam
    let res = RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour
        .call(instance, a1);
    BattleContext::handle_event(Ok(Event::TurnEnd));
    return res;
}

// #[named]
// fn _MakeLimboEntityDie(instance: *const c_void, a1: *const HBIAGLPHICO) -> bool {
//     log::debug!(function_name!());
//     // This isn't general kills
//     unsafe {
//         let attacker = (*a1).JKCOIOLCMEP;
//         match (*attacker)._Team {
//             TeamType::TeamLight => match (*attacker)._EntityType {
//                 EntityType::Avatar => {
//                     match get_avatar_from_entity(attacker) {
//                         Ok(avatar) => {}
//                         Err(_) => todo!(),
//                     }
//                     let avatar_id = UIGameEntityUtils_GetAvatarID((*a1).JKCOIOLCMEP);
//                     let avatar_data = helpers::get_avatar_data_by_id(avatar_id);
//                     let avatar_name = (*AvatarData_get_AvatarName(avatar_data))
//                         .to_string()
//                         .unwrap();
//                     BattleContext::handle_event(Event::OnKill(OnKillEvent {
//                         attacker: Avatar {
//                             id: avatar_id,
//                             name: avatar_name,
//                         },
//                     }));
//                 }
//                 _ => {}
//             },
//             _ => {}
//         }
//     }
//     return RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour.call(instance, a1);
// }

pub fn install_hooks() -> Result<()> {
    unsafe {
        hook_function!(
            DMFMLMJKKHB_OMPLOLLELLK_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x75d1360),
            on_damage
        );
        hook_function!(
            RPG_GameCore_SkillCharacterComponent_UseSkill_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x8f21e80),
            use_skill
        );
        hook_function!(
            MMNDIEBMDNL_FECMPGBOBOI_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x7781fd0),
            try_insert_ability
        );
        hook_function!(
            RPG_Client_BattleAssetPreload_SetBattleLineupData_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x762dba0),
            set_battle_lineup_data
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode_GameModeBegin_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943eab0),
            game_mode_begin
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode_GameModeEnd_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943ebd0),
            game_mode_end
        );
        hook_function!(
            RPG_GameCore_TurnBasedGameMode_DoTurnPrepareStartWork_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x94392d0),
            turn_begin
        );
        hook_function!(
            RPG_GameCore_TurnBasedAbilityComponent_ProcessOnLevelTurnActionEndEvent_Detour,
            mem::transmute(*GAMEASSEMBLY_HANDLE + 0x9400f10),
            turn_end
        );

        // This is not good
        // hook_function!(
        //     RPG_GameCore_TurnBasedGameMode__MakeLimboEntityDie_Detour,
        //     mem::transmute(*GAMEASSEMBLY_HANDLE + 0x943d6c0),
        //     _MakeLimboEntityDie
        // );
        Ok(())
    }
}