use serde::de::SeqAccess;

use super::MinecraftDeserializer;
use crate::result::MinecraftError;

impl<'de> SeqAccess<'de> for MinecraftDeserializer {
    type Error = MinecraftError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match seed.deserialize(&mut *self).map(Some) {
            Ok(value) => Ok(value),
            Err(_) => {
                self.rewind()?;
                Ok(None)
            }
        }
    }
}
