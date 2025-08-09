use serde::de::MapAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

pub struct MCStructAccessor<'de> {
    deserializer: &'de mut MinecraftDeserializer,
    remaining: usize,
}

impl<'de> MCStructAccessor<'de> {
    pub fn new(deserializer: &'de mut MinecraftDeserializer, len: usize) -> Self {
        MCStructAccessor {
            deserializer,
            remaining: len,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MCStructAccessor<'a> {
    type Error = MinecraftError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }

        self.remaining -= 1;
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}
