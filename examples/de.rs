use serdecraft::MinecraftDeserializer;
use std::{net::TcpListener, thread};
use tungstenite::accept;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserProfile {
    email: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserWithProfile {
    id: u32,
    username: String,
    direction: Direction,
    health: f64,
    profile: UserProfile,
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    for stream in server.incoming() {
        thread::spawn(move || {
            let websocket = accept(stream.unwrap()).unwrap();
            println!("WebSocket connection established!");

            let mut deserializer = MinecraftDeserializer::new(websocket);
            let x: Direction = Direction::deserialize(&mut deserializer).unwrap();
            println!("Received user: {:#?}", x);
        });
    }
}
