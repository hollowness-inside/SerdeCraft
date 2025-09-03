use serde::de::MapAccess;

use super::MinecraftDeserializer;
use crate::{MinecraftBlock, result::MinecraftError};

pub(super) struct MCMapAccess<'a> {
    deserializer: &'a mut MinecraftDeserializer,
    terminator: MinecraftBlock,
    finished: bool,
}

impl<'a> MCMapAccess<'a> {
    pub fn new(deserializer: &'a mut MinecraftDeserializer, terminator: MinecraftBlock) -> Self {
        Self {
            deserializer,
            terminator,
            finished: false,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MCMapAccess<'a> {
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
