use common::chat::ChatBuilder;
use util::packet::*;
use serde_json::json;

pub fn handle(socket: std::sync::mpsc::Sender<Vec<u8>>, buffer: Vec<u8>) {
    let Packet { pid, mut state, data }: Packet = serde_json::from_slice(buffer.clone().as_slice()).unwrap();
    let (_, offset) = read_varint(&data, 0).unwrap();
    let (packet_id, offset) = read_varint(&data, offset).unwrap();

    match packet_id {
        0x00 => {
            // Status request packet
            if data.len() <= 2 {
                return;
            }
            let (protocol_version, offset) = read_varint(&data, offset).unwrap();
            let (_, offset) = read_string(&data, offset).unwrap();
            let next_state = data[offset + 2];
            state["state"] = json!(next_state);

            match next_state {
                1 => {
                    let description = ChatBuilder::new("Welcome to Crust")
                        .color(String::from("gold"))
                        .bold()
                        .italic()
                        .finish();

                    let version = util::version::from_protocol(protocol_version as u16);

                    let motd = json!({
                        "version": {
                            "name": "1.8-1.19",
                            "protocol": if let None = version { 0 } else { protocol_version },
                        },
                        "players": {
                            "max": 100,
                            "online": 0,
                            "sample": [],
                        },
                        "description": description,
                    });

                    let packet = PacketBuilder::new(0x00, pid, state)
                        .write_string(motd.to_string())
                        .finish();

                    socket.send(packet).unwrap();
                }
                _ => {}
            }
        }
        0x01 => {
            socket.send(buffer).unwrap();
        }
        id => println!("Unhandled packet ID: {}", id)
    }
}

