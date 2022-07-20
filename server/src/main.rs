mod handler;

use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut buffer: [u8; 255];
    for socket in listener.incoming() {
        let mut socket = socket.unwrap();
        loop {
            buffer = [0; 255];
            socket.read(&mut buffer).unwrap();

            handler::handle(&mut socket, buffer.to_vec());
        }
    }
}

