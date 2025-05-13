use crate::{logging, overlay, server, subscribers};
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
            thread::sleep(Duration::from_millis(10));
        }

        log::info!("Build: {}", env!("TARGET_BUILD"));
        log::info!("Setting up...");
        overlay::initialize().unwrap();
        subscribers::battle::subscribe().unwrap();
        log::info!("Finished setup.");
        server::start_server();
    });
}