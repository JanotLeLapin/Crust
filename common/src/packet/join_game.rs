use serde_json::{Value,json};

pub struct JoinGamePacketBuilder {
    entity_id: u32,
    gamemode: u8,
    dimension: i8,
    difficulty: u8,
    max_players: u8,
    level_type: String,
    reduced_debug_info: bool,
}

impl JoinGamePacketBuilder {
    pub fn new() -> Self {
        JoinGamePacketBuilder {
            entity_id: 0,
            gamemode: 0,
            dimension: 0,
            difficulty: 0,
            max_players: 100,
            level_type: String::from("default"),
            reduced_debug_info: false,
        }
    }

    pub fn entity_id(mut self, entity_id: u32) -> Self {
        self.entity_id = entity_id;
        self
    }

    pub fn gamemode(mut self, gamemode: u8) -> Self {
        self.gamemode |= gamemode & 3;
        self
    }

    pub fn hardcore(mut self) -> Self {
        self.gamemode |= 4;
        self
    }

    pub fn dimension(mut self, dimension: i8) -> Self {
        self.dimension = dimension;
        self
    }

    pub fn difficulty(mut self, difficulty: u8) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn max_players(mut self, max_players: u8) -> Self {
        self.max_players = max_players;
        self
    }

    pub fn level_type(mut self, level_type: String) -> Self {
        self.level_type = level_type;
        self
    }

    pub fn reduced_debug_info(mut self, reduced: bool) -> Self {
        self.reduced_debug_info = reduced;
        self
    }

    pub fn finish(self, process_id: String, state: &Value) -> Vec<u8> {
        // Switch to play state
        let mut new_state = state.clone();
        new_state["state"] = json!(3);

        util::packet::PacketBuilder::new(0x01, process_id)
            .write_sized(self.entity_id)
            .write_sized(self.gamemode)
            .write_sized(self.dimension)
            .write_sized(self.difficulty)
            .write_sized(self.max_players)
            .write_string(self.level_type)
            .write_sized(self.reduced_debug_info as u8)
            .state(new_state)
            .finish()
    }
}

