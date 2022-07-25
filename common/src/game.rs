use crate::{Config,Client,client::ClientRef};

use std::sync::{Arc,Mutex,mpsc::Sender};
use std::collections::HashMap;

pub struct Game {
    config: Config,
    clients: HashMap<String, ClientRef>,
}

impl Game {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            clients: HashMap::new(),
        }
    }

    pub fn config(&self) -> Config {
        self.config.clone()
    }

    pub fn client(&self, process_id: String) -> Option<ClientRef> {
        return match self.clients.get(&process_id) {
            None => None,
            Some(client) => Some(client.clone())
        };
    }

    pub fn clients(&self) -> HashMap<String, ClientRef> {
        self.clients.clone()
    }

    pub fn add_client(&mut self, process_id: String, client: Client) {
        self.clients.insert(process_id, Arc::new(Mutex::new(client)));
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

