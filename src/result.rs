use thiserror::Error;

pub type MinecraftResult<T> = Result<T, MinecraftError>;

#[derive(Debug, Error)]
pub enum MinecraftError {
    // Network and I/O errors
    #[error("IO error occurred")]
    Io(#[from] std::io::Error),

    #[error("WebSocket error occurred")]
    WebSocket(#[from] Box<tungstenite::Error>),

    #[error("Failed to send message over WebSocket: {message}")]
    WebSocketSend {
        message: String,
        #[source]
        source: Box<tungstenite::Error>,
    },

    #[error("Failed to receive message from WebSocket: {message}")]
    WebSocketReceive {
        message: String,
        #[source]
        source: Box<tungstenite::Error>,
    },

    // Parsing and conversion errors
    #[error("Failed to parse integer")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid hexadecimal string: '{0}'")]
    InvalidHexString(String),

    #[error("Invalid number format for {0}-bit number")]
    InvalidNumberFormat(u8),

    // Block-related errors
    #[error("Unknown Minecraft block type: '{0}'")]
    UnknownBlockType(String),

    #[error("Block cannot be converted to digit: {0}")]
    BlockToDigitConversion(String),

    #[error("Invalid block sequence for {0}")]
    InvalidBlockSequence(String),

    #[error("Expected {expected} block, found {found}")]
    UnexpectedBlock { expected: String, found: String },

    // Serialization/Deserialization errors
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),

    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),

    #[error("Float serialization error: cannot serialize f64 value")]
    FloatSerializationError,

    #[error("Invalid wool sequence detected")]
    InvalidWoolSequence,

    #[error("Operation '{0}' failed")]
    OperationFailed(String),

    // Serde protocol errors
    #[error("Type mismatch: expected '{expected}', found '{found}'")]
    TypeMismatch { expected: String, found: String },

    #[error("Struct name mismatch: expected '{expected}', found '{found}'")]
    StructNameMismatch { expected: String, found: String },

    #[error("Invalid enum variant: '{0}'")]
    InvalidEnumVariant(String),

    #[error("Missing required field: '{0}'")]
    MissingField(String),

    // Protocol-specific errors
    #[error("Rewind operation failed")]
    RewindFailed,

    #[error("Peek operation failed")]
    PeekFailed,

    #[error("Consume operation failed")]
    ConsumeFailed,

    #[error("Invalid protocol state")]
    InvalidProtocolState,

    // Generic error for backward compatibility
    #[error("{0}")]
    Custom(String),
}

impl From<tungstenite::Error> for MinecraftError {
    fn from(err: tungstenite::Error) -> Self {
        MinecraftError::WebSocket(Box::new(err))
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
