use common::{ChatBuilder,chat::Chat,Client,Game};
use common::packet::*;
use util::packet::*;
use serde_json::json;

use std::sync::mpsc::Sender;

pub fn handle(socket: Sender<Vec<u8>>, game: Game, packet: Packet) {
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
                    socket.send(packet).unwrap();
                }
                1 => {
                    // Long value sent with ping request
                    let (ping_long, _): (u64, usize) = read_sized(&data, offset);
                    let packet = PacketBuilder::new(0x01, pid)
                        .write_sized(ping_long)
                        .finish();
                    socket.send(packet).unwrap();
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

            socket.send(login_success).unwrap();
            socket.send(join_game).unwrap();
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

                            socket.send(packet).unwrap();
                        }
                        // Client uses a supported version
                        Some(version) => {
                            // Remove "downloading terrain" screen
                            let packet = PositionAndLookPacketBuilder::new(0.0, 0.0, 0.0).finish(pid.clone());
                            socket.send(packet).unwrap();

                            let username = state["username"].as_str().unwrap();

                            // Add client
                            let client = Client::new(
                                socket.clone(),
                                pid,
                                version,
                                locale,
                                String::from(username),
                            );
                            game.add_client(client);

                            // Log join message
                            let message = format!("{} joined the game.", username);
                            println!("{}", message);
                            let chat = &ChatBuilder::new(&message)
                                .color("yellow")
                                .finish();
                            for (_, client) in game.clients() {
                                let client = client.lock().unwrap();
                                client.send_chat(chat);
                            }
                        }
                    };
                }
                // Chat message
                0x01 => {
                    // Client message
                    let (input, _) = util::packet::read_string(&data, offset).unwrap();

                    let message: String;
                    let chat: Chat;
                    {
                        // Get client
                        let client = game.client(pid).unwrap();
                        let client = client.lock().unwrap();

                        message = format!("{}: {}", client.username(), input);

                        // Convert to chat component
                        chat = ChatBuilder::new(&message)
                            .color("gray")
                            .finish();

                        // MutexGuard gets dropped, we can access client
                    }

                    println!("{}", message);

                    let chat = &chat;

                    // Get clients
                    let clients = game.clients();
                    for (_, client) in clients {
                        let client = client.lock().unwrap();
                        client.send_chat(chat);
                    }
                }
                _ => {}
            }
        }
        state => println!("Unknown state: {}", state)
    }
}

