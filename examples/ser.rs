use serdecraft::MinecraftSerializer;
use std::net::TcpListener;
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
    let v = UserWithProfile {
        id: 12345,
        username: "TestUser".to_string(),
        direction: Direction::North,
        health: 100.0,
        profile: UserProfile {
            email: "testuser@example.com".to_string(),
            age: 30,
        },
    };

    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    for stream in server.incoming() {
        println!("WebSocket connection established!");

        let websocket = accept(stream.unwrap()).unwrap();
        let mut serializer = MinecraftSerializer::new(websocket);
        v.serialize(&mut serializer).unwrap();
    }
}

#[test]
fn test_main() {
    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    for stream in server.incoming() {
        let websocket = accept(stream.unwrap()).unwrap();
        let mut serializer = MinecraftSerializer::new(websocket);
        "strong consulatation recommended"
            .serialize(&mut serializer)
            .unwrap();
    }
}
