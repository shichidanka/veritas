use super::{functions::rpg::client::AvatarModule_GetAvatar, globals::GlobalVars, types::rpg::client::{AvatarData, ModuleManager}};

#[inline]
pub fn get_avatar_data_by_id(avatar_id: u32) -> *const AvatarData {
    let s_module_manager = GlobalVars::get_global_var(GlobalVars::s_ModuleManager) as *const ModuleManager;
    let avatar_module = unsafe { (*s_module_manager).AvatarModule };
    AvatarModule_GetAvatar(avatar_module, avatar_id)
}
