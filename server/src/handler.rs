use common::chat::ChatBuilder;
use common::packet::*;
use util::packet::*;
use serde_json::json;

pub fn handle(socket: std::sync::mpsc::Sender<Vec<u8>>, packet: Packet) {
    let Packet { pid, state, data } = packet;
    let (_, offset) = read_varint(&data, 0).unwrap();
    let (packet_id, offset) = read_varint(&data, offset).unwrap();

    match state["state"].as_u64().unwrap_or(0) {
        // Status
        1 => {
            match packet_id {
                0 => {
                    let description = ChatBuilder::new("Welcome to Crust")
                        .color(String::from("gold"))
                        .bold()
                        .italic()
                        .finish();

                    let protocol = state["protocol"].as_u64().unwrap() as u16;
                    let version = util::version::from_protocol(protocol);

                    let motd = json!({
                        "version": {
                            "name": "1.8-1.19",
                            "protocol": if let None = version { 0 } else { protocol },
                        },
                        "players": {
                            "max": 100,
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
            let (name, _) = read_string(&data, offset).unwrap();

            // Login succes packet
            let login_success = PacketBuilder::new(0x02, pid.clone())
                .write_string(uuid::Uuid::new_v4().to_string())
                .write_string(name)
                .finish();

            // Join game packet
            let join_game = JoinGamePacketBuilder::new().finish(pid, &state);

            socket.send(login_success).unwrap();
            socket.send(join_game).unwrap();
        }
        // Play
        3 => {
        }
        state => println!("Unknown state: {}", state)
    }
}

