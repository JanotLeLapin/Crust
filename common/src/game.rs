use crate::Client;
use crate::Config;

use util::version::Version;

use std::sync::mpsc;
use mpsc::Sender;

pub struct Game {
    game_tx: Sender<GameCommand>,
}

impl Game {
    pub fn new(game_tx: Sender<GameCommand>) -> Self {
        Self {
            game_tx,
        }
    }

    pub fn game_tx(&self) -> Sender<GameCommand> {
        self.game_tx.clone()
    }

    pub fn send_packet(&self, packet: &Vec<u8>) {
        self.game_tx.send(GameCommand::SendPacket { packet: packet.clone() }).unwrap();
    }

    pub fn config(&self) -> Config {
        let (resp_tx, resp_rx) = mpsc::channel::<Config>();
        self.game_tx.send(GameCommand::GetConfig {
            resp: resp_tx,
        }).unwrap();

        resp_rx.recv().unwrap()
    }

    pub fn client(&self, process_id: &str) -> Option<Client> {
        let (resp_tx, resp_rx) = mpsc::channel::<bool>();
        self.game_tx.send(GameCommand::HasClient {
            resp: resp_tx,
            process_id: process_id.to_string(),
        }).unwrap();

        match resp_rx.recv().unwrap() {
            true => Some(Client::new(self.game_tx.clone(), process_id.to_string())),
            false => None,
        }
    }

    pub fn clients(&self) -> Vec<Client> {
        let (resp_tx, resp_rx) = mpsc::channel::<Vec<String>>();
        self.game_tx.send(GameCommand::GetClients {
            resp: resp_tx
        }).unwrap();

        resp_rx.recv().unwrap().into_iter().map(|pid| self.client(&pid).unwrap()).collect()
    }

    pub fn add_client(&self, process_id: &str, version: &Version, locale: &str, username: &str) -> Client {
        self.game_tx.send(GameCommand::AddClient {
            process_id: String::from(process_id),
            version: version.clone(),
            locale: String::from(locale),
            username: String::from(username),
        }).unwrap();

        Client::new(self.game_tx.clone(), String::from(process_id))
    }
}

pub enum GameCommand {
    SendPacket { packet: Vec<u8> },

    GetConfig { resp: Sender<Config> },

    GetClients { resp: Sender<Vec<String>> },
    HasClient {
        resp: Sender<bool>,
        process_id: String,
    },
    GetClientProperty {
        resp: Sender<String>,
        process_id: String,
        property: String,
    },
    AddClient {
        process_id: String,
        version: Version,
        locale: String,
        username: String,
    },
}

