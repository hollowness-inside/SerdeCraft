use serdecraft::{MinecraftDeserializer, MinecraftSerializer};
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
    let mut state = true;
    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    for stream in server.incoming() {
        let websocket = accept(stream.unwrap()).unwrap();
        println!("WebSocket connection established!");

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

        if state {
            let mut serializer = MinecraftSerializer::new(websocket);
            let _ = v.serialize(&mut serializer).unwrap();
        } else {
            let mut deserializer = MinecraftDeserializer::new(websocket);
            match <UserProfile>::deserialize(&mut deserializer) {
                Ok(x) => println!("Received user: {:#?}", x),
                Err(e) => println!("Failed to deserialize user: {:#?}", e),
            }
        }

        state = !state;
    }
}

#[test]
fn t() {
    println!(
        "{:?}",
        12i32.to_le_bytes().map(|x| format!("{:02x}", x)).join("")
    );
}
