mod socket_manager;

use std::{net::{TcpListener, TcpStream}, thread, time::Duration};
use anyhow::{ Context, Result };
use crate::models::packets::{EventPacket, Packet};

use self::socket_manager::SocketManager;

const SERVER_ADDR: &str = "127.0.0.1:1305";

pub fn start_server() -> Result<()> {
    let listener = TcpListener::bind(SERVER_ADDR).with_context(|| format!("Failed to start server address {}", SERVER_ADDR))?;

    // Accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                log::error!("Error: Client failed to connect to server:\n{e}")
            }
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    let mut socket_manager = SocketManager::get_instance();
    socket_manager.push(stream);
    drop(socket_manager);
    loop {
        let packet = Packet::from_event_packet(EventPacket::Heartbeat {  }).unwrap();
        let mut socket_manager = SocketManager::get_instance();
        socket_manager.broadcast_packet(packet);
        drop(socket_manager);
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn broadcast(packet: Packet) {
    let mut socket_manager = SocketManager::get_instance();
    socket_manager.broadcast_packet(packet)
}