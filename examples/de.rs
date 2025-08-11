use serde::{Deserialize, Serialize};
use serdecraft::MinecraftDeserializer;
use std::net::TcpListener;
use tungstenite::accept;

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
        let websocket = accept(stream.unwrap()).unwrap();
        println!("WebSocket connection established!");

        let mut deserializer = MinecraftDeserializer::new(websocket);
        match <UserWithProfile>::deserialize(&mut deserializer) {
            Ok(x) => println!("Received user: {:#?}", x),
            Err(e) => println!("Failed to deserialize user: {:#?}", e),
        }
    }
}
