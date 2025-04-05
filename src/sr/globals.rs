use std::ffi::c_void;

use super::offsets::GLOBAL_VARS_PTR;

pub enum GlobalVars {
    s_ModuleManager = 0x365c0,
}

impl GlobalVars {
    pub fn get_global_var(global_var: Self) -> *const c_void {
        unsafe {
            let ptr = *GLOBAL_VARS_PTR as *const c_void;
            *(ptr.byte_offset(global_var as _) as *const *const c_void)
        }
    }
}
