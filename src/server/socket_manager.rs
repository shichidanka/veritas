
use std::{io::Write, net::TcpStream, sync::{LazyLock, Mutex, MutexGuard}};
use anyhow::Result;

use crate::models::packets::Packet;

#[derive(Default)]
pub struct SocketManager {
    clients: Vec<TcpStream>,
}

static SOCKET_MANAGER: LazyLock<Mutex<SocketManager>> = LazyLock::new(|| Mutex::new(SocketManager::default()));

impl SocketManager {
    pub fn get_instance() -> MutexGuard<'static, Self> {
        SOCKET_MANAGER.lock().unwrap()
    }

    pub fn push(&mut self, client: TcpStream) {
        self.clients.push(client);
    }

    fn garbage_collect(&mut self) {
        self.clients.retain(|x: &TcpStream| match x.peer_addr() {
            Result::Ok(_) => true,
            Err(_) => false,
        });
    }

    pub fn broadcast_packet(&mut self, packet: Packet) {
        for socket in &mut self.clients {
            if let Err(e) = socket.write(&packet.to_bytes()) {
                log::warn!("Socket removal reason: {e}")
            };
        }
        self.garbage_collect();
    }
}
