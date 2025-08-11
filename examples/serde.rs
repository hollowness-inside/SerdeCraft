use serdecraft::{MinecraftDeserializer, MinecraftResult, MinecraftSerializer};
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

fn main() -> MinecraftResult<()> {
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

    let server = TcpListener::bind("127.0.0.1:8765")?;
    for stream in server.incoming() {
        let mut websocket = accept(stream?).unwrap();
        println!("WebSocket connection established!");

        let message = match websocket.read() {
            Ok(msg) => msg,
            Err(err) => {
                eprintln!("Failed to read message: {:?}", err);
                continue;
            }
        };

        let text = match message.to_text() {
            Ok(text) => text,
            Err(_) => {
                eprintln!("Failed to convert message to text");
                continue;
            }
        };

        match text {
            "ser" => {
                let mut serializer = MinecraftSerializer::new(websocket);
                match v.serialize(&mut serializer) {
                    Ok(_) => println!("Serialized successfully"),
                    Err(e) => eprintln!("Failed to serialize: {:?}", e),
                };
            }
            "de" => {
                let mut deserializer = MinecraftDeserializer::new(websocket);
                match <UserWithProfile>::deserialize(&mut deserializer) {
                    Ok(x) => println!("Deserialized: {:#?}", x),
                    Err(e) => eprintln!("Failed to deserialize: {:?}", e),
                }
            }
            _ => eprintln!("Unknown command"),
        }
    }

    Ok(())
}
