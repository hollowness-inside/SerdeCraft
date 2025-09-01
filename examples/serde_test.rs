#![feature(more_float_constants)]
#![feature(hash_map_macro)]

use serdecraft::{MinecraftDeserializer, MinecraftResult, MinecraftSerializer};
use std::{collections::HashMap, f32::consts::PI, f64::consts::PHI, net::TcpListener};
use tungstenite::accept;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UnitStruct;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum UnitVariants {
    A,
    B,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum NewtypeVariant {
    Byte(u8),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TupleVariant {
    Rgb(u8, u8, u8),
    Other(String, i64),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum StructVariant {
    Rgb { r: u8, g: u8, b: u8 },
    Other { name: String, value: i64 },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NewtypeStruct(f64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TupleStruct(i8, i8, u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SomeStruct {
    name: String,
    age: u8,
    health: f32,
    money: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestStructure {
    char_v: char,
    string: String,
    bytes: Vec<u8>,

    boolean: bool,
    i8_v: i8,
    i16_v: i16,
    i32_v: i32,
    i64_v: i64,

    u8_v: u8,
    u16_v: u16,
    u32_v: u32,
    u64_v: u64,

    f32_v: f32,
    f64_v: f64,

    none_value: Option<usize>,
    some_usize_value: Option<usize>,

    unit: (),
    unit_struct: UnitStruct,
    unit_variant: UnitVariants,

    newtype_struct: NewtypeStruct,
    // newtype_variant: NewtypeVariant,
    seq: Vec<f32>,
    tuple: (i8, u32, f64),
    tuple_struct: TupleStruct,

    // tuple_variant: TupleVariant,
    map: HashMap<String, i32>,
    struct_v: SomeStruct,
    // struct_variant: StructVariant,
}

fn main() -> MinecraftResult<()> {
    let v = TestStructure {
        boolean: false,
        i8_v: 1,
        i16_v: 2,
        i32_v: 3,
        i64_v: 4,
        u8_v: 5,
        u16_v: 6,
        u32_v: 7,
        u64_v: 8,
        f32_v: PI,
        f64_v: PHI,
        char_v: '😎',
        string: "Awesome".to_string(),
        bytes: b"Not Awesome".to_vec(),
        none_value: None,
        some_usize_value: Some(9),
        unit: (),
        unit_struct: UnitStruct,
        unit_variant: UnitVariants::A,
        newtype_struct: NewtypeStruct(123.456),
        seq: vec![1.0, 2.1, 3.2, 4.3],
        tuple: (11, 76, 3.0),
        tuple_struct: TupleStruct(4, 5, 63),
        map: hash_map! {
            "key".to_string() => 1,
            "value".to_string() => 2,
        },
        struct_v: SomeStruct {
            name: "John Doe".to_string(),
            age: 21,
            health: 76.32,
            money: 132421.2,
        },
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
                match <TestStructure>::deserialize(&mut deserializer) {
                    Ok(x) => assert_eq!(v, x),
                    Err(e) => eprintln!("Failed to deserialize: {:?}", e),
                }
            }
            _ => eprintln!("Unknown command"),
        }
    }

    Ok(())
}
