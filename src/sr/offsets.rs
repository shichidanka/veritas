use std::{ffi::c_void, sync::LazyLock};
use crate::GAMEASSEMBLY_HANDLE;

// This is to keep track of offsets that need to be manually updated
pub static TEXTID_TYPE_PTR: LazyLock<usize> = lazy_initialize_address!(
    *((*GAMEASSEMBLY_HANDLE + 0x456CFC0) as *const *const c_void)
);

pub static GLOBAL_VARS_PTR: LazyLock<usize> = lazy_initialize_address!(
    *((*GAMEASSEMBLY_HANDLE + 0x425B9F0) as *const *const c_void)
);

// pub static SKILLCHARACTERCOMPONENT_TYPE_PTR: LazyLock<usize> = lazy_initialize_address!(
//     *((*GAMEASSEMBLY_HANDLE + 0x435BF08) as *const *const c_void)
// );

// pub static GAMEENTITY_MAP_FN_PTR_VA: usize = 0x70EA980;