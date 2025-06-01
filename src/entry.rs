use crate::{kreide, logging, overlay, server, subscribers};
use ctor::ctor;
use std::thread::{self};

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        #[cfg(debug_assertions)]
        windows::Win32::System::Console::AllocConsole();
        logging::MultiLogger::init();

        log::info!("Build: {}", env!("CARGO_PKG_VERSION"));
        log::info!("Setting up...");
        kreide::il2cpp::init().unwrap();
        overlay::initialize().unwrap();
        subscribers::battle::subscribe().unwrap();
        log::info!("Finished setup.");
    });

    thread::spawn(||{
        server::start_server();
    });
}