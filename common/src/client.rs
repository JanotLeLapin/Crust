use crate::chat::Chat;
use util::{packet::PacketBuilder,version::Version};

use std::sync::{Arc,Mutex,mpsc::Sender};

pub type ClientRef = Arc<Mutex<Client>>;

pub struct Client {
    socket: Sender<Vec<u8>>,
    pid: String,
    version: Version,
    locale: String,
    username: String,
}

impl Client {
    pub fn new(
        socket: Sender<Vec<u8>>,
        pid: String,
        version: Version,
        locale: String,
        username: String,
    ) -> Self {
        Self {
            socket,
            pid,
            version,
            locale,
            username,
        }
    }

    pub fn process_id(&self) -> String {
        self.pid.clone()
    }

    pub fn locale(&self) -> String {
        self.locale.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn send_packet(&self, packet: Vec<u8>) {
        self.socket.send(packet).unwrap();
    }

    pub fn send_chat(&self, chat: &Chat) {
        let packet = PacketBuilder::new(0x02, self.process_id())
            .write_string(serde_json::to_string(chat).unwrap())
            .write_sized(0 as u8) // Position (chat box)
            .finish();

        self.send_packet(packet);
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            socket: self.socket.clone(),
            pid: self.pid.clone(),
            version: self.version.clone(),
            username: self.username.clone(),
            locale: self.locale.clone(),
        }
    }
}

