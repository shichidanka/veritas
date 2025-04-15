use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub mod rpg {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
	pub mod gamecore {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(0x9a1c000);
pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager,*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x59a1230);
pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<fn(*const TurnBasedAbilityComponent,*const NativeString) -> *const NativeString> = lazy_initialize_address!(0x5c3d4f0);
pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<fn(*const CharacterConfig,*const NativeString) -> i32> = lazy_initialize_address!(0xe3a89b0);
pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent,i32,i32) -> *const SkillData> = lazy_initialize_address!(0x666eaf0);
pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x60b28e0);
pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8629260);
pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(0x8628f30);
pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(0x6669cd0);
pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(0x6669c20);
pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(0x7f0b700);
pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(0x7f0b680);
	}
	pub mod client {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(0x9317630);
pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const NativeString> = lazy_initialize_address!(0xa9f8450);
pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void,u32) -> *const AvatarData> = lazy_initialize_address!(0xa9e82c0);
pub static TextmapStatic_GetText: LazyLock<fn(*const TextID,*const NativeArray<NativeObject>) -> *const NativeString> = lazy_initialize_address!(0x87564c0);
	}
}
