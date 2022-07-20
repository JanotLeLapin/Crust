use std::io::prelude::*;

use serde_json::json;
use util::packet::*;

pub fn handle(socket: &mut std::net::TcpStream, buffer: Vec<u8>) {
    let (process_id, offset) = read_string(&buffer, 0).unwrap();
    let (_, offset) = read_varint(&buffer, offset).unwrap();
    let (packet_id, offset) = read_varint(&buffer, offset).unwrap();

    println!("{:?}", buffer);

    match packet_id {
        0x00 => {
            println!("{}", offset);
            let (protocol_version, offset) = read_varint(&buffer, offset).unwrap();
            let (_, offset) = read_string(&buffer, offset).unwrap();
            let next_state = buffer[offset + 2];

            match next_state {
                1 => {
                    let motd = json!({
                        "version": {
                            "name": "1.8.8",
                            "protocol": protocol_version,
                        },
                        "players": {
                            "max": 100,
                            "online": 0,
                            "sample": [],
                        },
                        "description": {
                            "text": "Welcome to Crust",
                        },
                    });

                    let packet = PacketBuilder::new(0x00, process_id)
                        .write_string(motd.to_string())
                        .finish();

                    socket.write(&packet.as_slice()).unwrap();
                    socket.flush().unwrap();
                }
                _ => {}
            }
        }
        0x01 => {
            socket.write(&buffer.as_slice()).unwrap();
            socket.flush().unwrap();
        }
        id => println!("Unhandled packet ID: {}", id)
    }
}

