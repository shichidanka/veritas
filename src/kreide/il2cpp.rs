#![allow(static_mut_refs)]

use std::{borrow::Cow, collections::HashMap, sync::OnceLock};

use api::{Il2CppClass, Il2CppMethod};
use util::scan_unity_player_section;
use crate::prelude::*;
pub mod api;
pub mod misc;
pub mod native;
pub mod util;

static mut API_BASE_PTR: usize = 0;
static FUNCTIONS_TABLE: OnceLock<HashMap<String, Il2CppMethod>> = OnceLock::new();
static TYPE_TABLE: OnceLock<HashMap<Cow<'static, str>, Il2CppClass>> = OnceLock::new();

pub fn get_native_method(key: &str) -> Result<Il2CppMethod> {
    FUNCTIONS_TABLE.get().context("Failed to get function table")?.get(key).with_context(|| format!("Failed to get native method {}", key)).cloned()
}

pub fn get_cached_class(key: &str) -> Result<Il2CppClass> {
    TYPE_TABLE.get().context("Failed to get type table")?.get(key).with_context(|| format!("Failed to get cached class {}", key)).cloned()
}

pub fn init() -> anyhow::Result<()> {
    unsafe {
        API_BASE_PTR = scan_unity_player_section("48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0")?;

        let mut method_maps = HashMap::with_capacity(470_000);
        let mut type_table = HashMap::with_capacity(50_000);

        let domain = api::il2cpp_domain_get();
        api::il2cpp_thread_attach(domain);

        for assembly in domain.assemblies() {
            let image = api::il2cpp_assembly_get_image(assembly);

            for class in image.classes() {
                let type_name = class.byval_arg().name();
                for method in class.methods() {
                    method_maps.insert(format!("{type_name}::{}", method.format_params()), method);
                }
                type_table.insert(type_name, class);
            }
        }

        FUNCTIONS_TABLE.set(method_maps).unwrap();
        TYPE_TABLE.set(type_table).unwrap();
        Ok(())
    }
}
