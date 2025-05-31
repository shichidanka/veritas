use crate::{kreide, logging, overlay, server, subscribers, GAMEASSEMBLY_HANDLE, UNITYPLAYER_HANDLE};
use ctor::ctor;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::thread::{self};
use std::time::Duration;

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        #[cfg(debug_assertions)]
        windows::Win32::System::Console::AllocConsole();
        logging::MultiLogger::init();
        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err() ||
            GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err() {
            thread::sleep(Duration::from_millis(1));
        }

        log::info!("Build: {}", env!("CARGO_PKG_VERSION"));
        log::info!("Setting up...");
        kreide::il2cpp::init(*GAMEASSEMBLY_HANDLE, *UNITYPLAYER_HANDLE);
        overlay::initialize().unwrap();
        subscribers::battle::subscribe().unwrap();
        log::info!("Finished setup.");
    });

    thread::spawn(||{
        server::start_server();
    });
}