use crate::{kreide, logging, overlay, server, subscribers};
use ctor::ctor;
use egui_notify::Toast;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::{thread::{self}, time::Duration};

#[ctor]
fn entry() {
    thread::spawn(|| {
        init()
    });
}

fn init() {
    let mut toasts = Vec::<Toast>::new();
    match (|| -> anyhow::Result<()> { unsafe {
        logging::MultiLogger::init()?;
        #[cfg(debug_assertions)]
        windows::Win32::System::Console::AllocConsole()?;

        log::info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        log::info!("Setting up...");

        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err() ||
            GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err() {
            thread::sleep(Duration::from_millis(1));
        }

        kreide::il2cpp::init()?;
        subscribers::battle::subscribe()?;
        subscribers::enable_subscribers!()?;
        Ok(())
    } })() {
        Ok(_) => {
            let msg = format!("Finished setting up {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            log::info!("{}", msg);
            toasts.push(Toast::success(msg));
        },
        Err(e) => {
            let err = format!("{}", e);
            log::error!("{}", err);
            let mut toast = Toast::error(err);
            toast.duration(None);
            toasts.push(toast);
            let err = format!("{} {} has been disabled", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            log::error!("{}", err);
            let mut toast = Toast::error(err);
            toast.duration(None);
            toasts.push(toast);
        },
    };

    match overlay::initialize(toasts) {
        Ok(_) => log::info!("Finished setting up overlay"),
        Err(e) => log::error!("Failed to initialize overlay: {}", e),
    }

    thread::spawn(|| {
        server::start_server();
    });
}
