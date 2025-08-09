use serde::de::SeqAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

pub struct MCTupleAccessor<'a> {
    deserializer: &'a mut MinecraftDeserializer,
}

impl<'a> MCTupleAccessor<'a> {
    pub fn new(deserializer: &'a mut MinecraftDeserializer) -> Self {
        MCTupleAccessor { deserializer }
    }
}

impl<'a, 'de> SeqAccess<'de> for MCTupleAccessor<'a> {
    type Error = MinecraftError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer).map(Some)
    }
}
