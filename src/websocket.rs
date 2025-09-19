use tungstenite::{Message, WebSocket};

use crate::{MinecraftBlock, MinecraftError, MinecraftResult};

pub trait MCWebSocket {
    fn send_block(&mut self, block: MinecraftBlock) -> MinecraftResult<()>;
    fn consume_block(&mut self) -> MinecraftResult<MinecraftBlock>;
    fn skip_block(&mut self) -> MinecraftResult<()>;
    fn rewind_block(&mut self) -> MinecraftResult<()>;
    fn flush(&mut self) -> MinecraftResult<()> {
        Ok(())
    }
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

    fn consume_block(&mut self) -> MinecraftResult<MinecraftBlock> {
        self.send(Message::text("consume"))?;
        self.flush()?;

        let response = self.read()?;
        let text = response.to_text()?;
        text.try_into()
    }

    fn skip_block(&mut self) -> MinecraftResult<()> {
        self.write(Message::text("skip"))?;
        self.flush()?;
        Ok(())
    }

    fn rewind_block(&mut self) -> MinecraftResult<()> {
        self.write(tungstenite::Message::Text("rewind".into()))?;
        self.flush()?;

        let response = self.read()?;
        let text = response.to_text()?;
        match text == "done" {
            true => Ok(()),
            false => Err(MinecraftError::RewindFailed),
        }
    }
}
