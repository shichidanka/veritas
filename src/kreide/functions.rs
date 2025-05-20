#![allow(unused_imports)]
use std::{ffi::c_void, sync::LazyLock};
use function_name::named;
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub mod unityengine {
    use std::{ffi::c_void, sync::LazyLock};
    use function_name::named;
    use crate::kreide::native_types::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::client::*;
    #[named]
    pub unsafe fn Application_set_targetFrameRate(a1: i32) {
        log::debug!(function_name!());
        unsafe {
            std::mem::transmute::<
                usize,
                unsafe extern "C" fn(i32),
            >(0xe65e5f0 + *crate::GAMEASSEMBLY_HANDLE)(a1)
        }
    }
}
pub mod rpg {
    use std::{ffi::c_void, sync::LazyLock};
    use function_name::named;
    use crate::kreide::native_types::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::client::*;
    pub mod client {
        use std::{ffi::c_void, sync::LazyLock};
        use function_name::named;
        use crate::kreide::native_types::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::client::*;
        #[named]
        pub unsafe fn AvatarModule_GetAvatar(
            a1: *const c_void,
            a2: u32,
        ) -> *const AvatarData {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const c_void, u32) -> *const AvatarData,
                >(0xb01ffe0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn UIGameEntityUtils_GetAvatarID(a1: *const GameEntity) -> u32 {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const GameEntity) -> u32,
                >(0xac84910 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn TextmapStatic_GetText(
            a1: *const TextID,
            a2: *const NativeArray<NativeObject>,
        ) -> *const NativeString {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TextID,
                        *const NativeArray<NativeObject>,
                    ) -> *const NativeString,
                >(0x8053330 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn AvatarData_get_AvatarName(
            a1: *const AvatarData,
        ) -> *const NativeString {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const AvatarData) -> *const NativeString,
                >(0xb02d630 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
    }
    pub mod gamecore {
        use std::{ffi::c_void, sync::LazyLock};
        use function_name::named;
        use crate::kreide::native_types::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::client::*;
        #[named]
        pub unsafe fn MonsterRowData_get_CharacterName(
            a1: *const TextID,
            a2: *const MonsterRowData,
        ) -> *const TextID {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TextID,
                        *const MonsterRowData,
                    ) -> *const TextID,
                >(0x94f4eb0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn MonsterRowData_get_Level(a1: *const MonsterRowData) -> u32 {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const MonsterRowData) -> u32,
                >(0x94f46a0 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn EntityManager__GetEntitySummoner(
            a1: *const EntityManager,
            a2: *const GameEntity,
        ) -> *const GameEntity {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const EntityManager,
                        *const GameEntity,
                    ) -> *const GameEntity,
                >(0x59a05c0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn MonsterDataComponent_GetMonsterID(
            a1: *const MonsterDataComponent,
        ) -> u32 {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const MonsterDataComponent) -> u32,
                >(0x94f1c30 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn ServantSkillRowData_get_SkillName(
            a1: *const TextID,
            a2: *const ServantSkillRowData,
        ) -> *const TextID {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TextID,
                        *const ServantSkillRowData,
                    ) -> *const TextID,
                >(0x951a110 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn ServantSkillRowData_get_AttackType(
            a1: *const ServantSkillRowData,
        ) -> AttackType {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const ServantSkillRowData) -> AttackType,
                >(0x951a060 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn TurnBasedAbilityComponent_GetAbilityMappedSkill(
            a1: *const TurnBasedAbilityComponent,
            a2: *const NativeString,
        ) -> *const NativeString {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TurnBasedAbilityComponent,
                        *const NativeString,
                    ) -> *const NativeString,
                >(0x593c340 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn TurnBasedAbilityComponent_GetProperty(
            a1: *const TurnBasedAbilityComponent,
            a2: AbilityProperty,
        ) -> FixPoint {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TurnBasedAbilityComponent,
                        AbilityProperty,
                    ) -> FixPoint,
                >(0x5939aa0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn TurnBasedAbilityComponent_TryCheckLimboWaitHeal(
            a1: *const TurnBasedAbilityComponent,
            a2: *const GameEntity,
        ) -> bool {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TurnBasedAbilityComponent,
                        *const GameEntity,
                    ) -> bool,
                >(0x597a030 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn CharacterConfig_GetSkillIndexByTriggerKey(
            a1: *const CharacterConfig,
            a2: *const NativeString,
        ) -> i32 {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const CharacterConfig,
                        *const NativeString,
                    ) -> i32,
                >(0xe321a70 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn AvatarSkillRowData_get_SkillName(
            a1: *const TextID,
            a2: *const AvatarSkillRowData,
        ) -> *const TextID {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TextID,
                        *const AvatarSkillRowData,
                    ) -> *const TextID,
                >(0x8c5e6e0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn AvatarSkillRowData_get_AttackType(
            a1: *const AvatarSkillRowData,
        ) -> AttackType {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const AvatarSkillRowData) -> AttackType,
                >(0x8c5e3e0 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn GamePlayStatic_GetEntityManager() -> *const EntityManager {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn() -> *const EntityManager,
                >(0x8dffa80 + *crate::GAMEASSEMBLY_HANDLE)()
            }
        }
        #[named]
        pub unsafe fn AbilityStatic_GetActualOwner(
            a1: *const GameEntity,
        ) -> *const GameEntity {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const GameEntity) -> *const GameEntity,
                >(0x6140970 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn BattleEventSkillRowData_get_SkillName(
            a1: *const TextID,
            a2: *const BattleEventSkillRowData,
        ) -> *const TextID {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const TextID,
                        *const BattleEventSkillRowData,
                    ) -> *const TextID,
                >(0x8c663d0 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2)
            }
        }
        #[named]
        pub unsafe fn BattleEventSkillRowData_get_AttackType(
            a1: *const BattleEventSkillRowData,
        ) -> AttackType {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(*const BattleEventSkillRowData) -> AttackType,
                >(0x8c66320 + *crate::GAMEASSEMBLY_HANDLE)(a1)
            }
        }
        #[named]
        pub unsafe fn SkillCharacterComponent_GetSkillData(
            a1: *const SkillCharacterComponent,
            a2: i32,
            a3: i32,
        ) -> *const SkillData {
            log::debug!(function_name!());
            unsafe {
                std::mem::transmute::<
                    usize,
                    unsafe extern "C" fn(
                        *const SkillCharacterComponent,
                        i32,
                        i32,
                    ) -> *const SkillData,
                >(0x5928910 + *crate::GAMEASSEMBLY_HANDLE)(a1, a2, a3)
            }
        }
    }
}
