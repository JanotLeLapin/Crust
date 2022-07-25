use crate::{Config,Client,client::ClientRef};

use std::collections::HashMap;
use std::sync::mpsc;

use mpsc::Sender;

pub struct Game {
    tx: Sender<GameCommand>,
}

impl Game {
    pub fn new(tx: Sender<GameCommand>) -> Self {
        Self {
            tx,
        }
    }

    pub fn tx(&self) -> Sender<GameCommand> {
        self.tx.clone()
    }

    pub fn config(&self) -> Config {
        let (resp_tx, resp_rx) = mpsc::channel::<Config>();
        self.tx.send(GameCommand::GetConfig {
            resp: resp_tx,
        }).unwrap();

        resp_rx.recv().unwrap()
    }

    pub fn client(&self, process_id: String) -> Option<ClientRef> {
        let (resp_tx, resp_rx) = mpsc::channel::<Option<ClientRef>>();
        self.tx.send(GameCommand::GetClient {
            process_id,
            resp: resp_tx
        }).unwrap();

        resp_rx.recv().unwrap()
    }

    pub fn clients(&self) -> HashMap<String, ClientRef> {
        let (resp_tx, resp_rx) = mpsc::channel::<HashMap<String, ClientRef>>();
        self.tx.send(GameCommand::GetClients {
            resp: resp_tx
        }).unwrap();

        resp_rx.recv().unwrap()
    }

    pub fn add_client(&self, client: Client) {
        self.tx.send(GameCommand::AddClient {
            client,
        }).unwrap();
    }
}

pub enum GameCommand {
    GetConfig {
        resp: Sender<Config>,
    },
    GetClient {
        process_id: String,
        resp: Sender<Option<ClientRef>>,
    },
    GetClients {
        resp: Sender<HashMap<String, ClientRef>>,
    },
    AddClient {
        client: Client,
    },
}

