use serde::de::MapAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

pub(super) struct MCMapAccessor<'de> {
    deserializer: &'de mut MinecraftDeserializer,
}

impl<'de> MCMapAccessor<'de> {
    pub fn new(deserializer: &'de mut MinecraftDeserializer) -> Self {
        MCMapAccessor { deserializer }
    }
}

impl<'a, 'de> MapAccess<'de> for MCMapAccessor<'a> {
    type Error = MinecraftError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}
