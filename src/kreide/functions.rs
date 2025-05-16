use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use std::{ffi::c_void, sync::LazyLock};
pub mod rpg {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub mod gamecore {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<
            fn(*const BattleEventSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8c663d0);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<
            fn(*const BattleEventSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8c66320);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x59a05c0);
        pub static ServantSkillRowData_get_SkillName: LazyLock<
            fn(*const ServantSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x951a110);
        pub static ServantSkillRowData_get_AttackType: LazyLock<
            fn(*const ServantSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x951a060);
        pub static AbilityStatic_GetActualOwner: LazyLock<
            fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x6140970);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<
            fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData,
        > = lazy_initialize_address!(0x5928910);
        pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<
            fn(*const CharacterConfig, *const NativeString) -> i32,
        > = lazy_initialize_address!(0xe321a70);
        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> =
            lazy_initialize_address!(0x8dffa80);
        pub static MonsterRowData_get_CharacterName: LazyLock<
            fn(*const MonsterRowData) -> *const TextID,
        > = lazy_initialize_address!(0x94f4eb0);
        pub static MonsterRowData_get_Level: LazyLock<fn(*const MonsterRowData) -> u32> =
            lazy_initialize_address!(0x94f46a0);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<
            fn(*const AvatarSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8c5e6e0);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<
            fn(*const AvatarSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8c5e3e0);
        pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<
            fn(*const TurnBasedAbilityComponent, *const NativeString) -> *const NativeString,
        > = lazy_initialize_address!(0x593c340);
        pub static TurnBasedAbilityComponent_GetProperty: LazyLock<
            fn(*const TurnBasedAbilityComponent, AbilityProperty) -> FixPoint,
        > = lazy_initialize_address!(0x5939aa0);
        pub static TurnBasedAbilityComponent_TryCheckLimboWaitHeal: LazyLock<
            fn(*const TurnBasedAbilityComponent, *const GameEntity) -> bool,
        > = lazy_initialize_address!(0x597a030);
        pub static MonsterDataComponent_GetMonsterID: LazyLock<
            fn(*const MonsterDataComponent) -> u32,
        > = lazy_initialize_address!(0x94f1c30);
    }
    pub mod client {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static AvatarData_get_AvatarName: LazyLock<
            fn(*const AvatarData) -> *const NativeString,
        > = lazy_initialize_address!(0xb02d630);
        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> =
            lazy_initialize_address!(0xac84910);
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> =
            lazy_initialize_address!(0xb01ffe0);
        pub static TextmapStatic_GetText: LazyLock<
            fn(*const TextID, *const NativeArray<NativeObject>) -> *const NativeString,
        > = lazy_initialize_address!(0x8053330);
    }
}
pub mod unityengine {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub static Application_set_targetFrameRate: LazyLock<fn(i32)> =
        lazy_initialize_address!(0xe65e5f0);
    pub static Application_get_targetFrameRate: LazyLock<fn() -> i32> =
        lazy_initialize_address!(0xe65e5e0);
}
