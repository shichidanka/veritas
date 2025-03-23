macro_rules! lazy_initialize {
    ($addr:expr) => {
        LazyLock::new(|| unsafe { mem::transmute($addr) })
    };
}
pub mod rpg {
    pub mod client {
        use crate::{
            globals::GAMEASSEMBLY_HANDLE,
            sr::{
                il2cpp_types::Il2CppString,
                types::RPG::{Client::AvatarData, GameCore::GameEntity},
            },
        };
        use std::{ffi::c_void, mem, sync::LazyLock};

        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> =
            lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x8e39db0);
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> =
            lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x8926d80);
        pub static AvatarData_get_AvatarName: LazyLock<
            fn(*const AvatarData) -> *const Il2CppString,
        > = lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x82c35a0);
    }
    pub mod gamecore {
        use std::{mem, sync::LazyLock};

        use crate::{
            globals::GAMEASSEMBLY_HANDLE,
            sr::types::RPG::GameCore::{EntityManager, GameEntity},
        };

        pub static AbilityStatic_GetActualOwner: LazyLock<
            fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x919c730);

        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> =
            lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x83874c0);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize!(*GAMEASSEMBLY_HANDLE + 0x8711b20);
    }
}
