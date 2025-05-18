#![allow(unused_imports)]
use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
pub mod rpg {
    pub mod gamecore {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::sync::LazyLock;
        pub static AbilityStatic_GetActualOwner: LazyLock<
            unsafe fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x67483f0);
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<
            unsafe fn(*const BattleEventSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x93cd310);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<
            unsafe fn(*const BattleEventSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x93cd260);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            unsafe fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x5976260);
        pub static ServantSkillRowData_get_SkillName: LazyLock<
            unsafe fn(*const ServantSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x5a82350);
        pub static ServantSkillRowData_get_AttackType: LazyLock<
            unsafe fn(*const ServantSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x5a822a0);
        pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<
            unsafe fn(*const TurnBasedAbilityComponent, *const NativeString) -> *const NativeString,
        > = lazy_initialize_address!(0x5a9ae40);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<
            unsafe fn(*const AvatarSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x93c5910);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<
            unsafe fn(*const AvatarSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x93c55e0);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<
            unsafe fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData,
        > = lazy_initialize_address!(0x5a87470);
        pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<
            unsafe fn(*const CharacterConfig, *const NativeString) -> i32,
        > = lazy_initialize_address!(0xe28bb30);
        pub static GamePlayStatic_GetEntityManager: LazyLock<unsafe fn() -> *const EntityManager> =
            lazy_initialize_address!(0x7f9c510);
    }
    pub mod client {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static UIGameEntityUtils_GetAvatarID: LazyLock<unsafe fn(*const GameEntity) -> u32> =
            lazy_initialize_address!(0xae43d30);
        pub static AvatarModule_GetAvatar: LazyLock<
            unsafe fn(*const c_void, u32) -> *const AvatarData,
        > = lazy_initialize_address!(0x9c53f00);
        pub static AvatarData_get_AvatarName: LazyLock<
            unsafe fn(*const AvatarData) -> *const NativeString,
        > = lazy_initialize_address!(0xb3be890);
        pub static TextmapStatic_GetText: LazyLock<
            unsafe fn(*const TextID, *const NativeArray<NativeObject>) -> *const NativeString,
        > = lazy_initialize_address!(0x83e6850);
        pub static GlobalVars_cctor: LazyLock<unsafe fn() -> *const NativeObject> =
            lazy_initialize_address!(0x9d94050);
    }
}
pub mod unityengine {
    use std::sync::LazyLock;
    pub static Application_set_targetFrameRate: LazyLock<unsafe fn(i32)> =
        lazy_initialize_address!(0xe5f94d0);
    pub static Application_get_targetFrameRate: LazyLock<unsafe fn() -> i32> =
        lazy_initialize_address!(0xe5f94c0);
}
