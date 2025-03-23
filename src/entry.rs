use crate::globals::{Sockets, SOCKETS};
use crate::{globals, hooks};
use ctor::ctor;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1:1305";

#[ctor]
fn entry() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(10));
        unsafe { windows::Win32::System::Console::AllocConsole() };
        // Wait for il2cpp to initialize
        globals::BATTLE_LINEUP_TURN_DAMAGE.set(std::sync::RwLock::new(
            globals::BattleLineupTurnDamage::default(),
        ));
        println!("Installing hooks...");
        hooks::install_hooks().unwrap();
        println!("Finished installing hooks.");
    });

    thread::spawn(|| {
        main_server();
    });
}

fn handle_client(stream: TcpStream) {
    let mut sockets: std::sync::MutexGuard<'_, Sockets> = SOCKETS.get().unwrap().lock().unwrap();
    sockets.push(stream);
    drop(sockets);
    // loop {
    // let mut _client: tokio::sync::MutexGuard<'_, TcpStream> = client.lock().await;
    // let mut buf = [0; 1024];

    // match (*_client).read(&mut buf).await {
    //     Ok(0) => {
    //         println!("Client disconnected.");
    //         SOCKETS.get().unwrap().remove_socket(&client);
    //         break;
    //     }
    //     Ok(n) => {
    //         // For GatoSR
    //         // Handle ID w/ decrypted msg from client
    //         // And send back to client the decrypted msg or something similar

    //         // println!("Received: {:?}", &buffer[..n]);
    //         // // Echo the received data back to the client
    //         // if let Err(e) = stream.write_all(&buffer[..n]).await {
    //         //     eprintln!("Failed to send data to client: {}", e);
    //         //     break;
    //         // }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to read from stream: {}", e);
    //         break;
    //     }
    // }
    // }
}

fn main_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    SOCKETS.set(Mutex::new(Sockets::default()));

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    Ok(())
}
