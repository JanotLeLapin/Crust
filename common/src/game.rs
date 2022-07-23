use crate::config::Config;

use std::sync::mpsc::Sender;

pub struct Game {
    config: Config,
}

impl Game {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    pub fn config(&self) -> Config {
        self.config.clone()
    }
}

pub enum GameCommand {
    GetConfig {
        resp: Sender<Config>,
    },
}

