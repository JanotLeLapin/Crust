use common::chat::ChatBuilder;
use util::packet::*;
use serde_json::json;

pub fn handle(socket: std::sync::mpsc::Sender<Vec<u8>>, buffer: Vec<u8>) {
    let Packet { pid, state, data }: Packet = serde_json::from_slice(buffer.clone().as_slice()).unwrap();
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
                        .state(state)
                        .write_string(motd.to_string())
                        .finish();
                    socket.send(packet).unwrap();
                }
                1 => socket.send(json!({
                    "pid": pid,
                    "data": data,
                }).to_string().into_bytes()).unwrap(),
                _ => {}
            }
        }
        // Login
        2 => {
        }
        // Play
        3 => {
        }
        state => println!("Unknown state: {}", state)
    }
}

