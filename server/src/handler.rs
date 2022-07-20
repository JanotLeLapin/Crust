use serde_json::json;
use common::chat::ChatBuilder;
use util::packet::*;

pub fn handle(socket: std::sync::mpsc::Sender<Vec<u8>>, buffer: Vec<u8>) {
    let (process_id, offset) = read_string(&buffer, 0).unwrap();
    let (_, offset) = read_varint(&buffer, offset).unwrap();
    let (packet_id, offset) = read_varint(&buffer, offset).unwrap();

    println!("{:?}", buffer);

    match packet_id {
        0x00 => {
            let (protocol_version, offset) = read_varint(&buffer, offset).unwrap();
            let (_, offset) = read_string(&buffer, offset).unwrap();
            let next_state = buffer[offset + 2];

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

                    let packet = PacketBuilder::new(0x00, process_id)
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

