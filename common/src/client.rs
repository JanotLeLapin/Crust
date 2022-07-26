use crate::chat::Chat;
use crate::game::GameCommand;

use util::packet::PacketBuilder;

use std::sync::mpsc;
use mpsc::Sender;

#[derive(Clone)]
pub struct Client {
    tx: Sender<GameCommand>,
    pid: String,
}

impl Client {
    pub fn new(tx: Sender<GameCommand>, pid: String) -> Self {
        Self { tx, pid, }
    }

    pub fn process_id(&self) -> String {
        self.pid.clone()
    }

    pub fn locale(&self) -> String {
        let (tx, rx) = mpsc::channel::<String>();
        self.tx.send(GameCommand::GetClientProperty {
            resp: tx,
            process_id: self.pid.clone(),
            property: String::from("locale"),
        }).unwrap();
        return rx.recv().unwrap();
    }

    pub fn username(&self) -> String {
        let (tx, rx) = mpsc::channel::<String>();
        self.tx.send(GameCommand::GetClientProperty {
            resp: tx,
            process_id: self.pid.clone(),
            property: String::from("username"),
        }).unwrap();
        return rx.recv().unwrap();
    }

    pub fn send_packet(&self, packet: &Vec<u8>) {
        self.tx.send(GameCommand::SendPacket { packet: packet.clone() }).unwrap();
    }

    pub fn send_chat(&self, chat: &Chat) {
        let packet = PacketBuilder::new(0x02, self.process_id())
            .write_string(serde_json::to_string(chat).unwrap())
            .write_sized(0 as u8) // Position (chat box)
            .finish();

        self.send_packet(&packet);
    }
}

