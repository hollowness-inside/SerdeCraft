use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use torrento::BenValue;
use websocket::sync::Server;

use crate::ser::MinecraftSerializer;

mod de;
mod ser;

mod blocks;
mod result;

fn main() {
    let server = Server::bind("127.0.0.1:8765").expect("Failed to bind server");
    println!("Server is listening on ws://127.0.0.1:8765...");

    let s = std::fs::read("BigBuckBunny_124_archive.torrent").unwrap();
    let v: BenValue = torrento::de::from_bytes(&s).unwrap();

    for request in server.filter_map(Result::ok) {
        if let Ok(client) = request.accept() {
            let client = Arc::new(Mutex::new(client));

            let ip = client.lock().unwrap().peer_addr().unwrap();
            println!("Client connected: {}", ip);

            let mut ms = MinecraftSerializer::new(Arc::clone(&client));
            serde::Serialize::serialize(&v, &mut ms).unwrap();

            thread::sleep(Duration::from_secs(2));

            println!("Done with client {}. Closing connection.", ip);
        }
    }
}
