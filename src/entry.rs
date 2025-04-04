use crate::hooks;
use ctor::ctor;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::thread::{self};
use std::time::Duration;

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        // windows::Win32::System::Console::AllocConsole();

        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err() ||
            GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err() {
            thread::sleep(Duration::from_millis(10));
        }

        log::info!("Installing hooks...");
        hooks::directx::install_hooks().unwrap();
        hooks::battle::install_hooks().unwrap();
        log::info!("Finished installing hooks.");
    });

    thread::spawn(|| {
        crate::server::start_server();
    });
}