use crate::hooks;
use ctor::ctor;
use std::thread;
use std::time::Duration;

#[ctor]
fn entry() {
    thread::spawn(|| {
        unsafe { windows::Win32::System::Console::AllocConsole() };
        // Wait for il2cpp to initialize
        thread::sleep(Duration::from_secs(5));
        println!("Installing hooks...");
        hooks::install_hooks().unwrap();
        println!("Finished installing hooks.");
    });

    thread::spawn(|| {
        crate::server::start_server();
    });
}