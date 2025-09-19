use serde::de::SeqAccess;

use super::MinecraftDeserializer;
use crate::{result::MinecraftError, websocket::MCWebSocket};

impl<'de, S: MCWebSocket> SeqAccess<'de> for MinecraftDeserializer<S> {
    type Error = MinecraftError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match seed.deserialize(&mut *self).map(Some) {
            Ok(value) => Ok(value),
            Err(_) => {
                self.socket.rewind_block()?;
                Ok(None)
            }
        }
    }
}
