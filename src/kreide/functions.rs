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
pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x67483f0);
pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(0x93cd310);
pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(0x93cd260);
pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager,*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x5976260);
pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(0x5a82350);
pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(0x5a822a0);
pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<fn(*const TurnBasedAbilityComponent,*const NativeString) -> *const NativeString> = lazy_initialize_address!(0x5a9ae40);
pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(0x93c5910);
pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(0x93c55e0);
pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent,i32,i32) -> *const SkillData> = lazy_initialize_address!(0x5a87470);
pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<fn(*const CharacterConfig,*const NativeString) -> i32> = lazy_initialize_address!(0xe28bb30);
pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(0x7f9c510);
	}
	pub mod client {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(0xae43d30);
pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void,u32) -> *const AvatarData> = lazy_initialize_address!(0x9c53f00);
pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const NativeString> = lazy_initialize_address!(0xb3be890);
pub static TextmapStatic_GetText: LazyLock<fn(*const TextID,*const NativeArray<NativeObject>) -> *const NativeString> = lazy_initialize_address!(0x83e6850);
pub static GlobalVars_cctor: LazyLock<fn() -> *const NativeObject> = lazy_initialize_address!(0x9d94050);
	}
}
pub mod unityengine {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static Application_set_targetFrameRate: LazyLock<fn(i32)> = lazy_initialize_address!(0xe5f94d0);
pub static Application_get_targetFrameRate: LazyLock<fn() -> i32> = lazy_initialize_address!(0xe5f94c0);
}