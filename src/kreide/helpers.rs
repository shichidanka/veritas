use std::{ffi::c_void, mem, ptr::null, sync::LazyLock};

use super::{
    functions::rpg::{
        client::{
            AvatarData_get_AvatarName, AvatarModule_GetAvatar, TextmapStatic_GetText,
            UIGameEntityUtils_GetAvatarID,
        },
        gamecore::{
            AvatarSkillRowData_get_AttackType, AvatarSkillRowData_get_SkillName,
            BattleEventSkillRowData_get_AttackType, BattleEventSkillRowData_get_SkillName,
            EntityManager__GetEntitySummoner, GamePlayStatic_GetEntityManager,
            ServantSkillRowData_get_AttackType, ServantSkillRowData_get_SkillName,
        },
    },
    statics::MODULEMANAGER_FIELD_OFFSET,
    types::rpg::{
        client::{AvatarData, ModuleManager, TextID},
        gamecore::{AttackType, FixPoint, GameEntity, SkillData},
    },
};
use crate::{
    kreide::statics::MODULES_PTR_OFFSET,
    models::misc::{Avatar, Skill},
};
use anyhow::{Ok, Result, anyhow};
use function_name::named;

#[named]
pub unsafe fn get_module_manager() -> *const c_void {
    log::debug!(function_name!());
    unsafe {
        let modules_ptr = *(*MODULES_PTR_OFFSET as *const *const c_void);
        *(modules_ptr.byte_offset(MODULEMANAGER_FIELD_OFFSET as _) as *const *const c_void)
    }
}

#[named]
unsafe fn get_avatar_data_from_id(avatar_id: u32) -> *const AvatarData {
    log::debug!(function_name!());
    unsafe {
        let s_module_manager = get_module_manager() as *const ModuleManager;
        let avatar_module = (*s_module_manager).AvatarModule;
        AvatarModule_GetAvatar(avatar_module, avatar_id)
    }
}

#[named]
pub unsafe fn get_avatar_from_id(avatar_id: u32) -> Result<Avatar> {
    log::debug!(function_name!());
    unsafe {
        let avatar_data = get_avatar_data_from_id(avatar_id);
        if !avatar_data.is_null() {
            let avatar_name = AvatarData_get_AvatarName(avatar_data);
            if !avatar_name.is_null() {
                Ok(Avatar {
                    id: avatar_id,
                    name: (*avatar_name).to_string(),
                })
            } else {
                Err(anyhow!("AvatarData {} name was null", avatar_id))
            }
        } else {
            Err(anyhow!("AvatarData {} was null", avatar_id))
        }
    }
}

#[named]
pub unsafe fn get_avatar_skill_from_skilldata(skill_data: *const SkillData) -> Result<Skill> {
    log::debug!(function_name!());
    unsafe {
        let get_skill_name_callback: unsafe fn(_, _) -> _ = AvatarSkillRowData_get_SkillName;
        let get_skill_type_callback: unsafe fn(_) -> _ = AvatarSkillRowData_get_AttackType;

        get_skill_from_skilldata(
            skill_data,
            mem::transmute(get_skill_name_callback),
            mem::transmute(get_skill_type_callback),
        )
    }
}

#[named]
pub unsafe fn get_servant_skill_from_skilldata(skill_data: *const SkillData) -> Result<Skill> {
    log::debug!(function_name!());
    unsafe {
        let get_skill_name_callback: unsafe fn(_, _) -> _ = ServantSkillRowData_get_SkillName;
        let get_skill_type_callback: unsafe fn(_) -> _ = ServantSkillRowData_get_AttackType;

        get_skill_from_skilldata(
            skill_data,
            mem::transmute(get_skill_name_callback),
            mem::transmute(get_skill_type_callback),
        )
    }
}

#[named]
pub unsafe fn get_battle_event_skill_from_skilldata(skill_data: *const SkillData) -> Result<Skill> {
    log::debug!(function_name!());
    unsafe {
        let get_skill_name_callback: unsafe fn(_, _) -> _ = BattleEventSkillRowData_get_SkillName;
        let get_skill_type_callback: unsafe fn(_) -> _ = BattleEventSkillRowData_get_AttackType;

        get_skill_from_skilldata(
            skill_data,
            mem::transmute(get_skill_name_callback),
            mem::transmute(get_skill_type_callback),
        )
    }
}

#[named]
pub unsafe fn get_skill_from_skilldata(
    skill_data: *const SkillData,
    get_skill_name_callback: fn(*mut TextID, *const c_void) -> *const TextID,
    get_skill_type_callback: fn(*const c_void) -> AttackType,
) -> Result<Skill> {
    log::debug!(function_name!());
    unsafe {
        if !skill_data.is_null() {
            let row_data = (*skill_data).RowData as *const c_void;
            if !row_data.is_null() {
                let mut text_id: TextID = mem::zeroed::<TextID>();
                get_skill_name_callback(&mut text_id, row_data);
                let skill_name = TextmapStatic_GetText(&text_id, null());

                let skill_type = get_skill_type_callback(row_data);

                if !skill_name.is_null() {
                    Ok(Skill {
                        name: (*skill_name).to_string(),
                        skill_type: skill_type as isize,
                    })
                } else {
                    Err(anyhow!("SkillData type was null"))
                }
            } else {
                Err(anyhow!("SkillData RowData was null"))
            }
        } else {
            Err(anyhow!("SkillData was null"))
        }
    }
}

#[named]
pub unsafe fn get_avatar_from_entity(entity: *const GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    if !entity.is_null() {
        unsafe {
            let id = UIGameEntityUtils_GetAvatarID(entity);
            let avatar_data = get_avatar_data_from_id(id);
            if !avatar_data.is_null() {
                let name = AvatarData_get_AvatarName(avatar_data);
                if !name.is_null() {
                    Ok(Avatar {
                        id,
                        name: (*name).to_string(),
                    })
                } else {
                    Err(anyhow!("AvatarData {} name was null", id))
                }
            } else {
                Err(anyhow!("AvatarData {} was null", id))
            }
        }
    } else {
        Err(anyhow!("Avatar Entity was null"))
    }
}

#[named]
pub unsafe fn get_avatar_from_servant_entity(entity: *const GameEntity) -> Result<Avatar> {
    log::debug!(function_name!());
    if !entity.is_null() {
        unsafe {
            let entity_manager = GamePlayStatic_GetEntityManager();
            let avatar_entity = EntityManager__GetEntitySummoner(entity_manager, entity);
            get_avatar_from_entity(avatar_entity)
        }
    } else {
        Err(anyhow!("Servant Entity was null"))
    }
}

#[named]
pub fn fixpoint_to_raw(fixpoint: &FixPoint) -> f64 {
    log::debug!(function_name!());
    static float_conversion_const: LazyLock<f64> = LazyLock::new(|| (1f64 / (2f64.powf(32f64))));
    let hi = ((fixpoint.m_rawValue as u64 & 0xFFFFFFFF00000000) >> 32) as u32;
    let lo = (fixpoint.m_rawValue as u64 & 0x00000000FFFFFFFF) as u32;
    hi as f64 + lo as f64 * *float_conversion_const
}
