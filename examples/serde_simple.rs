#![feature(more_float_constants)]
#![feature(hash_map_macro)]

use serdecraft::{MinecraftDeserializer, MinecraftResult, MinecraftSerializer};
use std::net::TcpListener;
use tungstenite::accept;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
    color: Colors,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Colors {
    Red,
    Green,
    Blue,
}

fn main() -> MinecraftResult<()> {
    let v: Rgb = Rgb {
        r: 1,
        g: 12,
        b: 212,
        color: Colors::Blue,
    };
    let server = TcpListener::bind("127.0.0.1:8765")?;
    println!("Started");
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
                match <Rgb>::deserialize(&mut deserializer) {
                    Ok(x) => assert_eq!(v, x),
                    Err(e) => eprintln!("Failed to deserialize: {:?}", e),
                }
            }
            _ => eprintln!("Unknown command"),
        }
    }

    Ok(())
}
