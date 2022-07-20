mod handler;

use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut buffer: [u8; 1024];
    for socket in listener.incoming() {
        let mut socket = socket.unwrap();

        let (socket_tx, socket_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let mut thread_socket = socket.try_clone().unwrap();

        thread::spawn(move || while let Ok(message) = socket_rx.recv() {
            thread_socket.write(&message.as_slice()).unwrap();
            thread_socket.flush().unwrap();
        });

        loop {
            buffer = [0; 1024];
            match socket.read(&mut buffer).unwrap() {
                0 => break,
                len => handler::handle(socket_tx.clone(), buffer[0..len].to_vec()),
            };
        };
    }
}

