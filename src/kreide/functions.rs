use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use std::{ffi::c_void, sync::LazyLock};
pub mod rpg {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub mod client {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static AvatarData_get_AvatarName: LazyLock<
            fn(*const AvatarData) -> *const NativeString,
        > = lazy_initialize_address!(0xab487b0);
        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> =
            lazy_initialize_address!(0xa1bebf0);
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> =
            lazy_initialize_address!(0xab38860);
        pub static TextmapStatic_GetText: LazyLock<
            fn(*const TextID, *const NativeArray<NativeObject>) -> *const NativeString,
        > = lazy_initialize_address!(0x8f296b0);
    }
    pub mod gamecore {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<
            fn(*const CharacterConfig, *const NativeString) -> i32,
        > = lazy_initialize_address!(0xe463010);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x5abd9d0);
        pub static AbilityStatic_GetActualOwner: LazyLock<
            fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x60abe10);
        pub static ServantSkillRowData_get_SkillName: LazyLock<
            fn(*const ServantSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x65bd4a0);
        pub static ServantSkillRowData_get_AttackType: LazyLock<
            fn(*const ServantSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x65bd3f0);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<
            fn(*const AvatarSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x883eb80);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<
            fn(*const AvatarSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x883e880);
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<
            fn(*const BattleEventSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x76c63d0);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<
            fn(*const BattleEventSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8845340);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<
            fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData,
        > = lazy_initialize_address!(0x65c2360);
        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> =
            lazy_initialize_address!(0xa9cf5f0);
        pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<
            fn(*const TurnBasedAbilityComponent, *const NativeString) -> *const NativeString,
        > = lazy_initialize_address!(0x5dd0770);
    }
}
