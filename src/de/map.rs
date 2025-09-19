use serde::de::MapAccess;

use super::MinecraftDeserializer;
use crate::{MinecraftBlock, result::MinecraftError, websocket::MCWebSocket};

pub(super) struct MCMapAccess<'a, S: MCWebSocket> {
    deserializer: &'a mut MinecraftDeserializer<S>,
    terminator: MinecraftBlock,
    finished: bool,
}

impl<'a, S: MCWebSocket> MCMapAccess<'a, S> {
    pub fn new(deserializer: &'a mut MinecraftDeserializer<S>, terminator: MinecraftBlock) -> Self {
        Self {
            deserializer,
            terminator,
            finished: false,
        }
    }
}

impl<'a, 'de, S: MCWebSocket> MapAccess<'de> for MCMapAccess<'a, S> {
    type Error = MinecraftError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.finished {
            return Ok(None);
        }

        let next_block = self.deserializer.peek()?;
        if next_block == self.terminator {
            // We've reached the end of the map, consume the terminator
            self.deserializer.consume()?;
            self.finished = true;
            return Ok(None);
        }

        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}
