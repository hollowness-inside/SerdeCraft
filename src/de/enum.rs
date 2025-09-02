use serde::de::EnumAccess;

use super::{MCVariantAccessor, MinecraftDeserializer};
use crate::MinecraftError;

pub(super) struct MCEnumAccessor<'de> {
    deserializer: &'de mut MinecraftDeserializer,
}

impl<'de> MCEnumAccessor<'de> {
    pub fn new(deserializer: &'de mut MinecraftDeserializer) -> Self {
        MCEnumAccessor { deserializer }
    }
}

impl<'a, 'de> EnumAccess<'de> for MCEnumAccessor<'a> {
    type Error = MinecraftError;
    type Variant = MCVariantAccessor<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.deserializer)?;
        Ok((val, MCVariantAccessor::new(self.deserializer)))
    }
}
