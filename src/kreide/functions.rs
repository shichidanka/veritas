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
pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent,i32,i32) -> *const SkillData> = lazy_initialize_address!(0x588c190);
pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(0x5887090);
pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(0x5886f90);
pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager,*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x5903b90);
pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8d3f930);
pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(0x8d3f890);
pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<fn(*const CharacterConfig,*const NativeString) -> i32> = lazy_initialize_address!(0xe282a60);
pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<fn(*const TurnBasedAbilityComponent,*const NativeString) -> *const NativeString> = lazy_initialize_address!(0x589fb70);
pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(0x8a0c8f0);
pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x6014c40);
pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8d37fc0);
pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(0x8d37c90);
	}
	pub mod client {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const NativeString> = lazy_initialize_address!(0xaf6bb60);
pub static GlobalVars_cctor: LazyLock<fn() -> *const NativeObject> = lazy_initialize_address!(0x9e662e0);
pub static TextmapStatic_GetText: LazyLock<fn(*const TextID,*const NativeArray<NativeObject>) -> *const NativeString> = lazy_initialize_address!(0x8852340);
pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void,u32) -> *const AvatarData> = lazy_initialize_address!(0xaf5e580);
pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(0xac51c80);
	}
}
pub mod unityengine {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static Application_set_targetFrameRate: LazyLock<fn(i32)> = lazy_initialize_address!(0xe5ee960);
pub static Application_get_targetFrameRate: LazyLock<fn() -> i32> = lazy_initialize_address!(0xe5ee950);
}