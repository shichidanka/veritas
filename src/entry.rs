use crate::{kreide, logging, overlay, server, subscribers};
use ctor::ctor;
use egui_notify::{Toast, ToastOptions};
use std::{thread::{self}, time::Duration};

#[ctor]
fn entry() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        init()
    });

    thread::spawn(|| {
        server::start_server();
    });
}

fn init() {
    logging::MultiLogger::init();
    log::info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    log::info!("Setting up...");
    let mut toasts = Vec::<Toast>::new();
    match (|| -> anyhow::Result<()> {
        unsafe {
            #[cfg(debug_assertions)]
            windows::Win32::System::Console::AllocConsole()?;
        }
        kreide::il2cpp::init()?;
        subscribers::battle::subscribe()?;
        subscribers::enable_subscribers!()?;
        Ok(())
    })() {
        Ok(_) => {
            let msg = format!("Finished setting up {} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            log::info!("{}", msg);
            toasts.push(Toast::success(msg));
        },
        Err(e) => {
            let err = format!("{}", e);
            log::error!("{}", err);
            toasts.push(Toast::error(err));
            let err = format!("{} {} has been disabled", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            log::error!("{}", err);
            toasts.push(Toast::error(err));
        },
    };
    if overlay::initialize(toasts).is_err() {
        log::error!("Failed to initialize overlay");
    } else {
        log::info!("Finished setting up overlay");
    }
}
