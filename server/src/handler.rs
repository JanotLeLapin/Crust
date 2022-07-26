use common::{ChatBuilder,Game};
use common::packet::*;
use util::packet::*;
use serde_json::json;

pub fn handle(game: Game, packet: Packet) {
    let Packet { pid, state, data } = packet;
    let (_, offset) = read_varint(&data, 0).unwrap();
    let (packet_id, offset) = read_varint(&data, offset).unwrap();

    match state["state"].as_u64().unwrap_or(0) {
        // Status
        1 => {
            match packet_id {
                0 => {
                    let config = game.config();
                    let description = ChatBuilder::new(&config.status.motd)
                        .color("gold")
                        .bold()
                        .finish();

                    let protocol = state["protocol"].as_u64().unwrap() as u16;
                    let version = util::version::from_protocol(protocol);

                    let motd = json!({
                        "version": {
                            "name": "1.8-1.19",
                            "protocol": if let None = version { 0 } else { protocol },
                        },
                        "players": {
                            "max": config.status.max_players,
                            "online": 0,
                            "sample": [],
                        },
                        "description": description,
                    });

                    let packet = PacketBuilder::new(0x00, pid)
                        .write_string(motd.to_string())
                        .finish();
                    game.send_packet(&packet);
                }
                1 => {
                    // Long value sent with ping request
                    let (ping_long, _): (u64, usize) = read_sized(&data, offset);
                    let packet = PacketBuilder::new(0x01, pid)
                        .write_sized(ping_long)
                        .finish();
                    game.send_packet(&packet)
                }
                _ => {}
            }
        }
        // Login
        2 => {
            let (username, _) = read_string(&data, offset).unwrap();

            let mut state = state.clone();
            state["username"] = json!(username);

            // Login succes packet
            let login_success = PacketBuilder::new(0x02, pid.clone())
                .write_string(uuid::Uuid::new_v4().to_string())
                .write_string(username)
                .state(state.clone())
                .finish();

            // Join game packet
            let join_game = JoinGamePacketBuilder::new().finish(pid, &state);

            game.send_packet(&login_success);
            game.send_packet(&join_game);
        }
        // Play
        3 => {
            match packet_id {
                // Client settings
                0x15 => {
                    let (locale, _) = util::packet::read_string(&data, offset).unwrap();

                    match util::version::from_protocol(state["protocol"].as_u64().unwrap() as u16) {
                        // Client uses an unsupported client
                        None => {
                            let message = match locale.split("_").collect::<Vec<&str>>()[0] {
                                "fr" => "Version non supportée.",
                                _ => "Unsupported version."
                            };

                            let chat = ChatBuilder::new(message).color("red").finish();
                            let packet = PacketBuilder::new(0x40, pid.clone())
                                .write_string(serde_json::to_string(&chat).unwrap())
                                .finish();

                            game.send_packet(&packet);
                        }
                        // Client uses a supported version
                        Some(version) => {
                            // Remove "downloading terrain" screen
                            let packet = PositionAndLookPacketBuilder::new(0.0, 0.0, 0.0).finish(pid.clone());
                            game.send_packet(&packet);

                            let username = state["username"].as_str().unwrap();

                            // Add client
                            game.add_client(&pid, &version, &locale, username);

                            // Log join message
                            let message = format!("{} joined the game.", username);
                            println!("{}", message);
                            let chat = &ChatBuilder::new(&message)
                                .color("yellow")
                                .finish();
                            for client in game.clients() {
                                client.send_chat(chat);
                            }
                        }
                    };
                }
                // Chat message
                0x01 => {
                    // Client message
                    let (input, _) = util::packet::read_string(&data, offset).unwrap();

                    let client = game.client(&pid).unwrap();
                    let message = format!("{}: {}", client.username(), input);
                    let chat = &ChatBuilder::new(&message)
                        .color("gray")
                        .finish();
                    println!("{}", message);

                    // Get clients
                    let clients = game.clients();
                    for client in clients {
                        client.send_chat(chat);
                    }
                }
                _ => {}
            }
        }
        state => println!("Unknown state: {}", state)
    }
}

