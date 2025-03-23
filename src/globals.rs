use std::{
    io::Write,
    net::TcpStream,
    sync::{Mutex, OnceLock, RwLock},
};

use serde::{Deserialize, Serialize};

use std::sync::LazyLock;
use windows::{core::w, Win32::System::LibraryLoader::GetModuleHandleW};
pub static GAMEASSEMBLY_HANDLE: LazyLock<usize> =
    LazyLock::new(|| unsafe { GetModuleHandleW(w!("GameAssembly")).unwrap().0 as usize });

#[derive(Default)]
pub struct Sockets {
    _inner: Vec<TcpStream>,
}

impl Sockets {
    pub fn push(&mut self, client: TcpStream) {
        self._inner.push(client);
    }

    pub fn garbage_collect(&mut self) {
        self._inner.retain(|x: &TcpStream| match x.peer_addr() {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    pub fn broadcast(&mut self, buffer: &[u8]) {
        for socket in &mut self._inner {
            match socket.write(buffer) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        self.garbage_collect();
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Avatar {
    pub avatar_id: u32,
    pub avatar_name: String,
}

#[derive(Default, Clone)]
pub struct AvatarTurnDamage {
    pub avatar: Avatar,
    pub damage_chunks: Vec<u32>,
    // pub damage: u32,
    // is_turn_owner
}

#[derive(Default, Clone)]
pub struct BattleLineupTurnDamage {
    pub lineup: [AvatarTurnDamage; 0x4],
}

impl BattleLineupTurnDamage {
    pub fn find_avatar_index_by_id(&self, avatar_id: u32) -> usize {
        // Let's not unwrap here
        let (index, _) = self
            .lineup
            .iter()
            .enumerate()
            .find(|(index, item)| (*item).avatar.avatar_id == avatar_id)
            .unwrap();
        index
    }
}

pub static SOCKETS: OnceLock<Mutex<Sockets>> = OnceLock::new();
pub static BATTLE_LINEUP_TURN_DAMAGE: OnceLock<RwLock<BattleLineupTurnDamage>> = OnceLock::new();
