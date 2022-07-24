mod handler;

use common::Game;
use common::game::GameCommand;

use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for socket in listener.incoming() {
        let mut socket = socket.unwrap();

        let (socket_tx, socket_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let mut thread_socket = socket.try_clone().unwrap();

        thread::spawn(move || while let Ok(message) = socket_rx.recv() {
            thread_socket.write(&message.as_slice()).unwrap();
            thread_socket.flush().unwrap();
        });

        let (game_tx, game_rx): (Sender<GameCommand>, Receiver<GameCommand>) = mpsc::channel();
        thread::spawn(move || {
            // Construct game
            let mut game = Game::new(
                // Config
                toml::from_str(
                    &std::fs::read_to_string("config.toml").unwrap()
                ).unwrap()
            );
            while let Ok(cmd) = game_rx.recv() {
                use GameCommand::*;
                match cmd {
                    GetConfig { resp } => resp.send(game.config()).unwrap(),
                    GetClient { process_id, resp } => resp.send(game.client(process_id)).unwrap(),
                    AddClient { client } => game.add_client(client.process_id(), client),
                };
            }
        });

        let mut size_buffer = [0; 4];
        let mut buffer = [0; 1024];
        loop {
            // Read packet size (4 bytes)
            match socket.read_exact(&mut size_buffer) {
                Ok(_) => {}
                Err(_) => { break; }
            };
            let (size, _): (u32, usize) = util::packet::read_sized(&size_buffer.to_vec(), 0);

            // Read packet (size bytes)
            let mut handle = socket.try_clone().unwrap().take(size as u64);
            let len = handle.read(&mut buffer).unwrap();

            // Decode packet and handle it
            let packet = &buffer[0..len];
            let decoded = serde_json::from_slice(packet).unwrap();

            handler::handle(socket_tx.clone(), game_tx.clone(), decoded);
        };
    }
}

