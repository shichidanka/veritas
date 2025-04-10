use std::sync::LazyLock;

// This is to keep track of static offsets that need to be manually updated
pub static TEXTID_TYPE_PTR_OFFSET: LazyLock<usize> = lazy_initialize_address!(0x456CFC0);

pub static GLOBAL_VARS_PTR_OFFSET: LazyLock<usize> = lazy_initialize_address!(0x3fb6858);

pub static S_MODULEMANAGER_FIELD_OFFSET: isize = 0xcd8;