use crate::{hooks, ui};
use ctor::ctor;
use std::thread;
use std::time::Duration;

#[ctor]
fn entry() {
    thread::spawn(|| {
        unsafe { windows::Win32::System::Console::AllocConsole() };
        println!("Waiting for runtime to initialize...");
        thread::sleep(Duration::from_secs(5));
        hooks::directx::install_hooks().unwrap();
        println!("Installing hooks...");
        hooks::battle::install_hooks().unwrap();
        println!("Finished installing hooks.");
    });

    thread::spawn(|| {
        crate::server::start_server();
    });
}