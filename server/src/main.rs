mod handler;

use common::{client::ClientRef,ChatBuilder,Config};
use common::game::{GameCommand,Game};

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::{Arc,Mutex,mpsc};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for socket in listener.incoming() {
        let mut socket = socket.unwrap();

        let (socket_tx, socket_rx) = mpsc::channel::<Vec<u8>>();
        let mut thread_socket = socket.try_clone().unwrap();

        thread::spawn(move || while let Ok(message) = socket_rx.recv() {
            thread_socket.write(&message.as_slice()).unwrap();
            thread_socket.flush().unwrap();
        });

        let (game_tx, game_rx) = mpsc::channel::<GameCommand>();
        thread::spawn(move || {
            let config: Config = toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
            let mut clients = HashMap::<String,ClientRef>::new();
            while let Ok(cmd) = game_rx.recv() {
                use GameCommand::*;
                match cmd {
                    GetConfig { resp } => resp.send(config.clone()).unwrap(),
                    GetClient { process_id, resp } => resp.send(match clients.get(&process_id) {
                        None => None,
                        Some(client) => Some(client.clone()),
                    }).unwrap(),
                    GetClients { resp } => resp.send(clients.clone()).unwrap(),
                    AddClient { client } => { clients.insert(client.process_id(), Arc::new(Mutex::new(client))); },
                };
            }
        });

        let game = Game::new(game_tx.clone());
        thread::spawn(move || loop {
            // Get user input
            let mut command = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut command).unwrap();

            let chat = &ChatBuilder::new("Server:")
                .space()
                .append(ChatBuilder::new(&command.trim_end()).color("gray"))
                .finish();

            // Broadcast to each client
            for (_, client) in game.clients() {
                let client = client.lock().unwrap();
                client.send_chat(chat);
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

            handler::handle(socket_tx.clone(), Game::new(game_tx.clone()), decoded);
        };
    }
}

