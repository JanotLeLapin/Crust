use serde::Deserialize;

#[derive(Deserialize)]
pub struct Status {
    pub motd: String,
    pub max_players: u32,
}

impl Clone for Status {
    fn clone(&self) -> Self {
        Self {
            motd: self.motd.clone(),
            max_players: self.max_players,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub status: Status,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            status: self.status.clone(),
        }
    }
}

