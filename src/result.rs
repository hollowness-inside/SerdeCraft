pub type MinecraftResult<T> = Result<T, MinecraftError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinecraftError {
    Custom(String),
    InvalidData(String),
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
