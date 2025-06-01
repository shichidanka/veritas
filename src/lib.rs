#![recursion_limit = "256"]
#[macro_use]
extern crate rust_i18n;

mod battle;
mod entry;
mod kreide;
mod logging;
mod models;
mod overlay;
mod server;
mod subscribers;
mod ui;
mod prelude;

use phf::phf_map;
use std::{sync::LazyLock, thread, time::Duration};
use windows::{Win32::System::LibraryLoader::GetModuleHandleW, core::w};

pub static GAMEASSEMBLY_HANDLE: LazyLock<usize> = LazyLock::new(|| unsafe {
    loop {
        if let Ok(module) = GetModuleHandleW(w!("GameAssembly")) {
            return module.0 as usize
        }
        thread::sleep(Duration::from_millis(1));
    }
});

pub static UNITYPLAYER_HANDLE: LazyLock<usize> = LazyLock::new(|| unsafe {
    loop {
        if let Ok(module) = GetModuleHandleW(w!("UnityPlayer")) {
            return module.0 as usize
        }
        thread::sleep(Duration::from_millis(1));
    }
});

static LOCALES: phf::Map<&'static str, &'static str> = phf_map! {
    "en" => "English",
    "fr" => "Français",
    "es" => "Español",
    "de" => "Deutsch",
    "it" => "Italiano",
    "ja" => "日本語",
    "nl" => "Nederlands",
    "pl" => "Polski",
    "pt" => "Português",
    "ru" => "Русский",
    "vi" => "Tiếng Việt",
    "zh" => "中文",
    "ar" => "العربية",
};

rust_i18n::i18n!();
