use std::{ffi::c_void, sync::LazyLock};

use crate::globals::GAMEASSEMBLY_HANDLE;

pub enum GlobalVars {
    s_ModuleManager = 0x365c0,
}

impl GlobalVars {
    pub fn get_global_var(global_var: Self) -> *const c_void {
        unsafe {
            static global_vars_ptr_offset: LazyLock<usize> =
                LazyLock::new(|| *GAMEASSEMBLY_HANDLE + 0x425B9F0);
            let ptr = *(*global_vars_ptr_offset as *const *const c_void);
            *(ptr.byte_offset(global_var as _) as *const *const c_void)
        }
    }
}
