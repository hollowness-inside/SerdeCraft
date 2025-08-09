use std::{net::TcpStream, num::ParseIntError, string::FromUtf8Error};

use tungstenite::{HandshakeError, ServerHandshake, handshake::server::NoCallback};

pub type MinecraftResult<T> = Result<T, MinecraftError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinecraftError {
    Custom(String),
    InvalidData(String),
    SendError(String),
    RecvError(String),
    InvalidBlockType(String),
    Char,
    NotMatching(String),
}

impl std::fmt::Display for MinecraftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Minecraft serialization error")
    }
}

impl serde::ser::StdError for MinecraftError {}

impl serde::ser::Error for MinecraftError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        eprintln!("Serialization error: {}", msg);
        MinecraftError::Custom(msg.to_string())
    }
}

impl serde::de::Error for MinecraftError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        eprintln!("Deserialization error: {}", msg);
        MinecraftError::Custom(msg.to_string())
    }
}

impl From<tungstenite::Error> for MinecraftError {
    fn from(err: tungstenite::Error) -> Self {
        eprintln!("WebSocket error: {}", err);
        MinecraftError::Custom(err.to_string())
    }
}

impl From<std::io::Error> for MinecraftError {
    fn from(err: std::io::Error) -> Self {
        eprintln!("IO error: {}", err);
        MinecraftError::Custom(err.to_string())
    }
}

impl From<HandshakeError<ServerHandshake<TcpStream, NoCallback>>> for MinecraftError {
    fn from(err: HandshakeError<ServerHandshake<TcpStream, NoCallback>>) -> Self {
        eprintln!("Handshake error: {}", err);
        MinecraftError::Custom(err.to_string())
    }
}

impl From<FromUtf8Error> for MinecraftError {
    fn from(err: FromUtf8Error) -> Self {
        eprintln!("UTF-8 conversion error: {}", err);
        MinecraftError::Custom(err.to_string())
    }
}

impl From<ParseIntError> for MinecraftError {
    fn from(err: ParseIntError) -> Self {
        eprintln!("ParseInt error: {}", err);
        MinecraftError::Custom(err.to_string())
    }
}
