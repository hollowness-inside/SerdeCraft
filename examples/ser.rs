use serdecraft::MinecraftSerializer;
use std::{net::TcpListener, thread};
use tungstenite::accept;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Serialize, Deserialize)]
struct UserProfile {
    email: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
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
            let mut serializer = MinecraftSerializer::new(websocket);
            Direction::East.serialize(&mut serializer).unwrap();
        });
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
        // 1i8.serialize(&mut serializer).unwrap();
        // (false).serialize(&mut serializer).unwrap();
        // (true).serialize(&mut serializer).unwrap();
        // (1i8).serialize(&mut serializer).unwrap();
        // (2u8).serialize(&mut serializer).unwrap();
        // (3i16).serialize(&mut serializer).unwrap();
        // (4u16).serialize(&mut serializer).unwrap();
        // (5i32).serialize(&mut serializer).unwrap();
        // (6u32).serialize(&mut serializer).unwrap();
        // (7i64).serialize(&mut serializer).unwrap();
        // (8u64).serialize(&mut serializer).unwrap();
        // (-1i8).serialize(&mut serializer).unwrap();
        // (-2i16).serialize(&mut serializer).unwrap();
        // (-3i32).serialize(&mut serializer).unwrap();
        // (-4i64).serialize(&mut serializer).unwrap();
        // (-5i64).serialize(&mut serializer).unwrap();
    }
}
