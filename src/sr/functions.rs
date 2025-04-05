use std::{ffi::c_void, sync::LazyLock};

use crate::GAMEASSEMBLY_HANDLE;

use super::{offsets::GAMEENTITY_MAP_FN_PTR_VA, types::rpg::gamecore::GameEntity};

pub mod rpg {
    pub mod client {
        use std::{ffi::c_void, sync::LazyLock};
        
        use crate::sr::il2cpp_types::{Il2CppArray, Il2CppObject};
        use crate::sr::types::rpg::client::TextID;
        use crate::{sr::{il2cpp_types::Il2CppString, types::rpg::{client::AvatarData, gamecore::GameEntity}}, GAMEASSEMBLY_HANDLE};

        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8e39db0);
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8926d80);
        pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const Il2CppString> =
            lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x82c35a0);
        pub static TextmapStatic_GetText: LazyLock<fn(*const TextID, *const Il2CppArray<Il2CppObject>) -> *const Il2CppString> =
            lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8e43c30);
    }
    pub mod gamecore {
        use std::{ffi::c_void, sync::LazyLock};
        
        use crate::sr::types::rpg::gamecore::{AttackType, BattleEventSkillRowData, ServantSkillRowData, SkillCharacterComponent, SkillData};
        use crate::{sr::types::rpg::{client::TextID, gamecore::{AvatarSkillRowData, EntityManager, GameEntity}}, GAMEASSEMBLY_HANDLE};

        pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> =
            lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x919c730);

        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x83874c0);
        pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager, *const GameEntity) -> *const GameEntity> =
            lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8711b20);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*mut TextID, *const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x836aa60);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x836a760);
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*mut TextID, *const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8275da0);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8275cf0);
        pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*mut TextID, *const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x80f0de0);
        pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x80f0d30);
        pub static SkillCharacterComponent_GetCurrentSkillData: LazyLock<fn(*const c_void) -> *const SkillData> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8f23e60);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + 0x8f20df0);
    }
}

pub static GameEntity_Map: LazyLock<fn(*const GameEntity, *const c_void) -> *const c_void> = lazy_initialize_address!(*GAMEASSEMBLY_HANDLE + GAMEENTITY_MAP_FN_PTR_VA);

