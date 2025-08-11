use serde::de::SeqAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

pub(super) struct MCSeqAccessor<'a> {
    deserializer: &'a mut MinecraftDeserializer,
}

impl<'a> MCSeqAccessor<'a> {
    pub fn new(deserializer: &'a mut MinecraftDeserializer) -> Self {
        MCSeqAccessor { deserializer }
    }
}

impl<'a, 'de> SeqAccess<'de> for MCSeqAccessor<'a> {
    type Error = MinecraftError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match seed.deserialize(&mut *self.deserializer).map(Some) {
            Ok(value) => Ok(value),
            Err(_) => Ok(None),
        }
    }
}
