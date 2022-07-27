mod handler;
mod structures;

use common::{ChatBuilder,Config};
use common::game::{GameCommand,Game};

use log::debug;
use env_logger::{Env,Builder};

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

fn main() {
    // Init logger
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env)
        .format_level(true)
        .format_indent(Some(4))
        .format_timestamp_millis()
        .init();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for socket in listener.incoming() {
        debug!("Proxy connected");
        let mut socket = socket.unwrap();
        let mut game_socket = socket.try_clone().unwrap();
        let (game_tx, game_rx) = mpsc::channel::<GameCommand>();
        thread::spawn(move || {
            let config: Config = toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
            let mut clients = HashMap::<String, structures::client::Client>::new();
            while let Ok(cmd) = game_rx.recv() {
                use GameCommand::*;
                match cmd {
                    SendPacket { packet } => {
                        game_socket.write(packet.as_slice()).unwrap();
                        game_socket.flush().unwrap();
                    },

                    GetConfig { resp } => resp.send(config.clone()).unwrap(),

                    HasClient { resp, process_id } => resp.send(clients.contains_key(&process_id)).unwrap(),
                    GetClientProperty { resp, process_id, property } => {
                        let client = clients.get(&process_id).unwrap();
                        resp.send(match property.as_str() {
                            "locale" => client.locale.clone(),
                            "username" => client.username.clone(),
                            _ => String::from(""),
                        }).unwrap();
                    },
                    GetClients { resp } => resp.send(clients.keys().cloned().collect()).unwrap(),
                    AddClient { process_id, version, locale, username } => { clients.insert(process_id, structures::client::Client {
                        version,
                        locale,
                        username,
                    }); },
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
            for client in game.clients() {
                client.send_message(chat);
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

            handler::handle(Game::new(game_tx.clone()), decoded);
        };
    }
}

