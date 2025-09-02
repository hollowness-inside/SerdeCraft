use serde::de::MapAccess;

use super::MinecraftDeserializer;
use crate::{MinecraftBlock, result::MinecraftError};

impl<'de> MapAccess<'de> for MinecraftDeserializer {
    type Error = MinecraftError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.peek()? {
            MinecraftBlock::GildedBlackstone => seed.deserialize(self).map(Some),
            MinecraftBlock::EmeraldBlock => Ok(None),
            _ => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}
