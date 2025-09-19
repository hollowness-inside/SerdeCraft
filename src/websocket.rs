use tungstenite::{Message, WebSocket};

use crate::{MinecraftBlock, MinecraftResult};

pub trait MCWebSocket {
    fn send_block(&mut self, block: MinecraftBlock) -> MinecraftResult<()>;
}

impl<S> MCWebSocket for WebSocket<S>
where
    S: std::io::Read + std::io::Write,
{
    fn send_block(&mut self, block: MinecraftBlock) -> MinecraftResult<()> {
        let message = Message::text(block.to_string());
        self.send(message)?;
        self.read()?;
        Ok(())
    }
}
