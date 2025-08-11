use thiserror::Error;

pub type MinecraftResult<T> = Result<T, MinecraftError>;

#[derive(Debug, Error)]
pub enum MinecraftError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Custom: {0}")]
    Custom(String),

    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] Box<tungstenite::Error>),

    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Invalid Data: {0}")]
    InvalidData(String),

    #[error("Send error: {0}")]
    SendError(String),

    #[error("Receive error: {0}")]
    RecvError(String),

    #[error("Invalid Block Type: {0}")]
    InvalidBlockType(String),

    #[error("Char error: {0}")]
    Char(String),

    #[error("Char error")]
    CharChar,

    #[error("UTF-8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("Not Matching: {0}")]
    NotMatching(String),
}

impl From<tungstenite::Error> for MinecraftError {
    fn from(err: tungstenite::Error) -> Self {
        MinecraftError::Tungstenite(Box::new(err))
    }
}

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
